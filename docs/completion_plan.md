# Completion plan
How are we going to implement completion?

## General idea
- First, we will index the entire file, and attempt to discover any referenced modules as well
    - This will happen on each file load
    - Optimisation: Unless the file has not been changed since the last time Slingshot scanned it (CRC32?)
- The LSP will then build a list of symbols it believes would be valid for completion _at some point_
in the current file - and associated info about what scope they apply to
    - For example, macros would apply everywhere, but some `logic` elements might only apply in certain scopes
- As the user is typing, we will query the token list and filter for tokens that would be valid completion
targets for the scope they are typing in
    - We can just send these straight off to the editor, which will then do further filtering

Things left to consider:
- When do we update the token index? If a user instantiates a new module, for example, we need to parse that
    - Ideally, if we can access this from the LSP client, after time the user is "done typing", we refresh
    the entire index (go back to step 1)
- We should consider persisting the index to a database on disk
    - Probably just cereal and maybe zstd if it's huge
- When the user is typing, the syntax tree may not be valid, which may cause parser errors
    - I think this _won't_ happen because we basically do all our parsing _before_ they type and only suggest
    stuff that we've parsed from a previously valid syntax tree
    - Worth noting Veridian had this issue though: https://github.com/vivekmalneedi/veridian/pull/176#discussion_r1122616240


## Technical implementation
- We should use a trie data structure for suggestions, as this realistically makes a lot of sense and there
are libraries to make it easy
