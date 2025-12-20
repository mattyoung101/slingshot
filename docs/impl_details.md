## "Lang" module
The `slingshot::lang` module, usually referred to as "lang", contains an internal very high level representation of
(System)Verilog for the purposes of queries for the completion system. It is defined in
`slingshot/language.hpp`.

The lang module has a SAX-like system for constructing its high level representation out of Slang parse trees,
with routines like `startModule()` and `endModule()`.

It can also be serialised to JSON using `nlohmann_json`, and the intent is eventually to have it so that, like
clangd, documents do not have to be recompiled from scratch on server boot.

The "lang lifter", defined in `lang_lifter.cpp/lang_lifter.hpp` is a Slang parse tree visitor that "lifts"
SystemVerilog to this high-level representation. By the strict definition of "lifting", it's not really like
traditional lifters that do say x86 assembly to LLVM IR, but I named it like this as the inspiration.

## Compiler module
The compiler module, defined in `compiler.cpp/compiler.hpp` wraps Slang and performs:
- Concrete Syntax Tree (CST) initial parsing
- Abstract Syntax Tree (AST) parsing and analysis for diagnostics
- Higher level analysis (currently broken, probably?) using Slang's `AnalysisManager`
- Language lifting to the internal lang module for completion

The compiler uses a thread pool and operates asynchronously in the background. Compilation jobs can be
submitted to the pool. Once the compilation job has finished, the job will call `IndexManager` methods to
associate index entries with their updated parse trees and other results.

## Remote debug module
It can be hard to extract information from the server while it's running, for the purposes of debugging - for
example, to dump all the lang trees. For this reason, there's a remote debugger module. It binds a local TCP
server on port `6942` that can receive and execute commands.

There's a Python script, `./scripts/remote_debugger.py` for interacting with this.

The list of commands are defined at the bottom of `remote_debug.cpp`.

## Indexing module
Slingshot features a relatively competent, thread-safe (hopefully) indexing system. The basic purposes is to
associate a LSP file path with an `IndexEntry`, which contains a bunch of data about compilations, parse
trees, lang documents, etc.

The index is at the core of Slingshot and is how most other modules interact with each other, by pushing or
pulling data from the index.

The intent is to one day be able to serialise and deserialise the index to/from disk, but this is troublesome
due to the nature of C++ being a dogwater programming language. Hence, _deserialising_ internal Slang syntax
nodes is very complicated and something I would need to collaborate with the Slang project itself on.

## Completion system
The completion system is how the server attempts to understand SystemVerilog and decide what you want to type
next.

This is achieved by the completion system in `completion.cpp` walking the Slang CST and visiting a bunch of
nodes. If your cursor is inside a particular node, the completion system will call out to the completion
generator in `completion_generator.cpp`, which will make queries on the lang document, to for example
recommend ports in the module you're typing in.

Because the tree is walked from top to bottom, completion "recommendations" as they're called, get more and
more precise as it gets to the specific node your cursor is in.

For example, if you're typing `always_ff (<cursor>)`, we may first try and recommend you ports, since you're
inside a module. Later, we'll visit the Always block, and determine that you're inside an `always_ff`
sensitivity list, so we know to recommend you `posedge`.

It's pretty straightforward, but hopefully quite effective.

In the completion system, the `BEGIN` macro is used to do the cursor checking, and then the `RECOMMEND` macro
is used to actually push recommendations.
