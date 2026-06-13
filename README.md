# Slingshot - SystemVerilog LSP
> [!IMPORTANT]
> **This is BETA quality software; bugs may be present. Please file issues on the issue tracker.**

**Slingshot** is a language server for the **SystemVerilog** hardware description language, with a focus on
**stability**, **performance**, and **accuracy** in autocomplete. The overarching goal is to make
SystemVerilog as intuitive to edit as C++ or Rust.

Compared to other SV LSPs, the main feature that Slingshot brings to the table is a completion-first approach,
using the powerful [Slang](https://github.com/MikePopoloski/slang) frontend. The intent is to provide fast,
accurate and robust completion even in complex projects. Slingshot also features a relatively advanced
**asynchronous indexing system**, to power this completion system, that features graph-based dependency resolution.

The trade-off is this does mean that features such as "go-to-reference" take somewhat of a backseat; though
the plan is to implement them eventually.

Slingshot is (now) written in C++23. Previously, it was written in Kotlin and used ANTLR. This new rewrite
aims to:
- Improve speed and utility by using a proper SystemVerilog parser (Slang)
- Reduce memory usage

## Features
- Diagnostics
  - Based on the Slang SystemVerilog frontend, which is considered best in its class
- Completion system
    - Context-sensitive completion, based off the Slang parse tree, a "best-effort" approach to only recommend
      valid completions
- Advanced indexing system
    - Graph-based (backwards BFS) automatic dependency tracking between SV documents
        - Improves performance by only compiling the documents necessary to index a file
    - Asynchronous; does not block the main thread while indexing is active
- Project-wide Go to Definition support
  - _Brand new and highly experimental!_
- Simple configuration
  - Slingshot is configured through a simple `.slingshot.toml` file declared in the project's root
  directory
  - This format is documented in [docs/config.md](docs/config.md)
  - File discovery using directory walking or F-lists with automatic dependency resolution
  - "It Just Works!"
- Compatible with Ubuntu 22.04+ and similar Linux distributions

Future features are planned on the [issue tracker](https://github.com/mlyoung101/slingshot/issues).

### Current state
Slingshot is relatively stable and is ready for testing in larger projects. I'm actively using it myself daily
to develop my PhD dissertation. Please keep me posted!

### Timeline
Slingshot is being developed in my free time during my PhD. I do not _always_ have time to work on this
project, but I try my best.

### Known supported editors
- Neovim v0.11+
- Helix

## Building and running
### Building and environment
You will need:
- CMake 3.21+
- A C++23 compiler (Clang 20+ recommended)
- Ninja
- ccache (optional)
- Just (optional)
- mold (optional)
- Something compatible with Ubuntu 22.04+ (I develop on Arch, btw)

The simplest way to build, with Just, is to run `just build` and/or `just build_debug`.

If that doesn't work, you can do it yourself:

```bash
# remove references to ccache if you don't have it
cmake -B build -G Ninja -DCMAKE_BUILD_TYPE=Release -DCMAKE_EXPORT_COMPILE_COMMANDS=1 \
    -DCMAKE_CXX_COMPILER_LAUNCHER=ccache -DCMAKE_C_COMPILER_LAUNCHER=ccache
cd build
ninja
```

C++ dependency management is a fucking disaster, as is the language itself, as I have complained extensively
about [1](https://tech.lgbt/@mlyoung/115604990686028947), [2](https://tech.lgbt/@mlyoung/115605004052848993),
[3](https://tech.lgbt/@mlyoung/115605019086603912). This project uses the CMake CPM package manager, which is
basically a thin wrapper around `FetchContent`.

We vendor _all_ our dependencies. This should mean, by the grace of god, it'll compile on your system. The
price to pay is that you have to compile _all_ your deps from scratch each time, sorry. Use ccache.

### Running
Currently, I have only tested Slingshot in Neovim.

When Slingshot is a more capable LSP, it will (hopefully) be available in upstream LSP projects like
[mason.nvim](https://github.com/williamboman/mason.nvim) and [nvim-lspconfig](https://github.com/neovim/nvim-lspconfig);
although both of these projects are surprisingly quite restrictive about what LSPs they allow, and it's
possible Slingshot may never make the cut.

Until then, you can manually add Slingshot as a nvim-lspconfig server by inserting the following into `init.lua`:

```lua
vim.lsp.config("sv-slingshot", {
    cmd = { "<PATH_TO_SLINGSHOT_ROOT>/build/slingshot" },
    root_markers = { ".git", ".slingshot.toml" },
    filetypes = {
        "systemverilog",
        "verilog",
    },
})

vim.lsp.enable("sv-slingshot")
```

This is the setup I use for development as well.

**Important:** Please read [docs/config.md](docs/config.md) for instructions on how to create a
`.slingshot.toml` file to configure the server. This is mandatory for multi-file (read: most) projects.

### Troubleshooting
Slingshot issues can be diagnosed by reading the LSP log file. Slingshot prints to stderr, this will then
usually be saved by your editor somewhere; please refer to editor specific docs. For Neovim, it will be in
`~/.local/state/nvim/lsp.log`

To see more verbose debug messages, set the environment variable `SLING_LOG_DEBUG` to any value. To see _even
more_ verbose messages (spew), set the environment variable `SLING_LOG_TRACE` to any value.

If the LSP does nothing at all, please make sure you have read [docs/config.md](docs/config.md) and created
your `.slingshot.toml` file. Then, read the log to make sure that Slingshot has the correct root directory.
The best way to ensure this is invoking Neovim by typing `nvim .` in the project's root directory - don't edit
individual files.

If the above steps do not resolve your issue, please open a bug ticket in the GitHub issue tracker.
You must include your log file, detailed description of the issue, and also a SystemVerilog code
example if possible. As my time is extremely limited, I may not be able to respond to or
fix bug tickets. Pull requests are welcome as well, but may take some time to review.

## Design philosophy and goals
Slingshot's primary goals across the board are **stability**, **performance** and **accuracy**. Stability is
always difficult in an unsafe language like C++, but we try as much as we can, including through the use of
ASan and UBSan.

For its feature set, Slingshot prioritises these features and all the infrastructure necessary to make them
stable and performant:
- Autocompletion
- Diagnostics (error checking)
- Niceties (e.g. progress bars and simple configuration)

This is a pretty small set of features, but is the minimum set to have a reasonably nice editing experience.
Once those features are stabilised, I plan to eventually move out into more advanced features such as
go-to-definition, but importantly _only after_ the core features are sufficiently stable.

## Implementation details
See [docs/impl_details.md](docs/impl_details.md)

## Contributing guidelines
### No AI policy
As part of Slingshot's design philosophy, as well as the tenuous situation around the use of LLMs regarding
copyright and even ethics, I will **not** accept any pull requests or issues that are written in whole or in
part using LLMs. I also do not use any LLMs in any way when working on Slingshot. "Vibe coded" LLM
contributions tend to create an enormous maintenance burden that would be unacceptable for me as the sole
maintainer and editor of this project.

In addition, there are _already_ at least two SV LSPs developed with significant AI assistance, so if you
prefer this model, use them instead.

> [!IMPORTANT]
> Submitting a PR or issue that is reasonably believed to be written using AI will result in it being
> closed in almost all circumstances, and you will be referred back to this policy.

To be clear, here are some examples of this policy in effect:
- A user submits an issue or PR that has obvious hallmarks of AI writing and exaggeration: **Reject, refer to
  policy.**
- A user submits a PR that includes "Co-authored-by: Claude". **Reject without exception, refer to policy.**
- A user submits a PR that masquerades as human-authored, but is suspicious. **Clarify with user, refer to
  policy; reject if user admits AI, has made other AI PRs, or is otherwise evasive.**
- A user authors a PR using auto-complete style AI tools, but otherwise developed it manually. **Approve, as
  long as style is acceptable and user understands their contribution clearly.**
- A user employs an LLM-based translation tool to translate their otherwise hand-written issue. **Approve, as
  long as LLM has not modified the original intent significantly.**

This is based on precedent from [Zig](https://ziglang.org/code-of-conduct/#strict-no-llm-no-ai-policy),
[QEMU](https://www.qemu.org/docs/master/devel/code-provenance.html#use-of-ai-generated-content),
[LibreOffice](https://wiki.documentfoundation.org/Development/AI_policy), and others.

## Licence
Copyright (c) 2023-2026 Mel Young <mel@mlyoung.cool>. Available under the Mozilla Public License v2.0.

> This Source Code Form is subject to the terms of the Mozilla Public
> License, v. 2.0. If a copy of the MPL was not distributed with this
> file, You can obtain one at https://mozilla.org/MPL/2.0/.

## Third party libraries
Slingshot uses the following 3rd party libraries:

- **spdlog**: MIT licence
- **fmt**: MIT licence
- **Slang**: MIT licence
- **ankerl::unordered_dense**: MIT licence
- **nlohmann::json:** MIT licence
- **lsp-framework:** MIT licence
- **TOML++:** MIT licence
- **sockpp:** BSD 3-Clause licence
- **Graaf:** MIT licence
- **moodycamel::ConcurrentQueue:** BSD 3-Clause licence/BSL 1.0

