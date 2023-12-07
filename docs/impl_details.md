Fundamentally, Slingshot is a fully modular interface between "engines", that do the real parsing work, and
the LSP protocol. All LSP features, from completion to diagnostics, are driven by
a backend "engine". Currently, completion is driven by
[this ANTLR grammar](https://github.com/antlr/grammars-v4/tree/master/verilog/systemverilog)
and diagnostics are driven by Verilator. ANTLR was specifically chosen because of its readily available SV and its support
for error recovery. Things Slingshot handles itself are project indexing, analysing the parse tree to figure out _what_
to send back to the editor, and LSP communications.

Because I am extremely indecisive, Slingshot has been through many language iterations. It was originally written in
a mix of C++20 and Rust, where the C++20 side handled parsing through the Slang frontend and the Rust side handled
LSP communication. Unfortunately, that was a bit of a nightmare: near constant segfaulting and a hellish build process.

I then decided to Rewrite it in Rust (TM) (R) and use sv-parser. This was actually getting somewhere until we came
to the stage of completion. The Rust sv-parser frontend does not support error recovery, so refuses to generate syntax
trees unless the document is 100% valid. Unfortunately, this means we cannot use it for completion, beacuse we need
to parse the line the user is currently typing to figure out _what_ we should complete for them. With sv-parser, we
would have to recycle old document spans that may not map directly to the currently edited document. antlr-rust is
currently abandoned, and the beta version does not parse documents in the same way as Java (it generates errors for
perfectly valid SV documents, whereas Java does not).

Slingshot is now being written in Kotlin and using Java 17 as the runtime. This _will_ increase memory usage somewhat,
however, the modern JVM has an extremely powerful JIT and GC, so I don't expect latencies to be significantly impacted.
Kotlin is also a significantly more productive language than Rust, so development velocity may be improved.

TODO: simplify this section and explain how completion works (copy and paste the rest into docs/history.md)