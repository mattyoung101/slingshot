// Slingshot: A SystemVerilog language server.
//
// Copyright (c) 2025-2026 M. L. Young.
//
// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0. If a copy of the MPL
// was not distributed with this file, You can obtain one at https://mozilla.org/MPL/2.0/.
#include "slingshot/handlers.hpp"
#include "slingshot/conversions.hpp"
#include "slingshot/slingshot.hpp"
#include <exception>
#include <filesystem>
#include <lsp/messages.h>
#include <lsp/types.h>
#include <optional>
#include <regex>
#include <slang/diagnostics/Diagnostics.h>
#include <slang/syntax/SyntaxKind.h>
#include <slang/syntax/SyntaxNode.h>
#include <slang/syntax/SyntaxVisitor.h>
#include <slang/text/SourceLocation.h>
#include <slang/util/Enum.h>
#include <spdlog/spdlog.h>
#include <string>
#include <toml++/toml.hpp>
#include <vector>

namespace {
// https://stackoverflow.com/a/72900791
const std::regex SEMVER_REGEX(
    R"(^(0|[1-9]\d*)\.(0|[1-9]\d*)\.(0|[1-9]\d*)(?:-((?:0|[1-9]\d*|\d*[a-zA-Z-][0-9a-zA-Z-]*)(?:\.(?:0|[1-9]\d*|\d*[a-zA-Z-][0-9a-zA-Z-]*))*))?(?:\+([0-9a-zA-Z-]+(?:\.[0-9a-zA-Z-]+)*))?$)");
}; // namespace

namespace {
void parseConfigToml(std::filesystem::path &path) {
    try {
        auto config = toml::parse_file(path.string());

        if (!config.contains("version")) {
            SPDLOG_ERROR("Configuration is missing version");
            return;
        }

        auto *version = config["version"].as_string();

        std::smatch matches;
        std::string major, minor, patch;
        // wow, what a great language, we love C++, thank you C++, incredible language, great work
        // fucking awful piecea shit
        // https://stackoverflow.com/a/32164608
        std::string temporary = std::string(*version);
        if (!std::regex_match(temporary, matches, SEMVER_REGEX)) {
            SPDLOG_ERROR("Failed to match config version");
        }

        // also note that this godforsaken language has a slower regex engine than ruby and python 2 COMBINED
        // see: https://github.com/mariomka/regex-benchmark
        // anyway, the reason we're using this is because, while std::regex may be an abominably slow and
        // shittily designed API, it *is* at least included with the terrible language itself. re2 is good but
        // Google in their infinite wisdom made it completely unportable by depending on an unspecified (!)
        // version of absl, which will break our CI and break our users trying to install this project. as I
        // have written about extensively in the README, because this language is so fucking awful, I
        // absolutely refuse to depend on system deps because that guarantees we will be uncompilable on some
        // random platform in the future, so we'll just settle to use std::regex instead. after all, we only
        // have to do this at boot once.

        major = matches[1];
        minor = matches[2];
        patch = matches[3];

        if (major != slingshot::CONFIG_MAJOR_VERSION) {
            SPDLOG_ERROR("Config major version mismatch. Project uses {} (major: {}), but server uses {}.x.x",
                std::string(*version), major, slingshot::CONFIG_MAJOR_VERSION);
            SPDLOG_DEBUG("i.e. {} != {}", major, slingshot::CONFIG_MAJOR_VERSION);
            for (const auto &match : matches) {
                SPDLOG_DEBUG("match: {}", match.str());
            }
            return;
        }

        SPDLOG_INFO("Parsed config version: v{}.{}.{}", major, minor, patch);

        bool didFindFileSources = false;

        // begin initial indexing
        slingshot::g_indexManager.beginInitialIndexing();

        if (config.contains("include_dirs")) {
            SPDLOG_DEBUG("Config has include_dirs");

            auto *include_dirs = config["include_dirs"].as_array();

            std::vector<std::string> dirs;
            for (const auto &dir : *include_dirs) {
                auto str = std::string(*dir.as_string());
                slingshot::g_compilerManager.addIncludeDir(str);
                dirs.push_back(str);
            }
            // okay to assign here, this won't conflict with flist_files since we do that later
            slingshot::g_indexManager.includeDirs = dirs;

            // **now** that we've registered the include dirs, we can actually walk the directories
            for (const auto &dir : dirs) {
                slingshot::g_indexManager.walkDir(dir);
            }

            didFindFileSources = true;
        }

        if (config.contains("flist_files")) {
            SPDLOG_DEBUG("Config has flist_files");

            auto *flist_files = config["flist_files"].as_array();
            for (const auto &file : *flist_files) {
                slingshot::g_indexManager.parseFListFile(std::string(*file.as_string()));
            }
            didFindFileSources = true;
        }

        if (!didFindFileSources) {
            SPDLOG_ERROR("Config file defines no file sources. At least one of 'include_dirs' or "
                         "'flist_files' should be present.");
        }

        if (config.contains("defines")) {
            auto *defines = config["defines"].as_table();
            for (const auto &[key, value] : *defines) {
                auto keystr = std::string(key);
                auto valuestr = std::string(*value.as_string());
                SPDLOG_INFO("Add predefined macro: {}={}", keystr, valuestr);
                slingshot::g_compilerManager.addPreDefinedMacro(keystr, valuestr);
            }
        }

        // finished queueing all index jobs
        slingshot::g_indexManager.isStillQueueingIndexJobs = false;
    } catch (const std::exception &e) {
        SPDLOG_ERROR("Failed to parse config TOML: {}", e.what());
    }
}

}; // namespace

