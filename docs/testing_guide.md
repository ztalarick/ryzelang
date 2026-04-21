# Testing Guide

Ryzelang uses an **Expectation-Based Testing** system to verify the correctness of the interpreter, built-in spells, and custom combos.

## How it Works

The test runner iterates through all `.ryze` files in the `tests/` directory. For each file:
1. It executes the script using the Ryzelang CLI with the `--debug` flag.
2. It captures the entire output, including the final state of the **Rune Stack** and **Scroll Stack**.
3. It compares this output against a corresponding `.exp` (expectation) file.

## Running Tests

From the project root, execute the test runner script:

```bash
./bin/run_tests.sh
```

### Options
- `--bless` (or `-u`): Updates the `.exp` files with the current output of the interpreter. Use this when you have intentionally changed language behavior or added a new test file.

## Adding a New Test

1. Create a new `.ryze` file in the `tests/` directory (e.g., `tests/my_feature.ryze`).
2. Write your Ryzelang code. Use comments (`//`) to label sections.
3. Run the test runner with the bless flag to generate the initial expectation:
   ```bash
   ./bin/run_tests.sh --bless
   ```
4. Verify that the generated `tests/my_feature.ryze.exp` contains the stack states you expect.
5. Commit both the `.ryze` and `.exp` files.

## Testing for Errors

To test that the interpreter correctly rejects invalid code:
1. Create a test file that contains the invalid syntax or runtime error (e.g., `tests/invalid_logic.ryze`).
2. Run `./bin/run_tests.sh --bless`.
3. The `.exp` file will capture the `Runtime Error` or `Syntax Error` message. The test will "pass" in the future as long as the interpreter continues to produce that exact error.

## Best Practices
- **Isolation:** Keep tests focused on a single feature (e.g., one file for math, one for stack ops).
- **Comments:** Use comments in your `.ryze` files to make the expectations more readable.
- **Cleanup:** Ensure your tests don't leave the stack in an unpredictable state if they are part of a larger sequence, though each `.ryze` file runs in a fresh interpreter instance.
