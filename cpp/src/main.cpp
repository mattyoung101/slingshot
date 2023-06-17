#include <iostream>
#include "slingshot/token_extractor.hpp"
#include "slingshot/slingshot.hpp"

using namespace slingshot;

int main(void) {
    std::cout << "Slingshot CPP version: " << SLINGSHOT_CPP_VERSION << std::endl;
    std::cout << "Running token extractor" << std::endl;

    CompletionTokenExtractor extractor;
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

    extractor.extractTokens(document);

    return 0;
}
