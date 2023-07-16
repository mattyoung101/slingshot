use std::sync::{Arc, Mutex};

use fern::colors::{ColoredLevelConfig, Color};
use log::{warn, error};
/*
 * Copyright (c) 2023 Matt Young.
 *
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */
use log::{debug, info};
use slingshot::completion::CompletionProvider;
use slingshot::completion::SvParserCompletion;
use slingshot::diagnostics::DiagnosticProvider;
use slingshot::diagnostics::VerilatorDiagnostics;
use slingshot::indexing::IndexManager;
use tower_lsp::lsp_types::DidOpenTextDocumentParams;
use tower_lsp::lsp_types::InitializeParams;
use tower_lsp::lsp_types::InitializeResult;
use tower_lsp::lsp_types::InitializedParams;
use tower_lsp::lsp_types::MessageType;
use tower_lsp::lsp_types::OneOf;
use tower_lsp::lsp_types::ServerCapabilities;
use tower_lsp::lsp_types::ServerInfo;
use tower_lsp::lsp_types::TextDocumentItem;
use tower_lsp::lsp_types::TextDocumentSyncCapability;
use tower_lsp::lsp_types::TextDocumentSyncKind;
use tower_lsp::lsp_types::WorkspaceFoldersServerCapabilities;
use tower_lsp::lsp_types::WorkspaceServerCapabilities;
use tower_lsp::lsp_types::{CompletionOptions, DidChangeTextDocumentParams};
use tower_lsp::Client;
use tower_lsp::LanguageServer;
use tower_lsp::{jsonrpc, LspService, Server};

// Reference: https://stackoverflow.com/a/27841363/5007892
const VERSION: &str = env!("CARGO_PKG_VERSION");

struct Backend {
    client: Client,
    index: Mutex<Option<IndexManager>>,
}

impl Backend {
    async fn on_change(&self, params: TextDocumentItem) {
        debug!(
            "Text document changed. Path: {}, version: {}",
            params.uri, params.version
        );

        let path = match params.uri.to_file_path() {
            Ok(p) => p,
            Err(_) => {
                warn!("Error converting URI to file path: {:?}", params.uri);
                return;
            }
        };

        // run and publish diagnostics
        match VerilatorDiagnostics::diagnose(&path, &params.text).await {
            Ok(diags) => {
                self.client
                    .publish_diagnostics(params.uri, diags, Some(params.version))
                    .await
            }
            Err(e) => {
                warn!("VerilatorDiagnostics failed: {:?}", e)
            }
        };

        // run completion, only if the index has been created
        // mutex information: https://stackoverflow.com/a/62249079/5007892
        let mut guard = self.index.lock().unwrap();
        match &mut *guard {
            Some(index) => {
                // generate completion tokens
                match SvParserCompletion::extract_tokens(&params.text) {
                    Ok(completion) => {
                        // insert completion into index
                        index.insert(&path, &params.text, &completion);
                        
                        match index.maybe_flush() {
                            Ok(_) => {}
                            Err(e) => { error!("Failed to flush index: {:#?}", e); }
                        }
                        
                        // TODO generate completion tokens based on index and current cursor pos
                    }
                    Err(e) => {
                        warn!("Generating completion failed: {:?}", e);
                    }
                }
            }
            None => {
                debug!("Index does not exist yet, cannot run completion");
            }
        }
    }
}

