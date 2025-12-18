// 8-bit count leading zeroes (CLZ)
// https://electronics.stackexchange.com/a/412278
module count_lead_zero #(
    parameter W_IN = 8, // Must be power of 2, >=2
    parameter W_OUT = $clog2(W_IN) // Let this default
) (
    input wire  [W_IN-1:0] in,
    output wire [W_OUT-1:0] out,
    input wire clk
);

    always_ff @(posedge completely_invalid) begin
        out <= 1;
    end

endmodule
