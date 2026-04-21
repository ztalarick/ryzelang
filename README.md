# Ryzelang

Ryzelang is an esoteric, Turing-complete, functional stack-based programming language modeled after the gameplay mechanics of the League of Legends champion, Ryze.

Programs are constructed by chaining "Combos" (primitives and macros/functions). The language uses a dual-stack architecture to separate data (**Runes**) from behavior (**Scrolls**).

## Core Concepts

- **The Rune Stack:** A LIFO stack for numerical data and math.
- **The Scroll Stack:** A separate stack for code blocks (quotations).
- **The Scroll (Env):** A global dictionary mapping Combo names (strictly `Q`, `W`, `E`) to blocks.
- **Built-in Combos:** Primitives like `QQ` (Add), `WWW` (Sleep), and `EE/EQ` (Flux/Safe Unwrap).

## Project Structure

```text
ryzelang/
├── crates/
│   ├── ryzelang-core/    # The interpreter logic and dual-stack VM
│   ├── ryzelang-cli/     # The command-line interface
│   ├── ryzelang-ls/      # Language Server Protocol (LSP) implementation
│   └── tree-sitter-ryze/ # Rust bindings for the grammar
├── tree-sitter-ryze/     # The formal grammar source (JS/C)
├── vscode-ryzelang/      # VS Code extension (Highlighting + LSP Client)
├── zed-ryzelang/         # Zed editor extension
└── examples/             # Sample .ryze scripts (Hello World, Countdown)
```

## Installation

To install the Ryzelang interpreter (`ryze`) globally on your system:

```bash
cargo install --path crates/ryzelang-cli
```

Once installed, you can run Ryzelang scripts from anywhere:
```bash
ryze examples/hello_world.ryze
```

## Quick Start

### 1. Build the Project
```bash
cargo build --release
```

### 2. Run a Script
```bash
./target/release/ryze examples/hello_world.ryze
```

### 3. Debugging
Use the `--debug` flag to see the final state of both stacks:
```bash
./target/release/ryze --debug examples/countdown.ryze
```

## Language Reference

See the [docs/](docs/) directory for detailed information:
- [Getting Started Guide](docs/getting_started.md)
- [Spell Reference](docs/reference.md)
- [Architecture Overview](docs/architecture.md)
- [Built-in Combos Reference](docs/language_specification.md)
- [Formal Grammar](docs/formal_grammar.md)
- [Language Server Protocol](docs/language_server.md)
- [Testing Guide](docs/testing_guide.md)

## Testing
Run the expectation-based test suite:
```bash
./bin/run_tests.sh
```

## Editor Support

- [VS Code Extension](vscode-ryzelang/README.md): Syntax highlighting, completions, and hovers.

## License

This project is licensed under the MIT License - see the [LICENSE.MD](LICENSE.MD) file for details.