// Reference: https://github.com/IWANABETHATGUY/tower-lsp-boilerplate/blob/main/src/main.rs
#[tower_lsp::async_trait]
impl LanguageServer for Backend {
    async fn initialize(&self, params: InitializeParams) -> jsonrpc::Result<InitializeResult> {
        debug!("Initialising LSP for {:?}", params.client_info);

        match params.workspace_folders {
            Some(folder) => {
                let root_dir = folder[0].uri.to_file_path().unwrap();
                let cache = root_dir.join(".cache/slingshot/slingshot_cache.dat");
                debug!("Going to initialise index with root path: {:?}, cache: {:?}", root_dir, cache);
                
                let mut guard = self.index.lock().unwrap();
                *guard = Some(IndexManager::new(&cache));
            }
            None => {}
        }

        return Ok(InitializeResult {
            server_info: Some(ServerInfo {
                name: "Slingshot".to_string(),
                version: Some(VERSION.to_string()),
            }),
            capabilities: ServerCapabilities {
                text_document_sync: Some(TextDocumentSyncCapability::Kind(
                    TextDocumentSyncKind::FULL,
                )),
                completion_provider: Some(CompletionOptions {
                    // this should allow us to get completion when the user is doing module
                    // instantiation
                    trigger_characters: Some(vec![".".to_string()]),
                    resolve_provider: Some(false),
                    all_commit_characters: None,
                    work_done_progress_options: Default::default(),
                    completion_item: None,
                }),
                execute_command_provider: None,
                // neovim doesn't really have the concept of workspaces like vscode does, but we
                // might be able to use this information to generate better indexes or to know
                // where to store our index file
                workspace: Some(WorkspaceServerCapabilities {
                    workspace_folders: Some(WorkspaceFoldersServerCapabilities {
                        supported: Some(true),
                        change_notifications: Some(OneOf::Left(true)),
                    }),
                    file_operations: None,
                }),

                ..ServerCapabilities::default()
            },
        });
    }

    async fn initialized(&self, _: InitializedParams) {
        self.client
            .log_message(MessageType::INFO, "Slingshot init OK")
            .await;
    }

    async fn did_open(&self, params: DidOpenTextDocumentParams) {
        self.client
            .log_message(MessageType::INFO, "Document opened")
            .await;
        self.on_change(TextDocumentItem {
            uri: params.text_document.uri,
            text: params.text_document.text,
            version: params.text_document.version,
            language_id: "verilog".to_string(),
        })
        .await;
    }

    async fn did_change(&self, mut params: DidChangeTextDocumentParams) {
        self.on_change(TextDocumentItem {
            uri: params.text_document.uri,
            // FIXME is this really good practice?
            text: std::mem::take(&mut params.content_changes[0].text),
            version: params.text_document.version,
            language_id: "verilog".to_string(),
        })
        .await;
    }

    async fn shutdown(&self) -> jsonrpc::Result<()> {
        debug!("Shutdown LSP");

        let mut guard = self.index.lock().unwrap();
        match &mut *guard {
            Some(index) => {
                let _ = index.flush();
            }
            None => {}
        }
        
        Ok(())
    }
}

#[tokio::main]
async fn main() {
    // initialise logging framework
    color_backtrace::install();
    log_panics::init();
    // hack to force backtrace: https://stackoverflow.com/a/71731489/5007892
    std::env::set_var("RUST_BACKTRACE", "1");
    let colours_line = ColoredLevelConfig::new()
        .error(Color::Red)
        .warn(Color::Yellow)
        // we actually don't need to specify the color for debug and info, they are white by default
        .info(Color::Green)
        .debug(Color::White)
        // depending on the terminals color scheme, this is the same as the background color
        .trace(Color::BrightBlack);
    // we actually overwrite the file for ease of debugging
    let logfile = std::fs::OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open("/tmp/slingshot.log");
    fern::Dispatch::new()
        .format(move |out, message, record| {
            out.finish(format_args!(
                "{colour_line}[{} {} {}] {}\x1B[0m",
                // TODO: format time as local time not UTC
                humantime::format_rfc3339(std::time::SystemTime::now()),
                record.level(),
                record.target(),
                message,
                colour_line = format_args!(
                    "\x1B[{}m",
                    colours_line.get_color(&record.level()).to_fg_str()
                ),
            ))
        })
        .level(log::LevelFilter::Debug)
        .chain(std::io::stderr())
        .chain(logfile.unwrap())
        .apply()
        .unwrap();

    info!(
        "Slingshot v{} - Copyright (c) 2023 Matt Young. Mozilla Public License v2.0.",
        VERSION
    );

    let stdin = tokio::io::stdin();
    let stdout = tokio::io::stdout();

    debug!("Building LspService");
    let (service, socket) = LspService::build(|client| Backend {
        client,
        // index is initialised later
        index: Mutex::new(None),
    })
    .finish();

    debug!("Booting server");
    Server::new(stdin, stdout, socket).serve(service).await;
}
