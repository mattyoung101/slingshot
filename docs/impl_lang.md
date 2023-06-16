# C++?
A quick note on why we're going with C++. This is purely out of practical reasons. Rust has much better LSP
libraries. Unfortunately, while [sv-parser](https://github.com/dalance/sv-parser) looks cool, it's my understanding
that it's _not_ as accurate as slang (todo verify this). In addition, slang (and possibly the addition of verible) 
may provide good enough diagnostics that calling Verilator may not be required, which would improve performance.

---

**FIXME: Slang's diagnostics aren't _that_ good anyway (compared to verilator). It might be worth doing rust.**

**Reasons for Rust**
- Better LSP libraries (tower-lsp)
- Get to learn a new language
- While people complain about CMake a lot, and in my opinion it's not _thaaat_ bad, Cargo does seem better

**Reasons for C++**
- Get to use Slang (probably better than sv-parser)
- Already know C++ and CMake
- Get to practice with modern C++20 or higher (most C++ code I've written has been shitty ROS C++14)

---

As it turns out, diagnostics are not going to be the issue.

**KEY PROBLEM:** sv-parser is not robust against errors. We cannot easily use it for completion, because it will
refuse documents as they're being typed. So we should use C++ right? Maybe, except: 
1. I want to learn Rust,
2. C++ LSP libraries are shit compared to tower-lsp, so we'd need to spend time grinding that out first, which
I'm really _NOT_ keen to spend my time on these holidays. I already deal with enough C++ at work for it to be
detrimental to my overall health and self-esteem.
3. I'm keen to use https://docs.rs/trie-rs/latest/trie_rs/ for prefix trees for typing suggestions

So, I think instead we actually do the insane thing and write a Rust binding for slang. This could be a relatively
dogshit binding, even something despicable as writing C++ code that parses a string and returns the resulting
concrete syntax tree as a FlatBuffer, or even (lord forbid), JSON.

Our best approach would probably be to use cxx.rs and write a C++ routine that uses slang to parse the 
specified file into a CST, then extract completion tokens and their associated scopes as well as the current
cursor position, and return this all in a simple struct that Rust can understand.

Update: This is proving really complicated for some reason (mostly trying to integrate CMake with Cargo), so
we might have to give up on Rust entirely :(
