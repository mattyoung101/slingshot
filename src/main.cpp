// Slingshot: A SystemVerilog language server.
//
// Copyright (c) 2025-2026 M. L. Young.
//
// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0. If a copy of the MPL
// was not distributed with this file, You can obtain one at https://mozilla.org/MPL/2.0/.
#include "lsp/connection.h"
#include "lsp/io/standardio.h"
#include "lsp/messagehandler.h"
#include "lsp/messages.h"
#include "lsp/types.h"
#include "slang/util/VersionInfo.h"
#include "slingshot/compiler.hpp"
#include "slingshot/handlers.hpp"
#include "slingshot/slingshot.hpp"
#include "spdlog/sinks/basic_file_sink.h"
#include "spdlog/sinks/rotating_file_sink.h"
#include "spdlog/spdlog.h"
#include <cstdlib>
#include <exception>
#include <memory>
#include <spdlog/common.h>
#include <spdlog/sinks/ansicolor_sink.h>
#include <unistd.h>

namespace {

void addCallbacks(std::shared_ptr<lsp::MessageHandler> &msgHandler) {
    msgHandler->add<lsp::requests::Initialize>(slingshot::handlers::initialise);
    msgHandler->add<lsp::requests::Shutdown>(slingshot::handlers::shutdown);
    msgHandler->add<lsp::notifications::Exit>(slingshot::handlers::exit);
    msgHandler->add<lsp::notifications::TextDocument_DidOpen>(slingshot::handlers::textDocumentOpen);
    msgHandler->add<lsp::notifications::TextDocument_DidChange>(slingshot::handlers::textDocumentChange);
    msgHandler->add<lsp::requests::TextDocument_Completion>(slingshot::handlers::textDocumentCompletion);
    msgHandler->add<lsp::notifications::TextDocument_DidClose>(slingshot::handlers::textDocumentClose);
    msgHandler->add<lsp::requests::TextDocument_Definition>(slingshot::handlers::textDocumentDefinition);
}

} // namespace

namespace slingshot {
bool g_running = false;
IndexManager g_indexManager = { };
CompilationManager g_compilerManager = { };
RemoteDebugger g_debugger = { };
std::shared_ptr<lsp::MessageHandler> g_msgHandler = { };
CompletionManager g_completionManager = { };

void exit_handler() {
    SPDLOG_DEBUG("Exit handler!");
    g_compilerManager.shutdown();
    g_debugger.shutdown();

    SPDLOG_INFO("Goodbye!");

    for (const auto &sink : spdlog::default_logger()->sinks()) {
        sink->flush();
    }

    std::this_thread::sleep_for(500ms);
}

} // namespace slingshot

int main() {
    using namespace slang;
    using namespace slingshot;

    // info by default
    auto level = spdlog::level::info;
    if (std::getenv("SLING_LOG_DEBUG") != nullptr) {
        level = spdlog::level::debug;
    }
    if (std::getenv("SLING_LOG_TRACE") != nullptr) {
        level = spdlog::level::trace;
    }

    spdlog::set_level(level);
    spdlog::flush_on(level);
    atexit(slingshot::exit_handler);

    // keep stderr free for the LSP
    auto stderr_sink = std::make_shared<spdlog::sinks::ansicolor_stderr_sink_mt>();
    stderr_sink->set_level(level);
    spdlog::default_logger()->sinks().clear();
    spdlog::default_logger()->sinks().push_back(stderr_sink);

    spdlog::default_logger()->set_pattern("[%Y-%m-%d %H:%M:%S.%e thread %t] [%^%l%$] [%s:%# %!] %v");
    for (auto &sink : spdlog::default_logger()->sinks()) {
        sink->set_pattern("[%Y-%m-%d %H:%M:%S.%e thread %t] [%^%l%$] [%s:%# %!] %v");
    }

    SPDLOG_INFO(
        "Slingshot LSP v{} - (c) 2023-2026 Mel Young. Licenced under the MPL 2.0.", SLINGSHOT_VERSION);
    SPDLOG_INFO("Slang version: {}.{}", VersionInfo::getMajor(), VersionInfo::getMinor());

    g_debugger.boot(REMOTE_DEBUGGER_PORT);

    try {
        SPDLOG_INFO("Booting language server");
        auto connection = lsp::Connection(lsp::io::standardIO());
        g_msgHandler = std::make_shared<lsp::MessageHandler>(connection);
        addCallbacks(g_msgHandler);

        g_running = true;

        SPDLOG_INFO("Now running");
        while (g_running) {
            g_msgHandler->processIncomingMessages();
        }

    } catch (const std::exception &e) {
        SPDLOG_ERROR("LSP error: {}", e.what());
        return 1;
    }

    SPDLOG_INFO("Shutting down");

    return 0;
}
