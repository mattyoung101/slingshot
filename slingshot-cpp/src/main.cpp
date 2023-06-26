/*
 * Copyright (c) 2023 Matt Young.
 *
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */
#include <iostream>
#include "slingshot/slingshot.h"
#include <cstdlib>

int main(void) {
    char *slingshot_version = slingshot_get_cpp_version();
    char *slang_version = slingshot_get_slang_version();
    
    std::cout << "Slingshot CPP version: " << slingshot_version << std::endl;
    std::cout << "Slang version: " << slang_version << std::endl;

    slingshot_free_str(slingshot_version);
    slingshot_free_str(slang_version);

    std::string document = R"(
    `timescale 1ns/1ns
    `default_nettype none
    `define THING 2

    typedef enum logic[7:0] {
        ENUM_A = 0,
        ENUM_B = 1
    } Enum_t;

    // This module implements a 6 MHz -> 1 MHz clock divider
        module clk_div_6to1 #(
        parameter int PARAM = 0
        ) (
        // Input 6 MHz clock
        input logic i_clk, 

        // Reset line
        input logic i_rst,

        // Output 1 MHz clock
        output logic o_clk
    );
        // Counter
        logic[2:0] ctr;

        deez;
    endmodule

    module mod2 (
        input logic mod2_a,
        input logic mod2_b
    )
    endmodule
    )";

    // std::cout << "Running token extractor" << std::endl;
    // auto result = slingshot_complete(document.c_str(), true);
    // slingshot_free_completion(result);

    std::cout << "Running diagnostics" << std::endl;
    auto result2 = slingshot_diagnose("hello world", true);
    slingshot_free_diagnostics(result2);

    return 0;
}
