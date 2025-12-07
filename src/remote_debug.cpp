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
#include <sockpp/inet_address.h>
#include <sockpp/socket.h>
#include <sockpp/udp_socket.h>
#include <spdlog/spdlog.h>
#include <string>

using namespace slingshot;

void RemoteDebugger::boot(int port) {
    sockpp::initialize();
    socket.bind(sockpp::inet_address("localhost", REMOTE_DEBUGGER_PORT));
    booted = true;
    SPDLOG_INFO("Booted remote debugger on port {}", port);

    thread = std::thread(&RemoteDebugger::debuggerThread, this);
    thread.detach();
}

void RemoteDebugger::debuggerThread() {
    SPDLOG_DEBUG("debuggerThread() entrypoint");

    std::array<char, 512> buf {};
    sockpp::udp_socket::addr_t peer;
    ssize_t n = 0;

    while ((n = socket.recv_from(buf.data(), buf.size(), &peer)) > 0) {
        // copy the socket data into a string (should be ASCII)
        std::string str(buf.data(), n);

        SPDLOG_DEBUG("str is: {}", str);

        // parse command and return the response
        auto response = processMsg(str);
        if (socket.send_to(response + "\n", peer) == -1) {
            SPDLOG_ERROR("Failed to write to client: {}", socket.last_error_str());
            break;
        }

        // fill the buffer with zeroes again (just in case, I guess)
        buf.fill(0);
    }

    SPDLOG_INFO("Connection closed from {}", peer.to_string());
    connected = false;
    socket.close();
}

std::string RemoteDebugger::processMsg(std::string msg) {
    SPDLOG_DEBUG("Received message: {}", msg);

    // this is pretty nasty, we'll fix it up eventually
    if (msg == "dump") {
        return "OK";
    }

    return fmt::format("Command '{}' not found", msg);
}
