## "Lang" module
The `slingshot::lang` module, usually referred to as "lang", contains an internal very high level representation of
(System)Verilog for the purposes of queries for the completion system. It is defined in
`slingshot/language.hpp`.

The lang module has a SAX-like system for constructing its high level representation out of Slang parse trees,
with routines like `startModule()` and `endModule()`.

It can also be serialised to JSON using `nlohmann_json`, and the intent is eventually to have it so that, like
clangd, documents do not have to be recompiled from scratch on server boot.

The "lang lifter", defined in `lang_lifter.cpp/lang_lifter.hpp` is a Slang parse tree visitor that "lifts"
a Slang SV CST to this high-level representation. By the strict definition of "lifting", it's not really like
traditional lifters that do say x86 assembly to LLVM IR, but I named it like this as the inspiration.

## Compiler module
The compiler module, defined in `compiler.cpp/compiler.hpp` wraps Slang and performs:
- Concrete Syntax Tree (CST) initial parsing
- Abstract Syntax Tree (AST) parsing and analysis for diagnostics
- Higher level analysis (currently broken, probably?) using Slang's `AnalysisManager`
- Language lifting to the internal "lang" module for completion
- Dependency graph computation

The compiler runs as a separate asynchronous thread and uses a blocking work queue to manage jobs. Compilation
jobs can be submitted to the work queue and will be processed in order.

When pulling jobs from the work queue, the compilation manager thread will compare the nanosecond timestamp of
the work item and compare it against the last updated time of the item in the index. If the item in the index
is newer, the work item will be discarded to prevent processing of outdated data.

Once the compilation job has finished, the job will call `IndexManager` methods to associate index entries
with their updated parse trees and other results.

## Remote debug module
It can be hard to extract information from the server while it's running, for the purposes of debugging - for
example, to dump all the lang trees. For this reason, there's a remote debugger module. It binds a local TCP
server on port `6942` that can receive and execute commands.

There's a Python script, `./scripts/remote_debugger.py` for interacting with this.

The list of commands are defined at the bottom of `remote_debug.cpp`, and now you can also type `help` to
see a list of all the commands.

## Indexing module
Slingshot features a relatively competent, thread-safe (hopefully) indexing system. The basic purposes is to
associate a LSP file path with an `IndexEntry`, which contains a bunch of data about compilations, parse
trees, lang documents, etc.

The index is at the core of Slingshot and is how most other modules interact with each other, by pushing or
pulling data from the index.

The intent is to one day be able to serialise and deserialise the index to/from disk, but this is troublesome
due to the nature of C++ being a dogwater programming language. Hence, _deserialising_ internal Slang syntax
nodes is very complicated and something I would need to collaborate with the Slang project itself on.

### Dependency graph
The index also includes a dependency graph, which is modelled as a DAG (Directed Acyclic Graph) using the
Graaf library.

The import locator (`import_locator.cpp/.h`) analyses the CST of a SV document to figure out both what symbols
the document exports, and what symbols is required. Certain symbols in the document may not be clear just by
parsing the CST if they are actually required imported symbols, or if they are local to the document and not
imported at all. For this reason, we also store so-called "maybe-required" symbols, which will be kept until
at least indexing is complete.

Then, the document graph (`document_graph.cpp/.h`) builds a DAG using the Graaf library. The graph is modelled
as:

```
A ---(sym)---> B
```

where "A" is the document that provides the symbol "sym" to document "B".

The document graph subsystem also maintains an inverted version of the graph; for example in the above example
it would look like:

```
B ---(sym)---> A
```

To locate the required documents for a given document, the document graph subsystem performs a breadth-first
search (BFS) on the inverted version of the graph; i.e. a backwards BFS on the regular graph.

Because the document graph is modelled as a DAG, cycles are not permitted, and the server will complain
extensively if it detects cycles in the document graph. This can be debugged most easily using the remote
debugger defined above.

### Building the index
The index is built by recursively walking the include dirs from the config file, or by parsing an F-list file.
Then, each document is added to the `CompilationManager` work queue for indexing.

The indexing process compiles the CST and performs the lang lifting, but does not parse the AST. This is also
added to the dependency graph.

Once the dependency graph is built, it's finalised to resolve any outstanding dependencies. The document graph
attempts to greedily connect unconnected symbols if they are present. This includes "maybe required" symbols,
which are still present at this stage.

Then, a bulk compilation is performed serially: each document is compiled in order to generate ASTs, CSTs and
lang documents. During the bulk compilation, the dependents for every document in the index are calculated
using the backwards BFS mentioned above on the document graph.

At the end of the initial indexing stage, the document graph now purges all unresolved "maybe required"
symbols under the assumption that they were not required. This helps to reduce memory and improve the
performance of the document graph.

Once the dependency graph is computed, the diagnostics are re-evaluated for all open clients and resubmitted.
The index will not be entirely re-built again until the server is killed. However, when typing, if Slingshot
detects that the list of imports has changed (i.e. the hash of the import table has changed), it will re-build
the import table for that particular file.

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
