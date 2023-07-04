/*
 * Copyright (c) 2023 Matt Young.
 *
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */
use log::{debug, info};
use simple_logger::SimpleLogger;
use slingshot::completion::CompletionProvider;
use slingshot::completion::SvParserCompletion;
use slingshot::diagnostics::DiagnosticProvider;
use slingshot::diagnostics::VerilatorDiagnostics;
use slingshot::indexing::IndexManager;
use tower_lsp::jsonrpc::Result;
use tower_lsp::lsp_types::CompletionOptions;
use tower_lsp::lsp_types::InitializeParams;
use tower_lsp::lsp_types::InitializeResult;
use tower_lsp::lsp_types::InitializedParams;
use tower_lsp::lsp_types::MessageType;
use tower_lsp::lsp_types::OneOf;
use tower_lsp::lsp_types::ServerCapabilities;
use tower_lsp::lsp_types::ServerInfo;
use tower_lsp::lsp_types::TextDocumentSyncCapability;
use tower_lsp::lsp_types::TextDocumentSyncKind;
use tower_lsp::lsp_types::WorkspaceFoldersServerCapabilities;
use tower_lsp::lsp_types::WorkspaceServerCapabilities;
use tower_lsp::Client;
use tower_lsp::LanguageServer;

struct Backend {
    client: Client,
    index: IndexManager,
}

// Reference: https://stackoverflow.com/a/27841363/5007892
const VERSION: &str = env!("CARGO_PKG_VERSION");

// Reference: https://github.com/IWANABETHATGUY/tower-lsp-boilerplate/blob/main/src/main.rs
#[tower_lsp::async_trait]
impl LanguageServer for Backend {
    async fn initialize(&self, _: InitializeParams) -> Result<InitializeResult> {
        debug!("Initialize LSP");

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
        // notify client that LSP has initialised successfully
        self.client
            .log_message(MessageType::INFO, "Slingshot init OK")
            .await;
    }

    async fn shutdown(&self) -> Result<()> {
        debug!("Shutdown LSP");
        Ok(())
    }
}

#[tokio::main]
async fn main() {
    // initialise logging framework
    // TODO we will probably need to use a file-based logging service since we'll be using an stdio
    // based LSP, so logging to stdio will interfere with that
    SimpleLogger::new().init().unwrap();
    color_backtrace::install();

    info!("Slingshot v{} - Copyright (c) 2023 Matt Young.", VERSION);
    info!("Licenced under the Mozilla Public License v2.0.");

    //let document = "module x; logic [15:0] y;\nendmodule\n";

    let document = r#"
    typedef enum {
        ENUM_A,
        ENUM_B
    } Test_t;

module mymodule(
    input logic [15:0] a,
    input logic [15:0] b,
    output logic[31:0] c
);
    logic [15:0] something;

    always_comb begin
        c = a + b + something;
    end
endmodule;

    "#;

    let result = VerilatorDiagnostics::diagnose(document).unwrap();
    for entry in result.iter() {
        info!("{:?}", entry);
    }

    let result2 = SvParserCompletion::extract_tokens(document).unwrap();
    let module = &result2.modules[0];
    for port in module.ports.as_slice() {
        info!("port: {:?}", port);
    }
    for var in module.variables.as_slice() {
        info!("variable: {:?}", var);
    }
}
