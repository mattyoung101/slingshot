# Slingshot - SystemVerilog LSP
**Slingshot** is a language server for the **SystemVerilog** hardware description language, with
a focus on accurate multi-file completion. The overarching goal is to make SystemVerilog 
as intuitive to edit as C++ or Python.

Compared to other SV LSPs, the main feature that Slingshot brings to the table is a powerful completion 
system, backed by ANTLR, that supports multi-file projects through automatic indexing. Slingshot also
supports instant linting, backed by Verilator.

Slingshot is written in Kotlin and runs on a Java 17 JVM or higher.

## Features
- Diagnostics (powered by Verilator)
  - Fully in-memory, does not write any temporary files to disk
- Completion (powered by ANTLR)
  - Context-sensitive completion based on cursor's position in the ANTLR parse tree 
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
  - Slingshot is configured through a simple `.slingshot.yaml` file declared in the project's root
  directory
  - This format is documented in [docs/config.md](docs/config.md)
  
Future features are planned on the [issue tracker](https://github.com/mattyoung101/slingshot/issues).

### Current state
Slingshot is very close to a 1.0.0 release, with a few remaining tasks to do. I'm
working on adding more completion items and testing the server on larger projects. It's probably useful in
moderately complicated SystemVerilog projects right now - feel free to give it a spin and report
any issues you encounter!

### Timeline
Slingshot is being actively developed for my processor design thesis project (expected to start midway
through 2024). My aim is to get a 1.0.0 release out before the start of the thesis, and then provide
light maintenance and improvements over the next 1 year while I complete my thesis.

## Building and running
### Building and environment
**Important:** Slingshot currently only runs on Linux and other *nix systems. Windows is not 
supported, and no support is currently planned (unless specifically requested).

You need Java 17 or higher. 

The program can be built with `./gradlew build`, which also generates JAR files in the `build/libs` directory.

Because the SystemVerilog generated parser is so massive, you will need to modify IntelliJ's max file parse
size to be larger. Go to Help -> Edit Custom Properties and insert `idea.max.intellisense.filesize=999999`.
Then, restart the IDE.

You also need to download and install Verilator using your system's package manager.

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

This is the setup I use for development as well.

**Important:** Please read [docs/config.md](docs/config.md) for instructions on how to create a
`.slingshot.yaml` file to configure the server. This is mandatory for multi-file (read: most) projects.

### Troubleshooting
Slingshot issues can be diagnosed by reading the log file. This is located in 
`~/.local/share/slingshot/slingshot.$PID.log`, where `$PID` is the Slingshot process ID. You can
just read the most recent log file. Currently, the 5 most recent log files are retained in that
directory. The `lnav` tool is very useful for reading Slingshot log files.

If Verilator syntax checking is not working, please make sure Verilator is installed and is in your
$PATH. The log file will also contain the exact cause of why the Verilator didn't work.

If there are errors about missing includes or not being able to find certain files, please make
sure you have read [docs/config.md](docs/config.md) and created your `.slingshot.yaml` file. Then,
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
Copyright (c) 2023, 2024 Matt Young. Available under the Mozilla Public License v2.0

> This Source Code Form is subject to the terms of the Mozilla Public
> License, v. 2.0. If a copy of the MPL was not distributed with this
> file, You can obtain one at https://mozilla.org/MPL/2.0/.
