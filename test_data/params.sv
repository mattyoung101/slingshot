module mod # (
    parameter int FOO = 2,
    parameter int BAR = 4
) (
    input logic foo1,
    input logic clk,
    output logic foo2
);
    always_ff @(posedge clk) begin
        foo2 <= foo1;
    end
endmodule
