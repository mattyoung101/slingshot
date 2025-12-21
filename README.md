# Slingshot - SystemVerilog LSP
> [!IMPORTANT]
> **This is the new rewrite, in C++ using Slang. The old version of Slingshot is on the "legacy" branch.**

**Slingshot** is a language server for the **SystemVerilog** hardware description language, with
a focus on accurate multi-file completion. The overarching goal is to make SystemVerilog
as intuitive to edit as C++ or Rust.

Compared to other SV LSPs, the main feature that Slingshot brings to the table is a completion-first approach,
using the powerful [Slang](https://github.com/MikePopoloski/slang) frontend. The intent is to provide fast,
accurate and robust completion even in complex projects. The trade-off is this does mean that features such as
"go-to-reference" take somewhat of a backseat.

Slingshot is (now) written in C++23. Previously, it was written in Kotlin and used ANTLR. This new rewrite
aims to:
- Improve speed and utility by using a proper SystemVerilog parser (Slang)
- Reduce memory usage

## Features
- Diagnostics
  - Based on the powerful Slang SystemVerilog frontend
- Advanced completion system
    - Context-sensitive completion, based off the Slang parse tree, a "best-effort" approach to only recommend
      valid completions
    - Slang's parser is highly error resistant, making it excellent for this use case
- Multi-threaded, project-wide index and compilation system
- Simple configuration
  - Slingshot is configured through a simple `.slingshot.toml` file declared in the project's root
  directory
  - This format is documented in [docs/config.md](docs/config.md)
- Compatible with Ubuntu 22.04+ and similar Linux distributions

Future features are planned on the [issue tracker](https://github.com/mattyoung101/slingshot/issues).

### Current state
Slingshot is being rewritten, but _may_ be useful in some simple projects at the moment. Please keep me
posted!

### Timeline
Slingshot is being developed in my free time, and I'm hoping to work on it somewhat during my PhD. One day, it
might become a joint project of my PhD lab, hopefully.

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
Slingshot issues can be diagnosed by reading the log file. This is located in
`~/.local/share/slingshot/slingshot-$PID.log`, where `$PID` is the Slingshot process ID. You can
just read the most recent log file. Currently, the 5 most recent log files are retained in that
directory. The `lnav` tool is very useful for reading Slingshot log files.

If there are errors about missing includes or not being able to find certain files, please make
sure you have read [docs/config.md](docs/config.md) and created your `.slingshot.toml` file. Then,
read the log to make sure that Slingshot has the correct root directory. The best way to ensure this
is invoking Neovim by typing `nvim .` in the project's root directory - don't edit individual files.

If the above steps do not resolve your issue, please open a bug ticket in the GitHub issue tracker.
You must include your log file, detailed description of the issue, and also a SystemVerilog code
example if possible. As my time is extremely limited, I may not be able to respond to or
fix bug tickets. Pull requests are welcome as well, but may take some time to review.

## Design goals
See [docs/design_goals.md](docs/design_goals.md)

## Implementation details
See [docs/impl_details.md](docs/impl_details.md)

## No AI policy
I do not use any LLMs when developing this project, and I will not accept contributions, issues or code that
are written in whole or in part using LLMs. These tickets will be closed on sight.

## Licence
Copyright (c) 2023-2025 M. L. Young. Available under the Mozilla Public License v2.0

> This Source Code Form is subject to the terms of the Mozilla Public
> License, v. 2.0. If a copy of the MPL was not distributed with this
> file, You can obtain one at https://mozilla.org/MPL/2.0/.
