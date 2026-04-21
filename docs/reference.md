# Ryzelang Spell Reference

This table provides a quick reference for all built-in **Spells** (primitives). 

| Spell | Name | Stack Effect (Rune) | Stack Effect (Scroll) | Description | Example |
| :--- | :--- | :--- | :--- | :--- | :--- |
| `Q` | **Overload** | `[] -> [1]` | - | Pushes the integer `1` to the Rune Stack. | `Q` -> `[1]` |
| `W` | **Rune Prison** | `[a] -> []` | - | Pops and discards the top value of the Rune Stack. | `Q W` -> `[]` |
| `E` | **Spell Flux** | `[a] -> [a, a]` | - | Duplicates the top value of the Rune Stack. | `Q E` -> `[1, 1]` |
| `EW` | **Flux Swap** | `[a, b] -> [b, a]` | - | Swaps the top two values of the Rune Stack. | `Q Q Q QQ EW` -> `[2, 1]` |
| `QQ` | **Add** | `[a, b] -> [a+b]` | - | Pops top two, pushes their sum. | `Q Q QQ` -> `[2]` |
| `QW` | **Subtract** | `[a, b] -> [a-b]` | - | Pops `b`, pops `a`, pushes `a - b`. | `Q Q Q QQ QW` -> `[1]` |
| `WE` | **Multiply** | `[a, b] -> [a*b]` | - | Pops top two, pushes their product. | `Q Q QQ E WE` -> `[4]` |
| `WQ` | **Divide** | `[a, b] -> [a/b]` | - | Pops `b`, pops `a`, pushes `a / b`. | `Q Q Q QQ Q Q QQ WQ` -> `[1]` |
| `EQ` | **Logic / Unwrap** | `[a] -> [0/1]` | - | If `0`, pushes `1`. Else `0`. Also unwraps Flux-Marked Runes. | `Q EQ` -> `[0]` |
| `WEQ` | **Conditional** | `[cond] -> []` | `[F, T] -> []` | Executes `T` if `cond > 0`, else executes `F`. | `Q [Q] [W] WEQ` -> Executes `Q` |
| `WWW` | **Stasis** | `[ms] -> []` | - | Pauses execution for `ms` milliseconds. | `Q Q Q QQ WWW` -> Sleep 2ms |
| `EE** | **Flux Mark** | `[a] -> [f(a)]` | - | Marks Rune with Flux. `0` becomes Empty, others become Charged. | `Q EE` -> `[Flux(1)]` |
| `EWQ` | **Print Int** | `[a] -> []` | - | Pops and prints top Rune as an integer. | `Q EWQ` -> Output: `1` |
| `EQE` | **Print Char** | `[a] -> []` | - | Pops and prints top Rune as an ASCII character. | `... EQE` -> Output: `A` |
| `EEW** | **Read Char** | `[] -> [a]` | - | Reads one ASCII character from stdin and pushes its value. | `EEW` -> `[65]` |
| `R` | **Realm Warp** | - | `[B] -> []` | Binds the top block `B` to the preceding combo name. | `[E QQ] QQE R` |

## Notation Guide
- **`[a, b]`**: Represents the stack where `b` is the top element.
- **`cond`**: A Rune value used as a boolean (Positive = True, Zero/Negative = False).
- **`f(a)`**: A Flux-Marked Rune. Use `EQ` to unwrap.

## Combo Definitions
To define a custom combo, push a block to the Scroll Stack and follow it with a name and the `R` spell:
```ryze
[ E QQ ] QQE R  // Binds the "Double" logic to QQE
Q Q Q QQE       // Pushes 1, 1, 1, then doubles the top to get [1, 1, 2]
```
