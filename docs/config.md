# Slingshot configuration

## .slingshot.toml
Slingshot configuration is loaded from a file canonically called `.slingshot.toml`
(also acceptably `.slingshot.yml`) in the root directory of the project. The project root directory
is usually the Git root directory, so the top of your Git repo. However, this is decided by the
LSP client, so it depends on how that is configured.

The current `.slingshot.toml` format looks as follows:

```toml
# Slingshot SystemVerilog language server configuration file
# https://github.com/mlyoung101/slingshot

# Config file version
version = "1.2.0"

# List of include directories, relative to project root, to search for SystemVerilog files.
# Note: The project root directory is usually the Git root directory.
include_dirs = [
  "rtl"
  # ... any other RTL source directories here ...
]

# List of F-list files (*.f) to pull dependencies from.
# Paths are relative to the project root as well.
flist_files = [
  "my_project.f"
]

# List of manual definitions
[defines]
IS_FOO = "bar"
```

If Slingshot is not behaving correctly, see the troubleshooting steps in the main README. I expect
that a common cause of problems will be that the server can't find the config TOML, or that it is
not processing the `include_dirs` correctly. Always check the log file (see readme), which contains
a large amount of information.

## Config file version history
### v1.1.0
- Support f-list files (*.f)

### v1.0.0
- Initial release
