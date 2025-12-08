// Slingshot: A SystemVerilog language server.
//
// Copyright (c) 2025 M. L. Young.
//
// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0. If a copy of the MPL
// was not distributed with this file, You can obtain one at https://mozilla.org/MPL/2.0/.
#pragma once
#include <sockpp/inet_address.h>
#include <sockpp/socket.h>
#include <sockpp/tcp_acceptor.h>
#include <sockpp/tcp_socket.h>
#include <sockpp/udp_socket.h>
#include <string>
#include <thread>

namespace slingshot {

/// This is a remote debugging tool that provides a simple networked command CLI for interacting with the
/// server
class RemoteDebugger {
public:
    void boot(int port);

    void shutdown();

    bool isBooted() const {
        return booted;
    }

    bool isConnected() const {
        return connected;
    }

private:
    sockpp::tcp_acceptor acceptor;
    sockpp::tcp_socket socket;
    sockpp::inet_address peer;
    std::thread thread;

    void debuggerThread();
    std::string processMsg(std::string msg);

    bool booted;
    bool connected;
};

}; // namespace slingshot
