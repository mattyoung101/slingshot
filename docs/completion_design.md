# Slingshot completion
One of Slingshot's biggest focuses is fast and accurate auto-complete. The goal is to end up with a
completion system that's just as good as the major players in other languages, like clangd and rust-analyzer.

How do we achieve this?

## Context
Where are we in the stack? When completion has run, we have already parsed the document and updated the
index, so we need not worry about that. Our goal is to understand where the cursor is in the document, and
thus what _type_ of tokens we should recommend to the user, and in what _scope_ we should select these
tokens from.

## Cursor positioning
We need to understand what type of completions to recommend to the user. If they have code like this:
```systemverilog
    always_comb begin
        // CURSOR HERE
    end
```
then we probably want to recommend "variables" that exist in the parent module (ports, logics, etc).

To do this we'll need to store the SvParser parse tree.
TODO: figure out how to store the parse tree as an opaque or generic type to support multiple backends.

We will visit the parse tree from the top down, and find the deepest nested node that we understand and that
contains our cursor position. Nodes that we understand will be determined by `completion.rs::get_node_type`
_not_ returning `TokenType::NotInterested`.
TODO can we integrate this into SvDocument directly? -> I suspect _not really_ because SvDocument is too broad
TODO this is only going to work if the document always parses, which SvParser will not do.

Now that we know the position of the cursor in a part of the document we understand, we'll have the following
decision tree to figure out what to recommend:

- Did we just type "`"?
    - Recommend: macros. Done.
- Are we inside a module instantiation? (e.g. `mymodule test(/* CURSOR */)`)
    - Recommend: ports that belong to that module. Done.
- Are we inside a module?
    - Recommend: ports and logics that belong to that module. Done.
- Otherwise:
    - We must be top-level.
    - Recommend: Keywords (`module`, `typedef`, `class`, etc.). Done.

## Alternative really hacky approach
- Figure out if we are top level or not. Should be doable even with an ancient copy of the SvParser tree.
- If we are top-level:
    - Suggest only module names
- If we are not top-level:
    - Suggest ports, logics of the module we are inside
