module mod (
    input logic foo1,
    input logic clk,
    output logic foo2
);
    wire x = foo;
    always_ff @(posedge clk) begin
        foo2 <= foo1;
    end
endmodule
