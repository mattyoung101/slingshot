# Slingshot indexing
This document contains information about how the Slingshot will index SystemVerilog projects to provide project-wide
completion. For example, if a module `testing` is defined in `file2.sv`, you can type `test` in `file1.sv` and
it will be completed to `testing.sv`.

The overall indexing methodology is heavily based on clangd.

## Indexing scope
Indexing and caching is _per project_, so all files will be stored in the same directory as the project. For
example, `/home/matt/workspace/project/.cache/slingshot/slingshot_index.dat`. This is inspired by clangd.

## Building a simplified hierarchy of the project
Because we not only want to complete module instantiations, but also the ports inside these modules, we need
to build a simplified hierarchy of the project. That hierarchy will look like this:

```
Document (root)
    Module
        Port
        Port
        ModulePrivate
            Logic, Wire, Etc
    Module
        Port
    Enum
        EnumEntry
        EnumEntry
```

Document is the root, which can contain multiple Modules, which in turn can contain multiple Ports. We also
record private details like Logics, Wires, etc, for when you are editing that specific module.

We store items like Enums separately as well.

TODO: what about classes and structs?

## Acquiring symbols
Each document's symbols are provided by the currently selected completion provider. At the moment this will
be sv-lang. It returns the above document hierarchy for each module.

Every time you edit a file, the completion is rerun and the results are sent to the LSP. However, we also
asynchronously send the list of symbols to the IndexManager as well, so that it can update the global index
and write out the results to disk if necessary.

## Caching
We want to avoid having to reindex the entire project on boot if nothing has changed. 

We will record a BTreeMap of absolute file paths (keys) and their XXHash3 hash (values). When Slingshot loads,
it will scan all the files it can detect in the project and compute their hash. 

If:
- each file in the project is in the cache; AND
- the project contains no files that are not in the cache (i.e. no new files have been added); AND
- each file in the project matches the xxhash of that in the cache

Then we load the cache and use it. Otherwise, we trigger a full project reindex (we could instead be smart and
only index missing files). 

### Method 1: Serde
We write a `Vec<Document>` to disk. We want to avoid overwriting the document every time you type a letter,
so we'll have to batch updates or something like that. I'd say we would batch updates into groups, maybe
have a "max flush only every 1 minute" thing, do it asynchronously always, and only flush if the hash of
`Vec<Document>` is _different_ to the one we last flushed to disk (maybe skip this check, because if we're
typing, it almost certainly will be different).

### Method 2: Key-value store
We could use an embedded key value store like Redis to do this work for us. There might be Rust-specific
embedded key value stores which we could use as well. (redb, sled)

## Returning completions
The primary goal for the index is to return valid SV completions across files boundaries, for the entire
project. What we basically want to know is whether a partial symbol like "mymod..." will resolve into a 
full symbol like "mymodule", and do so with lexically valid scoping (e.g. suggest port completions only from
that module when you are filling in ports).

This implies a trie data structure, which we might use with trie_rs. However, tries can only tell you if
a word _starts_ with a particular set of characters and trie_rs also does not support serialisation. What
we'll do for now is the naive O(n^2) method where we check every document and every module. In the future
we should look for more optimal methods to locate these though.

## Future improvements
Note that in the future we will probably also want the index to record things like document comments which
will be saved in the `SvDocument` struct.

