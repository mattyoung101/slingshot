// Slingshot: A SystemVerilog language server.
//
// Copyright (c) 2025 M. L. Young.
//
// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0. If a copy of the MPL
// was not distributed with this file, You can obtain one at https://mozilla.org/MPL/2.0/.
#include "lsp/connection.h"
#include "lsp/io/standardio.h"
#include "lsp/messagehandler.h"
#include "lsp/messages.h"
#include "slang/util/VersionInfo.h"
#include "slingshot/compiler.hpp"
#include "slingshot/handlers.hpp"
#include "slingshot/slingshot.hpp"
#include "spdlog/sinks/rotating_file_sink.h"
#include "spdlog/spdlog.h"
#include <csignal>
#include <exception>
#include <filesystem>
#include <memory>
#include <spdlog/sinks/ansicolor_sink.h>
#include <unistd.h>

namespace {

void addCallbacks(std::shared_ptr<lsp::MessageHandler> &msgHandler) {
    msgHandler->add<lsp::requests::Initialize>(slingshot::handlers::initialise);
    msgHandler->add<lsp::requests::Shutdown>(slingshot::handlers::shutdown);
    msgHandler->add<lsp::notifications::Exit>(slingshot::handlers::exit);
    msgHandler->add<lsp::notifications::TextDocument_DidOpen>(slingshot::handlers::textDocumentOpen);
    msgHandler->add<lsp::notifications::TextDocument_DidChange>(slingshot::handlers::textDocumentChange);
    msgHandler->add<lsp::requests::TextDocument_Diagnostic>(slingshot::handlers::textDocumentDiagnostic);
    msgHandler->add<lsp::requests::TextDocument_Completion>(slingshot::handlers::textDocumentCompletion);
    msgHandler->add<lsp::notifications::TextDocument_DidClose>(slingshot::handlers::textDocumentClose);
}

void sigIntHandler(int signal) {
    SPDLOG_INFO("Caught SIGINT");

    // TODO flush index and so on

    exit(0);
}

} // namespace

namespace slingshot {
bool g_running = false;
IndexManager g_indexManager = {};
CompilationManager g_compilerManager = {};
RemoteDebugger g_debugger = {};
std::shared_ptr<lsp::MessageHandler> g_msgHandler = {};
CompletionManager g_completionManager = {};
} // namespace slingshot

int main() {
    using namespace slang;
    using namespace slingshot;

    auto level = spdlog::level::debug;

    spdlog::set_level(level);
    spdlog::flush_on(level);

    // keep stderr free for the LSP
    auto stderr_sink = std::make_shared<spdlog::sinks::ansicolor_stderr_sink_mt>();
    stderr_sink->set_level(level);
    spdlog::default_logger()->sinks().clear();
    spdlog::default_logger()->sinks().push_back(stderr_sink);

    // and do a file sink as well
    std::string homeDir;

    {
        char *homeDirPtr = getenv("HOME");
        if (homeDirPtr == nullptr) {
            SPDLOG_ERROR("Failed to get home dir from HOME env var");
            return 1;
        }
        homeDir = homeDirPtr;
    }

    std::filesystem::create_directories(homeDir + "/.local/share/slingshot");

    int pid = getpid();

    std::filesystem::path logPath
        = homeDir + "/.local/share/slingshot/slingshot-" + std::to_string(pid) + ".log";
    auto rotating = std::make_shared<spdlog::sinks::rotating_file_sink_mt>(logPath, 4096 * 1024, 5, false);
    rotating->set_level(level);
    spdlog::default_logger()->sinks().push_back(rotating);

    signal(SIGINT, sigIntHandler);

    spdlog::default_logger()->set_pattern("[%Y-%m-%d %H:%M:%S.%e thread %t] [%^%l%$] [%s:%# %!] %v");
    for (auto &sink : spdlog::default_logger()->sinks()) {
        sink->set_pattern("[%Y-%m-%d %H:%M:%S.%e thread %t] [%^%l%$] [%s:%# %!] %v");
    }

    SPDLOG_INFO("Slingshot LSP - (c) 2023-2025 M. L. Young. Licenced under the MPL 2.0.");
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

    return 0;
}