namespace slingshot::handlers {

lsp::requests::Initialize::Result initialise(const lsp::requests::Initialize::Params &&params) {
    SPDLOG_INFO("Received init");

    if (params.rootUri.isNull()) {
        SPDLOG_ERROR("No root URI path specified!");
    } else {
        auto root = params.rootUri->path();
        SPDLOG_INFO("Attempting to locate .slingshot.toml file in {}", root);

        auto tomlFile = std::filesystem::path(std::string(root) + "/.slingshot.toml");

        if (std::filesystem::exists(tomlFile)) {
            // parse it
            parseConfigToml(tomlFile);
        } else {
            SPDLOG_ERROR("Could not locate .slingshot.toml file. Index may be non-functional!");
            SPDLOG_ERROR("Tried: {}", tomlFile.string());
            // TODO warn client
        }
    }

    g_compilerManager.boot();

    return lsp::requests::Initialize::Result{
				.capabilities = {
					.positionEncoding = lsp::PositionEncodingKind::UTF8,
					.textDocumentSync = lsp::TextDocumentSyncOptions {
						.openClose = true,
						// this should probably be incremental in future if we actually see any performance
						// problems but W/E for now
						.change    = lsp::TextDocumentSyncKind::Full,
						.save      = true,
					},
					.completionProvider = lsp::CompletionOptions {
                        .triggerCharacters = std::vector<std::string>{".", "`", "[", "{"},
					},
                    .definitionProvider = true
				},
				.serverInfo = lsp::InitializeResultServerInfo{
					.name    = "Slingshot",
					.version = SLINGSHOT_VERSION,
				},
			};
}

void exit() {
    SPDLOG_INFO("Shutting down (exit)");
    slingshot::g_running = false;
}

lsp::requests::Shutdown::Result shutdown() {
    SPDLOG_INFO("Shutting down (shutdown)");
    slingshot::g_running = false;
    return lsp::requests::Shutdown::Result { };
}

void textDocumentOpen(const lsp::notifications::TextDocument_DidOpen::Params &&params) {
    SPDLOG_DEBUG("Open document: {}", params.textDocument.uri.path());

    // register in the document database
    g_indexManager.insert(
        std::filesystem::absolute(params.textDocument.uri.path()), params.textDocument.text, false);
    auto lock = g_compilerManager.acquireLock();
    g_compilerManager.openFiles.insert(std::filesystem::absolute(params.textDocument.uri.path()));
}

void textDocumentClose(const lsp::notifications::TextDocument_DidClose::Params &&params) {
    SPDLOG_DEBUG("Close document: {}", params.textDocument.uri.path());
    auto lock = g_compilerManager.acquireLock();
    g_compilerManager.openFiles.erase(std::filesystem::absolute(params.textDocument.uri.path()));
}

void textDocumentChange(const lsp::notifications::TextDocument_DidChange::Params &&params) {
    for (const lsp::TextDocumentContentChangeEvent &change : params.contentChanges) {
        std::visit(
            [&](auto &&arg) {
                using T = std::decay_t<decltype(arg)>;
                if constexpr (std::is_same_v<T, lsp::TextDocumentContentChangeEvent_Range_Text>) {
                    // we specifically told the client to send us full text, so if they're sending this, get
                    // mad at them
                    SPDLOG_ERROR("We don't yet handle Event_Range_Text. Your LSP client is borked.");
                }
                if constexpr (std::is_same_v<T, lsp::TextDocumentContentChangeEvent_Text>) {
                    const lsp::TextDocumentContentChangeEvent_Text &event = arg;
                    SPDLOG_TRACE("===== UPDATE {} =====\n{}\n", params.textDocument.uri.path(), event.text);
                    g_indexManager.insert(
                        std::filesystem::absolute(params.textDocument.uri.path()), event.text, false);
                }
            },
            change);
    }
}

void textDocumentSave(const lsp::notifications::TextDocument_DidSave::Params &&params) {
    SPDLOG_TRACE("Did save document: {}", params.textDocument.uri.path());

    if (params.text.has_value()) {
        // register in the document database
        g_indexManager.insert(std::filesystem::absolute(params.textDocument.uri.path()), *params.text, false);
    }
}

lsp::requests::TextDocument_Completion::Result textDocumentCompletion(
    const lsp::requests::TextDocument_Completion::Params &&params) {
    auto path = std::filesystem::absolute(params.textDocument.uri.path()).string();
    SPDLOG_TRACE("Completion request in {}", path);

    auto lock = g_indexManager.acquireLock();
    auto result = g_indexManager.retrieve(path);
    if (!result.has_value()) {
        SPDLOG_WARN("Document {} is not in index", path);
        return { };
    }
    if ((*result)->tree == nullptr) {
        SPDLOG_WARN("Document {} has no parse tree at all, can't do completion", path);
        return { };
    }

    // ensure the index entry is valid
    lock.unlock();
    (*result)->ensureValidByWaiting();

    return CompletionManager::getCompletions(path, params.position, *result);
}

lsp::requests::TextDocument_Definition::Result textDocumentDefinition(
    const lsp::requests::TextDocument_Definition::Params &&params) {
    auto path = std::filesystem::absolute(params.textDocument.uri.path()).string();
    SPDLOG_INFO("GO TO DEFINITION request in {}", path);

    auto indexLock = g_indexManager.acquireLock();
    auto compilerLock = g_compilerManager.acquireLock();

    auto result = g_indexManager.retrieve(path);
    if (!result.has_value()) {
        SPDLOG_WARN("Document {} is not in index", path);
        return lsp::Null();
    }
    if (!(*result)->doc.has_value() || (*result)->tree == nullptr) {
        SPDLOG_WARN("Document {} has no lifted lang doc associated with it or no CST", path);
        return lsp::Null();
    }

    auto loc = toSlangLocation(
        params.position, params.textDocument.uri.path(), g_compilerManager.getSourceManager());

    // resolve the symbol under the cursor; find the deepest nested syntax node from the tree root
    // basically DFS
    std::string target;
    AllSyntaxVisitor visitor([&loc, &target](const SyntaxNode &node) {
        if (node.sourceRange().contains(loc)) {
            SPDLOG_TRACE("Contains: {}", node.toString());
            target = node.toString();
        }
    });
    visitor.visit((*result)->tree->root());
    trim(target);

    SPDLOG_INFO("Target node lookup: '{}'", target);

    auto doc = *(*result)->doc;

    // first, attempt to resolve the symbol within the file
    for (const auto &module : doc.modules) {
        auto lookup = module.querySymbolLocation(target);
        SPDLOG_INFO("Found lookup target! {} in {}", lookup->name, lookup->path.string());
        if (lookup.has_value()) {
            return lsp::Location {
                // should be in the same file? look it up anyway
                .uri = lsp::Uri::parse("file://" + lookup->path.string()),
                .range = lsp::Range { .start = lookup->pos, .end = lookup->pos }
            };
        }
    }

    SPDLOG_WARN("Unable to resolve go to definition on symbol '{}'", target);

    return lsp::Null();
}

} // namespace slingshot::handlers
