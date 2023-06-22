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
    std::cout << "Running token extractor" << std::endl;

    slingshot_free_str(slingshot_version);
    slingshot_free_str(slang_version);

    std::string document = R"(
    `timescale 1ns/1ns
    `default_nettype none

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

        always_ff @(posedge i_clk) begin
            if (i_rst) begin
                ctr <= 0;
            end else begin
                // every 6 cycles, give a rising edge to i_clk, this should mean i_clk is 1 MHz as required
                if (ctr == 6) begin
                    o_clk <= 1;
                    // reset counter !important!
                    ctr <= 0;
                end else begin
                    o_clk <= 0;
                    // increment timer
                    ctr <= ctr + 1;
                end
            end
        end
    endmodule    
    )";

    auto result = slingshot_extract_completion_tokens(document.c_str(), true);
    slingshot_free_completion(result);

    return 0;
}
