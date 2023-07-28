# Slingshot - SystemVerilog LSP
**Slingshot** is a **work in progress** language server for the **SystemVerilog** hardware description language.
It also supports **Verilog**, which is a strict subset of SystemVerilog.

Slingshot is written in Kotlin and runs on a Java 17 compliant JVM or higher.

Slingshot was born out of a frustration with existing SystemVerilog LSPs and editor plugins. While many exist,
and most are functional, I still found them imperfect for my needs. Many are missing crucial features like
complete-as-you-type, have rudimentary error reporting, or suffer from other limitations. The ultimate goal of 
Slingshot is to create the _ultimate_ SystemVerilog LSP, with all the features you've come to know and love. 
I'm not sure if I can actually achieve this, but I'm up to trying.

**Current state:** Slingshot has only just started development. Right now, it is only capable of providing
Verilator linting. I am working on adding completion and whole project indexing.

**Timeline:** Due to university, my time is extremely limited. However, I am hoping to get Slingshot fully
functional by no later than June 2024, so I can use it to develop the SV code for my thesis.

**Author:** Matt Young (m.young2@uqconnect.edu.au)

## Currently implemented features
This is the list of _currently_ implemented features. See below for the full list of design goals and planned
features.

- Diagnostics (powered by Verilator)
  - Fully in-memory, does not write any temp files to reduce disk trashing
- Completion (powered by ANTLR)
  - Slingshot is aware of both line and block comments, and does not suggest completions when you are inside
    a comment

## Building and running
**Toolchain**

You will need:
- Java 17 or higher

**Building Slingshot**

The program can be built with `./gradlew build` and an executable JAR file can be made with `./gradlew shadowJar`.

Because the SystemVerilog generated parser is so massive, you will need to modify IntelliJ's max file parse
size to be larger. Go to Help -> Edit Custom Properties and insert `idea.max.intellisense.filesize=999999`.
Then, restart the IDE.

**Running**

Currently I have only tested Slingshot in Neovim.

When Slingshot is a more capable LSP, it will (hopefully) be available in upstream LSP projects like
[mason.nvim](https://github.com/williamboman/mason.nvim) and [nvim-lspconfig](https://github.com/neovim/nvim-lspconfig).

Until then, you can manually add Slingshot as a nvim-lspconfig server by inserting the following into `init.lua`:

```lua
local lspconfig = require 'lspconfig'
local configs = require 'lspconfig.configs'

if not configs.slingshot then
  -- this require lspconfig.configs is the trick required to make it work
  require("lspconfig.configs").slingshot = {
    default_config = {
      cmd = {'java', '-jar', '/home/matt/workspace/slingshot/build/libs/slingshot-1.0-SNAPSHOT-all.jar'};
      filetypes = {'verilog', 'systemverilog'};
      root_dir = function(fname)
        return lspconfig.util.find_git_ancestor(fname) or vim.loop.os_homedir()
      end;
      settings = {};
    };
  }
end

lspconfig.slingshot.setup{}
```

This is the setup I use for development as well.

## Design goals
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

## Implementation details
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

## Licence
Mozilla Public License v2.0
