# Virtual Dice

A simple command-line virtual dice roller written in Rust.

## Features

- Roll a die with any number of sides (default is 6).
- If rolling a standard six-sided die (d6), shows a nice ASCII-art representation of the result.
- Uses the `rand` crate for random number generation.

## Usage

1. **Clone the repository:**
   ```bash
   git clone https://github.com/pixxechan/Virtual_dice.git
   cd Virtual_dice
   ```

2. **Add dependency:**
   Make sure your `Cargo.toml` includes:
   ```
   [dependencies]
   rand = "0.8"
   ```

3. **Run the program:**
   ```bash
   cargo run --release -- [number_of_sides]
   ```
   - Replace `[number_of_sides]` with the number of sides you want on your die (e.g. `6` for d6, `20` for d20).
   - If no argument is provided, defaults to a standard six-sided die.

## Example

Roll a d6 (six-sided die):
```bash
cargo run --release
```
Output:
```
┌───┐
│ ● │
│   │
│ ● │
└───┘
```

Roll a d20 (twenty-sided die):
```bash
cargo run --release -- 20
```
Output:
```
🎲 d20: 13
```

## File Structure

- `Dice.rs` — Main source file containing all logic

## License

MIT License

---
Created by [pixxechan](https://github.com/pixxechan)
