# Slingshot - SystemVerilog LSP
**Slingshot** is a **work in progress** language server for the **SystemVerilog** hardware description language.
It also partially supports Verilog.

Slingshot is written in Kotlin and runs on a Java 17 compliant JVM or higher.

Although there are a few existing LSPs for SystemVerilog, I decided to create Slingshot specifically with a
focus on completion, which some existing SV LSPs are lacking. Slingshot's goal is to create the ultimate 
SystemVerilog LSP, with all the features you know and love from mature LSPs like clangd and pyright. This is
a pretty ambitious goal that I may not actually be able to achieve, but here's to trying!

**Current state:** Slingshot is almost stable, but still a work in progress. Right now, it is capable of providing
Verilator linting and _some_ completion. I'm working on adding more completion items and stabilising the
server. It's probably useful in _very simple_ SystemVerilog projects right now.

**Timeline:** Due to university, my time is extremely limited. However, I am hoping to get Slingshot fully
functional by no later than June 2024, so I can use it to develop the SV code for my thesis.

**Author:** Matt Young (m.young2@uqconnect.edu.au)

## Currently implemented features
This is the list of _currently_ implemented features. See below for the full list of design goals and planned
features.

- Diagnostics (powered by Verilator)
  - Fully in-memory, does not write any temp files to reduce disk thrashing
- Completion (powered by ANTLR)
  - Context-sensitive completion based on cursor's position in the ANTLR parse tree 
  - Slingshot is aware of both line and block comments, and does not suggest completions when you are inside
    a comment
  - Completion for "variables" (logic, wire, etc) and ports in the current module
  - Completion for various keywords, e.g. `always_ff`, `always_comb`, `posedge` (snippet support to auto-complete the entire block TBA)
  - Completion for macros

## Building and running
**Toolchain**

You will need:
- Java 17 or higher

**Building Slingshot**

The program can be built with `./gradlew build`, which also generates JAR files in the `build/libs` directory.

Because the SystemVerilog generated parser is so massive, you will need to modify IntelliJ's max file parse
size to be larger. Go to Help -> Edit Custom Properties and insert `idea.max.intellisense.filesize=999999`.
Then, restart the IDE.

**Running**

Currently, I have only tested Slingshot in Neovim.

When Slingshot is a more capable LSP, it will (hopefully) be available in upstream LSP projects like
[mason.nvim](https://github.com/williamboman/mason.nvim) and [nvim-lspconfig](https://github.com/neovim/nvim-lspconfig).

Until then, you can manually add Slingshot as a nvim-lspconfig server by inserting the following into `init.lua`:

```lua
local lspconfig = require 'lspconfig'
local configs = require 'lspconfig.configs'

if not configs.slingshot then
  -- this require lspconfig.configs is the trick required to make it work
  require("lspconfig.configs").slingshot = {
    default_config = {
      cmd = {'java', '-jar', '<PATH_TO_SLINGSHOT>/slingshot/build/libs/slingshot-1.0-SNAPSHOT-all.jar'};
      filetypes = {'verilog', 'systemverilog'};
      root_dir = function(fname)
        return lspconfig.util.find_git_ancestor(fname) or vim.loop.os_homedir()
      end;
      settings = {};
    };
  }
end

lspconfig.slingshot.setup{}
```

This is the setup I use for development as well.

## Design goals
See [docs/design_goals.md](docs/design_goals.md)

## Implementation details
See [docs/impl_details.md](docs/impl_details.md)

## Licence
Mozilla Public License v2.0
