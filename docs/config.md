# Slingshot configuration

## .slingshot.yaml
Slingshot configuration is loaded from a file called `.slingshot.yaml` (or `.slingshot.yml`) in the
root directory of the project. The project root directory is usually the Git root directory, so
the top of your Git repo. However, this is decided by the LSP client, so it depends on how that is
configured.

The current `.slingshot.yaml` format looks as follows:

```yaml
# Slingshot SystemVerilog language server configuration file
# https://github.com/mattyoung101/slingshot

# Config file version
version: "1.0.0"

# List of glob patterns, relative to the project root directory, to search for SystemVerilog files.
# Note: The project root directory is usually the Git root directory.
# For example, in the following tree:
# - myrepo
#   \ rtl
#       \ myfile.sv
#       \ include.sv
#   \ docs
#       \ markdown.md
# You would want a glob pattern of "rtl/*.sv"
# This option is used to configure Verilator linting, to resolve `include directives, and to index
# multi-file projects, so it's very important that this is correct and kept up to date.
globs:
  - "rtl/*.sv"
  - "rtl/*.svh"
```

## Config file version history
### v1.0.0
- Initial release