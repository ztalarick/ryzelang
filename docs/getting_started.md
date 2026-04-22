# Getting Started with Ryzelang

Welcome, student of the Runes. This guide will help you set up your environment and write your first Ryzelang program.

## Prerequisites

Before setting up Ryzelang, ensure you have the following tools installed:

- **Rust & Cargo:** Required for the interpreter (`ryze`) and the Language Server.
  - [Install Rust](https://www.rust-lang.org/tools/install)
- **Node.js & npm:** Required for the VS Code extension and Tree-sitter tools.
  - [Install Node.js](https://nodejs.org/)
- **Tree-sitter CLI:** Required only if you intend to modify the language grammar (`grammar.js`).
  - Install via npm: `npm install -g tree-sitter-cli`
- **C Compiler (gcc or clang):** Required to compile the Tree-sitter parser during the Rust build process.

## Installation

### Global Installation (Recommended)
To use the `ryze` command from anywhere on your system, install it via Cargo:

```bash
# Run this from the project root
cargo install --path crates/ryzelang-cli
```

### Local Build
If you prefer not to install it globally, you can build it locally:

```bash
cargo build --release
```
The binary will be located at `./target/release/ryze`.

## Your First Script

Ryzelang scripts use the `.ryze` extension. Create a file named `hello.ryze`:

```ryze
// Push numbers to the stack
Q Q QQ // 1 + 1 = 2
Q Q QQ // 1 + 1 = 2
QQ // 2 + 2 = 4
Q QQ // 1 + 4 = 5

// Print the top value as an integer
EWQ
```

## Running the Interpreter

Run your script using the `ryze` command:

```bash
ryze hello.ryze
```

### Debugging
To see the state of your **Rune** and **Scroll** stacks after execution, use the `--debug` flag:

```bash
ryze --debug hello.ryze
```

## Next Steps
- Explore the [Spell Reference](reference.md) for a quick lookup of all built-in commands.
- Read the [Language Specification](language_specification.md) for a deep dive into the rules.
- Learn about the [Architecture](architecture.md) to understand the dual-stack model.
- Check out the `examples/` directory for more complex programs.
