// Slingshot: A SystemVerilog language server.
//
// Copyright (c) 2025 M. L. Young.
//
// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0. If a copy of the MPL
// was not distributed with this file, You can obtain one at https://mozilla.org/MPL/2.0/.
#include "slingshot/remote_debug.hpp"
#include <array>
#include <sockpp/socket.h>
#include <sockpp/tcp_acceptor.h>
#include <sockpp/tcp_connector.h>
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
        SPDLOG_DEBUG("Now waiting for a client");
        socket = acceptor.accept(&peer);
        SPDLOG_INFO("Client connected: {}", peer.to_string());

        if (!socket) {
            SPDLOG_ERROR("Could not accept incoming connection: {}", acceptor.last_error_str());
            continue;
        }

        while (socket.is_open()) {
            ssize_t n = 0;
            std::array<char, 512> buf {};

            while ((n = static_cast<ssize_t>(socket.read(buf.data(), buf.size()) > 0)) != 0) {
                // copy the socket data into a string (should be ASCII)
                std::string data;
                data.resize(buf.size());
                for (const auto &c : buf) {
                    data.push_back(c);
                }

                // parse command and return the response
                auto response = processMsg(data);
                if (socket.write(response) == -1) {
                    SPDLOG_ERROR("Failed to write to client: {}", socket.last_error_str());
                    break;
                }

                // fill the buffer with zeroes again (just in case, I guess)
                buf.fill(0);
            }

            SPDLOG_INFO("Connection closed from {}", socket.peer_address().to_string());
            connected = false;
            socket.close();
        }
    }
}

std::string RemoteDebugger::processMsg(const std::string &msg) {
    SPDLOG_DEBUG("Received message: {}", msg);
    return "OK";
}
