module mod (
    input logic foo1,
    output logic foo2
);
    assign foo1 = foo2;

    always_ff @(posedge blah) begin
        foo1 = foo2;
    end
endmodule
