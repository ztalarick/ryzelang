# Ryzelang VS Code Extension

This extension provides syntax highlighting and Language Server Protocol (LSP) support for the **Ryzelang** esoteric programming language.

## Features

- **Syntax Highlighting:** High-performance highlighting for Spells (`QWE`), Blocks (`[]`), and the Memorization operator (`R`).
- **Hover Support:** Hover over built-in spells to see their effect on the Rune and Scroll stacks.
- **Completion:** Autocomplete for all base kit primitives.
- **Diagnostics:** Live syntax error reporting.
- **Go-to-Definition:** Warp instantly to where a custom combo was defined with `R`.

## Setup (Local Development)

Since this extension is for a custom local language, you need to set up the Language Server manually.

### 1. Prerequisites

- **Node.js & npm:** To compile the extension client.
- **Rust & Cargo:** To build the Ryzelang Language Server.
- **VS Code:** Obviously!

### 2. Build the Language Server

From the root of the Ryzelang workspace, build the LSP binary:

```bash
cargo build --package ryzelang-ls
```

The extension is pre-configured to automatically find the server binary in the project's `target/debug` directory.

### 3. Install and Compile

Navigate to this directory (`vscode-ryzelang`) and run:

```bash
npm install
npm run compile
```

### 4. Link to VS Code

To install the extension locally, copy or link this folder to your VS Code extensions directory:

**macOS / Linux:**
```bash
ln -s $(pwd) ~/.vscode/extensions/ryze
```

**Windows (PowerShell):**
```powershell
New-Item -ItemType SymbolinkLink -Path "$HOME\.vscode\extensions\ryze" -Target (Get-Location)
```

### 5. Restart VS Code

Restart VS Code or run the `Developer: Reload Window` command. Open any `.ryze` file to start coding.

## License

This project is licensed under the MIT License - see the [LICENSE.MD](../LICENSE.MD) file for details.
