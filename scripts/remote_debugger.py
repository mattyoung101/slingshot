#!/usr/bin/env python3
# Slingshot: A SystemVerilog language server.
#
# Copyright (c) 2025 M. L. Young.
#
# This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0. If a copy of the MPL
# was not distributed with this file, You can obtain one at https:#mozilla.org/MPL/2.0/.

import socket


def main():
    s = socket.socket(socket.AF_INET, socket.SOCK_DGRAM)
    addr = ("localhost", 6942)

    print("Slingshot Remote Debugger")

    while True:
        try:
            cmd = input("> ").strip()
        except EOFError:
            print("Goodbye.")
            return

        s.sendto(cmd.encode("ascii"), addr)

        response, server = s.recvfrom(1024)
        print(response.decode("ascii").strip())


if __name__ == "__main__":
    main()
