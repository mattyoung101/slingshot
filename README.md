# Slingshot - SystemVerilog LSP
**Slingshot** is a **work in progress** language server for the SystemVerilog hardware description language, powered by the 
[slang](https://github.com/MikePopoloski/slang) frontend. Slang is among the fastest and most feature complete 
open-source SystemVerilog frontends, which means that Slingshot should be fast and accurate. Slingshot is
developed in a mix of Rust and C++20.

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

## Design goals and features
**Mandatory:**
- Complete-as-you-type
    - Should recommend wires, ports and macros available in the current scope
    - When you're typing ports in a module, it should suggest ports to fill
    - Like clangd, should only suggest completions when it semantically makes sense to do so
- Diagnostics engine for warnings/errors
    - Should have a pluggable backend to support Verilator, Slang, Verible, and others
- No false positives: If Verilator accepts the input, slingshot should as well
- No false negatives: If Verilator reports an error in the code, slingshot should as well
- Low latency: The LSP should respond quickly to user inputs, even at the cost of CPU usage

**Suggested:**
- Semantic tokens for semantic highlighting
- Go to definition 
- Formatting (possibly via verible if slang doesn't support it)
- Documentation on hover (extract from comments)
- Low memory: We should try to use significantly less resident RAM than other SV LSPs
    - Hopefully this should naturally happen because of C++/Rust, but manual optimisation may be required

**Current NON-goals:**
- CPU usage: SV parsing is expensive, and we are prioritising lower latency. That being said, we still do _care_
about CPU usage (we don't want like 100% CPU all the time), but it's not the #1 priority here.
- Full-compliance with SystemVerilog: SV is a complex language, and I do not (yet!) work in the
industry, so I am basing this plugin off my own personal workload and the support of open-source tools like
Slang. 
    - Parsing is done by the upstream Slang project, so any Slingshot bugs that are actually parse errors in
    Slang should be reported to them.
    - If Slingshot is unable to understand your project structure (and you've configured it correctly), this is
    a bug in Slingshot and should be reported to us.
    - We would welcome PRs to this project or Slang that fix any issues you encounter when applying Slingshot to 
    industry designs, if you are able to contribute.

## Implementation details
Slingshot will be written in a mix of Rust and C++, the majority being Rust. The C++ side is used to interface
with the Slang parser and extract completion symbols and their associated scopes, as well as diagnostics. This
is then sent over to Rust via an FFI binding, which handles the rest of the language server implementation
via tower-lsp.

The C++ side specifically uses C++20 and CMake, so needs a recent version of both gcc/clang and CMake. I
personally compile with Clang 15.

This project doubles as my way of learning Rust, so bare with me if it's not idiomatic. We are targeting the
latest stable Rust, currently 1.70.0.

## Licence
Mozilla Public License v2.0
