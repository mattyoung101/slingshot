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

Slingshot is (now) written in C++20. Previously, it was written in Kotlin and used ANTLR. This new rewrite
aims to:
- Improve speed and utility by using a proper SystemVerilog parser (Slang)
- Reduce memory usage

## Features
**WORK IN PROGRESS**

- Diagnostics
- Completion
  - Context-sensitive completion based on cursor's position in the parse tree
  - Slingshot is aware of both line and block comments, and does not suggest completions when you are inside
    a comment
  - Completion for "variables" (logic, wire, etc) and ports in the current module
  - Completion for language keywords, e.g. `always_ff`, `always_comb`, `posedge`
  - Snippets for various blocks, e.g. `always_ff`, `always_comb`, `case`, `if`
  - Completion for macros
- Cross-file completion
  - Slingshot can complete modules, enums and macros declared in other files
  - Every include path specified in the config file is searched and indexed automatically
- Simple configuration
  - Slingshot is configured through a simple `.slingshot.toml` file declared in the project's root
  directory
  - This format is documented in [docs/config.md](docs/config.md)

Future features are planned on the [issue tracker](https://github.com/mattyoung101/slingshot/issues).

### Current state
Slingshot is being rewritten and is not currently useful in any projects.

### Timeline
Slingshot is being developed in my free time, and I'm hoping to work on it somewhat during my PhD.

## Building and running
### Building and environment
**TODO**

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

## Licence
Copyright (c) 2023-2025 M. L. Young. Available under the Mozilla Public License v2.0

> This Source Code Form is subject to the terms of the Mozilla Public
> License, v. 2.0. If a copy of the MPL was not distributed with this
> file, You can obtain one at https://mozilla.org/MPL/2.0/.
