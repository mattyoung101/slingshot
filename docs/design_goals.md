**Mandatory:**
- Complete-as-you-type
    - Should recommend wires, ports and macros available in the current scope
    - When you're typing ports in a module, it should suggest ports to fill
    - Like clangd, should only suggest completions when it semantically makes sense to do so
- Diagnostics engine for warnings/errors
    - Should have a pluggable backend to support Verilator, Slang, Verible, and others
- Whole project indexing
    - Slingshot should discover referenced files and add them to an index cache
- No false positives: If Verilator accepts the input, slingshot should as well
- Low latency: The LSP should respond quickly to user inputs, even at the cost of CPU usage. This is our most
  important metric.
- Snippet support
    - e.g. if you type `always_ff` it should auto-generate an `always_ff` block

**Suggested:**
- Semantic tokens for semantic highlighting
- Go to definition
- Documentation on hover (extract from comments)
    - Requires our parser to match comments or us to regex it
- No false negatives: If Verilator reports an error in the code, slingshot should as well
    - This is probably harder than no false positives to achieve, based on how the project indexing will likely
      work

**Current NON-goals:**
- CPU usage: SV parsing is expensive, and we are prioritising lower latency. That being said, we still do _care_
  about CPU usage (we don't want like 100% CPU all the time), but it's not the #1 priority here.
- Low memory: This was originally a goal when Slingshot was written in compiled languages, but now that we are
  using the JVM, low memory usage is difficult without reducing our most important metric: latency.
- Full-compliance with SystemVerilog: SV is a complex language, and I do not (yet!) work in the
  industry, so I am basing this plugin off my own personal workload and the support of open-source tools.
    - If a document fails to parse, this is most likely an upstream issue, either with Verilator or the ANTLR
      grammar in use.
    - If Slingshot is unable to understand your project structure (and you've configured it correctly), this is
      a bug in Slingshot and should be reported to me.
    - If you are able to share snippets of code from your industry projects, or produce minimum
      reproducible examples, these would be greatly appreciated. I can also work with log files (/tmp/slingshot.log).