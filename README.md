# Slingshot - SystemVerilog LSP
**Slingshot** is a **work in progress** language server for the **SystemVerilog** hardware description language.
It also partially supports **Verilog**.

Slingshot is written in Kotlin and runs on a Java 17 compliant JVM or higher.

Although there are a few existing LSPs for SystemVerilog, I decided to create Slingshot specifically with a
focus on completion, which some existing SV LSPs are lacking. Slingshot's goal is to create the ultimate 
SystemVerilog LSP, with all the features you know and love from mature LSPs like clangd and pyright. This is
a pretty ambitious goal that I may not actually be able to achieve, but here's to trying!

### Current state
Slingshot is almost stable, but still a work in progress. Right now, it is capable of providing
Verilator linting and some completion. I'm working on adding more completion items and stabilising the
server. It's probably useful in _very simple_ SystemVerilog projects right now - feel free to give it a spin!

The big feature that remains to be implemented is multi-file support, so you can reference modules in other
files. Currently, Slingshot does not understand this, but works pretty well for single files.

### Timeline 
Slingshot is currently in active use by myself while working on my thesis project, which is
implemented entirely in SystemVerilog. So, rest assured, over the 1.5 years or so it *should* receive constant
improvements and updates.

During the university semester, my time is extremely limited, and I'm not really able to work on this program.
I have more time to work on it over the holidays and whenever my workload is light. All in all, I am hoping to 
get Slingshot fully functional by no later than June 2024, so I can use it to develop my thesis.

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
### Building and environment

**Important:** Slingshot currently only runs on Linux and other *nix systems. Windows is not 
currently supported, and no support is currently planned.

You need Java 17 or higher. 

The program can be built with `./gradlew build`, which also generates JAR files in the `build/libs` directory.

Because the SystemVerilog generated parser is so massive, you will need to modify IntelliJ's max file parse
size to be larger. Go to Help -> Edit Custom Properties and insert `idea.max.intellisense.filesize=999999`.
Then, restart the IDE.

### Running

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

**Important:** Please read [docs/config.md](docs/config.md) for instructions on how to create a
`.slingshot.yaml` file to configure the server. This is mandatory for multi-file (read: most) projects.

This is the setup I use for development as well.

### Troubleshooting

Slingshot issues can be diagnosed by reading the log file. This is located in 
`~/.local/share/slingshot/slingshot.$PID.log`, where `$PID` is the Slingshot process ID. You can
just read the most recent log file. Currently, the 5 most recent log files are retained in that
directory. The `lnav` tool is very useful for reading Slingshot log files.

If Verilator syntax checking is not working, please make sure Verilator is installed and is in your
$PATH. The log file will also contain the exact cause of why the Verilator didn't work.

If there are errors about missing includes or not being able to find certainn files, please make
sure you have read [docs/config.md](docs/config.md) and created your `.slingshot.yaml` file. Then,
read the log to make sure that Slingshot has the correct root directory. The best way to ensure this
is invoking Neovim by typing `nvim .` in the project's root directory - don't edit individual files.

If the above steps do not resolve your issue, please open a bug ticket in the GitHub issue tracker.
You must include your log file, detailed description of the issue, and also a SystemVerilog code
example if possible. As my time is extremely limited, I may not be able to respond to or 
fix most bug tickets. Pull requests are welcome as well, but may take some time to review.

## Design goals
See [docs/design_goals.md](docs/design_goals.md)

## Implementation details
See [docs/impl_details.md](docs/impl_details.md)

## Licence
Mozilla Public License v2.0
