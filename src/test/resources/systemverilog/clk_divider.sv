`timescale 1ns/1ns
`default_nettype none

typedef enum logic[2:0] {
    ENUM_A,
    ENUM_B
} Thing_t;

typedef struct {
    logic[2:0] x;
} Struct_t;

// This module implements a 6 MHz -> 1 MHz clock divider
// TODO parameterise this
module clk_div_6to1 (
    // Input 6 MHz clock
    input logic i_clk,

    // Reset line
    input logic i_rst,

    // Output 1 MHz clock
    output logic o_clk
);
    // Counter
    logic[2:0] ctr;
    logic[4:0] xyz;
    Thing_t thingt;

    /*
     * this is a block comment and that's why the block comment is like this
     */

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