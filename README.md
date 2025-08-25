# Rust Test Practice

This repository contains a set of small Rust coding exercises (called *questions* in the code).
Each function demonstrates a different concept in Rust: loops, strings, tuples, slices, mutable
references, and user input.

> Note: In `main`, multiple questions are called. To avoid unnecessary I/O or panics while
> experimenting, comment out the ones you’re not currently working on.

---

## Questions Overview

### Question 1 – Sum of Squares
- **Function:** `sum_of_squares(n: i32)`
- **What it does:** Calculates the sum of squares from `0` through `n` (inclusive) and prints the result.
- **Example:** `n = 5` → prints `55`.

---

### Question 2 – Custom Message
- **Function:** `custom_message(name: &str, age: i32, height: f32, is_student: bool)`
- **What it does:** Prints a formatted sentence about a person using their name, age, height, and
  whether they are a student.
- **Example:** For `("Talal", 24, 1.88, false)` → prints  
  `Talal is 24 years old. Their height is 1.88 meters and they are not a student`

---

### Question 3 – Min, Max, and Average
- **Function:** `min_max_avg(numbers: &[i32]) -> Option<(i32, i32, f32)>`
- **What it does:** Given a slice of integers, computes:
  - **max** value
  - **min** value
  - **average** (mean) as `f32`
  Returns `None` if the slice is empty.
- **Example:** For `[3, 2, 6, 4, 5, 8, 2]` → returns `Some((8, 2, 4.285714))`, and in `main`
  it’s printed with `{:?}`.

> Note: In the current implementation `max` is initialized to `0`, so an all-negative input slice
> would not behave as expected.

---

### Question 4 – Guessing Game
- **Function:** `guessing_game()`
- **What it does:** Simple terminal-based guessing game for the fixed number `42`. The user has
  up to **5 attempts**. Prints success/failure at the end.
- **Input expectation:** The program calls `.parse().unwrap()`, so **non-integer input will panic**.
  Enter integers only (e.g., `42`).

---

### Question 5 – Repeat String
- **Function:** `repeat_string(word: String) -> String`
- **What it does:** Returns a new string that repeats the input string three times.
- **Example:** `"hi"` → `"hihihi"` (and `main` prints it).

---

### Question 6 – Append Exclamation
- **Function:** `append_exclamation(word: &mut String)`
- **What it does:** Appends `!` to the end of the string **only if** it doesn’t already end with one.
- **Example:** `"POTATO"` → `"POTATO!"`; `"POTATO!"` stays `"POTATO!"`.

> Note: The current slicing logic assumes the string is non-empty; calling it on an empty string
> would panic.

---

### Question 7 – Process Word
- **Function:** `process_word(word: &String) -> String`
- **What it does:** If the string ends with `!`, it removes that `!` and then reverses the remaining
  characters. Otherwise, it simply reverses the entire string.
- **Examples:**
  - `"RUST!"` → `"TSUR"`
  - `"HELLO"` → `"OLLEH"`

---

## Running the Project

1. Install Rust and Cargo: <https://www.rust-lang.org/tools/install>
2. Clone the repository:
   - `git clone https://github.com/TalalFarrukh/rust-test.git`
   - `cd rust-test`
3. Run:
   - `cargo run`
4. (Optional) Comment out questions in `main` that you don’t want to run while testing.

---

## Project Structure

- `src/main.rs` — contains all seven question functions and the `main` entry point.
- `Cargo.toml` / `Cargo.lock` — standard Cargo configuration files.
- `.gitignore` — typical Rust target/IDE ignores.

---

## Notes & Caveats

- **Input handling:** `guessing_game()` expects valid integer input; invalid input will cause a panic due to `.unwrap()`.
- **Edge cases:**
  - `append_exclamation` will panic on empty strings because it slices the last character.
  - `min_max_avg` initializes `max` to `0`, which is unsuitable for all-negative slices.
  These are acceptable for practice but good candidates for improvement exercises.

---

## License

This project is for learning and practice purposes. Add a license file if you plan to share or collaborate.
