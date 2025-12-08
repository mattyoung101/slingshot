# Remote debugger
Interacting with an LSP server can be quite difficult, because its standard in/out is usually gobbled by the
editor. Slingshot has log files, but these are not interactive.

To solve this problem, Slingshot includes a remote debugger. This opens a simple UDP socket on port 4269 that
can be sent commands.

To interact with this, use the script in `./scripts/remote_debugger.py`.

## Command listing
- `dump index`: Dumps the contents of the index
- `sigtrap`: Immediately raises a SIGTRAP
- `die`: Causes the server to exit
- `compiler stats`: Gets timing statistics about the compiler

FIXME: this needs to be put back to TCP actually and null-terminated
