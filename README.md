# Slingshot - SystemVerilog LSP
**Slingshot** is a **work in progress** language server for the SystemVerilog hardware description language, 
currently powered by dalance's [sv-parser](https://github.com/dalance/sv-parser).

Slingshot was born out of a frustration with existing SystemVerilog LSPs and editor plugins. While many exist,
and most are functional, I still found them imperfect for my needs. Many are missing crucial features like
complete-as-you-type, have rudimentary error reporting, or suffer from other limitations. The ultimate goal of 
Slingshot is to create the _ultimate_ SystemVerilog LSP, with all the features you've come to know and love. 

Of course this is a moonshot, but here's to trying!

**Current state:** Slingshot has only just started development, and is a long way off implementing any features
described below. So, stay tuned, I guess.

**Timeline:** Hoping to complete some initial tests these holidays after uni exams. Assuming I commit to this,
I'm hoping to get it functional by no later than June 2024.

**Author:** Matt Young (m.young2@uqconnect.edu.au)

## Building and running
**Toolchain**

You will need:
- Rust, latest stable version, at least 1.70

**Building Slingshot**

You should now be able to build Slingshot with just `cargo build`, fingers crossed.

**Running**

Slingshot currently does not function as a LSP (i.e. the LSP backend has not yet been written). When it has
been, I will add instructions for using it here.

## Design goals and features
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

- Low latency: The LSP should respond quickly to user inputs, even at the cost of CPU usage

**Suggested:**
- Semantic tokens for semantic highlighting
- Go to definition 
- Formatting (possibly via verible if slang doesn't support it)
- Documentation on hover (extract from comments)
- No false negatives: If Verilator reports an error in the code, slingshot should as well
    - This is probably harder than no false positives to achieve, based on how the project indexing will likely
    work
- Low memory: We should try to use significantly less resident RAM than other SV LSPs
    - Hopefully this should naturally happen because of C++/Rust, but manual optimisation may be required

**Current NON-goals:**
- CPU usage: SV parsing is expensive, and we are prioritising lower latency. That being said, we still do _care_
about CPU usage (we don't want like 100% CPU all the time), but it's not the #1 priority here.
- Full-compliance with SystemVerilog: SV is a complex language, and I do not (yet!) work in the
industry, so I am basing this plugin off my own personal workload and the support of open-source tools like
Slang. 
    - Most parsing is done by an engine, not by Slingshot itself. Most parsing issues will be issues in these
    upstream engines, not Slingshot itself.
    - If Slingshot is unable to understand your project structure (and you've configured it correctly), this is
    a bug in Slingshot and should be reported to me.
    - If you are able to share snippets of code from your industry projects (unlikely), or produce minimum
    reproducible examples (more likely), these would be greatly appreciated.

## Implementation details
Fundamentally, Slingshot is a fully modular interface between "engines", that do the real parsing work, and
the LSP protocol. All LSP features, from completion to diagnostics, are driven by
a backend "engine". Currently, completion is driven by sv-parser and diagnostics are driven by Verilator,
but I aim to make these fully runtime configurable. Things Slingshot handles itself are project indexing and
LSP communications.

Slingshot is currently written in just Rust. In a past life, it was written in a mix of Rust and C++20 to
interface with the Slang SystemVerilog frontend developed by Mike Popoloski. Unfortunately, that proved
extremely difficult to work with from both the Rust and C++ side - the worst of both worlds, constant segfaulting,
and a nightmarish build process. So that has been scrapped, and I'm trying just Rust for now. _However_, if
sv-parser is not suitable for the task at hand, then at this point I'll bundle a Slang executable that starts
a server and communicates with the main Slingshot process via IPC.

sv-parser does not have good error recovery support. Slingshot will therefore
have to make a "best guess" attempt at providing useful feedback while the user is typing, probably by splicing
lines that are causing errors. In an ideal world where I have unlimited time to dedicate to this project, I'd
write a new SV parser using chumsky, but that is an absolutely mammoth task and a project unto itself. This is
massively annoying because one of Slingshot's _primary goals_ is accurate autocomplete. So, as nice as Rust is,
if sv-parser is not capable of good enough error recovery to be useable, we will have no choice but to move to
C++, which in turn will make the LSP side of things a massive pain.

Alto, to note, this project doubles as my way of learning Rust, so bare with me if it's not idiomatic.

## Licence
Mozilla Public License v2.0
