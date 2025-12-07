#!/usr/bin/env python3
# Slingshot: A SystemVerilog language server.
#
# Copyright (c) 2025 M. L. Young.
#
# This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0. If a copy of the MPL
# was not distributed with this file, You can obtain one at https:#mozilla.org/MPL/2.0/.

import socket


def main():
    s = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
    s.connect(("localhost", 6942))

    print("Slingshot Remote Debugger")

    while True:
        try:
            cmd = input("> ")
        except EOFError:
            print("Goodbye.")
            return

        s.send(cmd.encode("ascii"))

        response = s.recv(512).decode("ascii")
        print(response)


if __name__ == "__main__":
    main()
