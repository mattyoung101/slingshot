# Slingshot - SystemVerilog LSP
**Slingshot** is a language server for the SystemVerilog hardware description language, powered by the 
[slang](https://github.com/MikePopoloski/slang) frontend. Slang is among the fastest and most feature complete 
open-source SystemVerilog frontends, which means that Slingshot should be fast and accurate.

Slingshot was born out of a frustration with existing LSPs for SystemVerilog. The end goal is
to essentially create the "clangd of Verilog", that is, to make the editing experience as smooth as popular
languages like C++ and Python. Slingshot has a particular focus around fast and accurate _complete as you type_,
since this is a feature lacking in existing SV LSPs.

**Current state:** Slingshot has only just started development, and is a long way off implementing any features
described below. So, stay tuned, I guess!

**Timeline:** Hoping to complete some initial tests these holidays after uni exams. Assuming I commit to this,
I'm hoping to get it functional by no later than June 2024.

**Author:** Matt Young (m.young2@uqconnect.edu.au)

## Design goals and features
**Mandatory:**
- Complete-as-you-type
    - Should recommend wires, ports and macros available in the current scope
    - When you're typing ports in a module, it should suggest ports to fill
    - Like clangd, should only suggest completions when it semantically makes sense to
- Diagnostics engine for warnings/errors
    - Should have a pluggable backend to support Verilator, Slang, Verible, and others
- No false positives: If Verilator accepts the input, slingshot should as well
- No false negatives: If Verilator reports an error in the code, slingshot should as well
- Low memory: We should try target no more than 20-50 MB of what slang uses
- Low latency: The LSP should respond quickly to user inputs, even at the cost of CPU usage

**Suggested:**
- Semantic tokens
- Go to definition 
- Formatting (possibly via verible if slang doesn't support it)
- Documentation on hover

**Current NON-goals:**
- CPU usage: SV parsing is expensive, and we are prioritising lower latency. That being said, we still do _care_
about CPU usage (we don't want like 100% CPU all the time), but it's not the #1 priority here.
- Full-compliance with SystemVerilog: SV is a complex language, and I do not (yet!) work in the
industry, so I am basing this plugin off my own personal workload and the support of open-source tools like
Verilator. 
    - This means support for things like UVM and certainly any vendor-specific macros may not work.
    - Slingshot may not like code that does correctly synthesise under industry tools like Synopsys or Cadence.
    - We would welcome PRs that fix any issues you encounter when applying Slingshot to industry designs, and
    Slingshot being wrong in this case _would_ be considered a bug.

## Implementation details
Slingshot will be written in a mix of Rust and C++, the majority being Rust. The C++ side is used to interface
with the Slang parser and extract completion symbols and their associated scopes, as well as diagnostics. This
is then sent over to Rust via an FFI binding, which handles the rest of the language server implementation
via tower-lsp.

The C++ side specifically uses C++20 and CMake, so needs a recent version of both gcc/clang and CMake. I
personally compile with Clang 15.

## What will Slingshot bring to the table?
There are already a few existing LSPs for SV. What does Slingshot aim to bring to the table? Slingshot's main advantage is
its use of the slang frontend and its C++/Rust implementation. Here's a comparison of some other LSPs and what Slingshot
aims to improve on:

- [svlangserver](https://github.com/imc-trading/svlangserver),
    - Written in TypeScript, so requires additional Node.js process overhead
    - Implements its own Verilog frontend, which may be less accurate than slang/Verilator
    - Does not auto-complete inside module ports
- [svls](https://github.com/dalance/svls)
    - Only appears to lint code, does not currently offer other LSP features
- [veridian](https://github.com/vivekmalneedi/veridian)
    - Somewhat outdated
    - Does not auto-complete inside module ports
    - Veridian is the inspiration of this plugin, as it also uses Slang in some configurations

## Licence
Mozilla Public License v2.0
