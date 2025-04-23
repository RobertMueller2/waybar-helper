# waybar-helper

A small Rust program for Waybar helpers, shared for reference purposes. It currently has one subcommand: wayeyes, which indicates via swayipc-async whether the focused node has xdgshell, xwaylandshell or something else.

Config example is included in my [Dotfile-Snippets](https://github.com/RobertMueller2/Dotfile-Snippets) repository.

## Building

From the repository root:

```sh
# Build all crates
cargo build

# Run the binary (e.g. the 'wayeyes' subcommand)
cargo run -p waybar-helper -- wayeyes

# install the binary in $HOME/.cargo/bin
cargo install -p waybar-helper-- wayeyes
```
