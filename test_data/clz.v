// 8-bit count leading zeroes (CLZ)
// https://electronics.stackexchange.com/a/412278
module count_lead_zero #(
    parameter W_IN = 8, // Must be power of 2, >=2
    parameter W_OUT = $clog2(W_IN) // Let this default
) (
    input wire  [W_IN-1:0] in,
    output wire [W_OUT-1:0] out
);

generate
if (W_IN == 2) begin: base
    assign out = !in[1];
end else begin: recurse
    wire [W_OUT-2:0] half_count;
    wire [W_IN / 2-1:0] lhs = in[W_IN / 2 +: W_IN / 2];
    wire [W_IN / 2-1:0] rhs = in[0        +: W_IN / 2];
    wire left_empty = ~|lhs;

    count_lead_zero #(
        .W_IN (W_IN / 2)
    ) inner (
        .in  (left_empty ? rhs : lhs),
        .out (half_count)
    );

    assign out = {left_empty, half_count};
end
endgenerate

endmodule
