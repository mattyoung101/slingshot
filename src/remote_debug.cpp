// Slingshot: A SystemVerilog language server.
//
// Copyright (c) 2025 M. L. Young.
//
// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0. If a copy of the MPL
// was not distributed with this file, You can obtain one at https://mozilla.org/MPL/2.0/.
#include "slingshot/remote_debug.hpp"
#include "slingshot/slingshot.hpp"
#include <array>
#include <csignal>
#include <sockpp/acceptor.h>
#include <sockpp/inet_address.h>
#include <sockpp/socket.h>
#include <sockpp/tcp_acceptor.h>
#include <sockpp/udp_socket.h>
#include <spdlog/spdlog.h>
#include <string>

using namespace slingshot;

void RemoteDebugger::boot(int port) {
    sockpp::initialize();
    acceptor = sockpp::tcp_acceptor(port);
    booted = true;
    SPDLOG_INFO("Booted remote debugger on port {}", port);

    thread = std::thread(&RemoteDebugger::debuggerThread, this);
    thread.detach();
}

void RemoteDebugger::debuggerThread() {
    SPDLOG_DEBUG("debuggerThread() entrypoint");

    while (true) {
        // wait for a debugger client to be connected
        SPDLOG_INFO("Now waiting for a client");
        socket = acceptor.accept(&peer);
        SPDLOG_INFO("Client connected: {}", peer.to_string());

        if (!socket) {
            SPDLOG_ERROR("Could not accept incoming connection: {}", acceptor.last_error_str());
            continue;
        }

        while (socket.is_open()) {
            std::array<char, 512> buf {};
            ssize_t n = socket.read(buf.data(), buf.size());
            if (n <= 0) {
                break;
            }

            // copy the socket data into a string (should be ASCII)
            std::string str(buf.data(), n);
            SPDLOG_TRACE("Received {} bytes: {}", n, str);

            // parse command and return the response
            auto response = processMsg(str);
            if (socket.write(response + "\nEND_TRANSMISSION\n") == -1) {
                SPDLOG_ERROR("Failed to write to client: {}", socket.last_error_str());
                break;
            }

            // fill the buffer with zeroes again (just in case, I guess)
            buf.fill(0);
        }
    }

    SPDLOG_INFO("Connection closed from {}", peer.to_string());
    connected = false;
    socket.close();
}

std::string RemoteDebugger::processMsg(std::string msg) {
    SPDLOG_DEBUG("Received message: {}", msg);

    // this is pretty nasty, we'll fix it up eventually
    if (msg == "dump index") {
        return g_indexManager.debugDump();
    }
    if (msg == "dump lang") {
        return g_indexManager.dumpLangTrees();
    }
    if (msg == "sigtrap") {
        SPDLOG_WARN("Debugger sent 'sigtrap', raising SIGTRAP now");
        raise(SIGTRAP);
        return "OK";
    }
    if (msg == "die") {
        SPDLOG_WARN("Debugger sent 'die', killing server now");
        exit(1);
        return "OK";
    }

    return fmt::format("Command '{}' not found", msg);
}
