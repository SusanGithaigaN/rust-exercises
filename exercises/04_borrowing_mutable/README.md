# 04 — Mutable borrowing

Work on one numbered step at a time. Write your code in `src/bin/`; the supplied checks describe expected behaviour and are not solutions.

Run commands from the repository root. A function check runs only that step, so other `todo!()` bodies can remain unfinished. Do not change the supplied checks to make an incorrect answer pass.

The checks cover the listed examples; also explain your result and try one new input before recording a task as done.

## Small syntax reference

`String::from("hello")` creates an owned string. `&s` lends shared access; `&mut s` lends mutable access. A vector such as `vec![1, 2, 3]` is a growable list; `for value in values` visits its elements. A signature tells you whether an input is owned or borrowed. More vector practice comes in topic 10.

## Write

Open `src/bin/recall.rs`.

1. Implement `append_greeting(s: &mut String)`. Append `", hello!"` to the original string. Return nothing. `"world" → "world, hello!"`; empty input becomes `", hello!"`.

   Check: `python3 scripts/check.py 04_borrowing_mutable recall append_greeting`.

2. Implement `double_all(v: &mut Vec<i32>)`. Double every value in place without changing order or length. Empty input stays empty.

   Check: `python3 scripts/check.py 04_borrowing_mutable recall double_all`.

3. Implement `clamp_to_zero(n: &mut i32)`. Replace a negative value with 0. Leave zero and positive values unchanged.

   Check: `python3 scripts/check.py 04_borrowing_mutable recall clamp_to_zero`.

After all steps: `cargo run -p ex_04_borrowing_mutable --bin recall`.

Main example (expressions → expected results):

| Expression | Expected result |
| --- | --- |
| `s` | `world, hello!` |
| `v` | `[2, 6, 10, 14]` |
| `a` | `0` |
| `b` | `3` |

## Predict

Open `src/bin/predict.rs`.

Both examples are called. Predict the output of each. Explain why a second mutable borrow is allowed after the first reference’s last use, and why the shared borrows end before r3 is used.

Run: `cargo run -p ex_04_borrowing_mutable --bin predict`.

Done when you have compared the output with your prediction and explained each difference.

## Repair

Open `src/bin/repair.rs`.

Use r1 before creating r2; keep both prints. Read and print the first vector element before pushing. Change n through a mutable reference, producing 20. Explain each reference’s last use.

Run: `cargo run -p ex_04_borrowing_mutable --bin repair`.

Done when the program compiles, runs without an unexpected panic, and meets the results above. Explain the cause of each error. Some fixes address logic errors or warnings, not compiler errors.

## Extend

Open `src/bin/extend.rs`.

1. Reimplement from recall: `append_greeting(s: &mut String)`. Append `", hello!"` to the original string. Return nothing. `"world" → "world, hello!"`; empty input becomes `", hello!"`.

   Check: `python3 scripts/check.py 04_borrowing_mutable extend append_greeting`.

2. Reimplement from recall: `double_all(v: &mut Vec<i32>)`. Double every value in place without changing order or length. Empty input stays empty.

   Check: `python3 scripts/check.py 04_borrowing_mutable extend double_all`.

3. Reimplement from recall: `clamp_to_zero(n: &mut i32)`. Replace a negative value with 0. Leave zero and positive values unchanged.

   Check: `python3 scripts/check.py 04_borrowing_mutable extend clamp_to_zero`.

4. Implement `capitalize_first(s: &mut String)`. Uppercase the first Unicode character and leave the remaining characters unchanged. An uppercase conversion can expand into multiple characters. Empty input stays empty.

   Check: `python3 scripts/check.py 04_borrowing_mutable extend capitalize_first`.

After all steps: `cargo run -p ex_04_borrowing_mutable --bin extend`.

Main example (expressions → expected results):

| Expression | Expected result |
| --- | --- |
| `s` | `world, hello!` |
| `v` | `[2, 6, 10, 14]` |
| `a` | `Hello` |
| `b` | `Rust is great` |

## Review later

Open `src/bin/revisit.rs`.

Try these a few days later without copying earlier solutions. Continue to the next topic now if you prefer; return here for review.

1. Implement `remove_duplicates(v: &mut Vec<i32>)`. Keep only the first occurrence of each value, preserving original order. `[3, 1, 3, 2, 1] → [3, 1, 2]`; empty input stays empty.

   Check: `python3 scripts/check.py 04_borrowing_mutable revisit remove_duplicates`.

2. Implement `normalize(v: &mut Vec<f64>)`. For nonnegative, finite inputs, divide each value by the largest value. `[1, 2, 3] → [1/3, 2/3, 1]`. Leave empty and all-zero vectors unchanged.

   Check: `python3 scripts/check.py 04_borrowing_mutable revisit normalize`.

After all steps: `cargo run -p ex_04_borrowing_mutable --bin revisit`.

Main example (expressions → expected results):

| Expression | Expected result |
| --- | --- |
| `v` | `[3, 1, 4, 5, 9, 2, 6]` |
| `f` | `[0.333..., 0.666..., 1.0]` |

Next topic: [05_slices_strings](../05_slices_strings/README.md). Move on after Write, Predict, Repair, and Extend; keep Review later for another session.
