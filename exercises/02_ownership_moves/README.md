# 02 — Ownership and moves

Work on one numbered step at a time. Write your code in `src/bin/`; the supplied checks describe expected behaviour and are not solutions.

Run commands from the repository root. A function check runs only that step, so other `todo!()` bodies can remain unfinished. Do not change the supplied checks to make an incorrect answer pass.

The checks cover the listed examples; also explain your result and try one new input before recording a task as done.

## Small syntax reference

`String::from("hello")` creates an owned string. `&s` lends shared access; `&mut s` lends mutable access. A vector such as `vec![1, 2, 3]` is a growable list; `for value in values` visits its elements. A signature tells you whether an input is owned or borrowed. More vector practice comes in topic 10.

## Write

Open `src/bin/recall.rs`.

1. Implement `append_exclamation(s: String) -> String`. Take ownership of `s`, append one `!`, and return the string. `"hello" → "hello!"`; an empty string becomes `"!"`.

   Check: `python3 scripts/check.py 02_ownership_moves recall append_exclamation`.

2. Implement `shout(s: String) -> String`. Take ownership and return the uppercase version. Preserve punctuation. `"hello!" → "HELLO!"`; an empty string stays empty.

   Check: `python3 scripts/check.py 02_ownership_moves recall shout`.

After all steps: `cargo run -p ex_02_ownership_moves --bin recall`.

Main example (expressions → expected results):

| Expression | Expected result |
| --- | --- |
| `s` | `hello!` |
| `shout(s)` | `HELLO!` |

## Predict

Open `src/bin/predict.rs`.

Predict every printed line. Explain why the commented prints of s and s2 would fail, why n remains usable, and why cloning leaves s4 usable.

Run: `cargo run -p ex_02_ownership_moves --bin predict`.

Done when you have compared the output with your prediction and explained each difference.

## Repair

Open `src/bin/repair.rs`.

Keep every printed value available. For error 1, print s before transferring ownership. For error 2, print s2 before moving it to s3, then print s3 separately. For error 3, preserve s4 by cloning at the ownership-taking call. Use clone only for error 3.

Run: `cargo run -p ex_02_ownership_moves --bin repair`.

Done when the program compiles, runs without an unexpected panic, and meets the results above. Explain the cause of each error. Some fixes address logic errors or warnings, not compiler errors.

## Extend

Open `src/bin/extend.rs`.

1. Reimplement from recall: `append_exclamation(s: String) -> String`. Take ownership of `s`, append one `!`, and return the string. `"hello" → "hello!"`; an empty string becomes `"!"`.

   Check: `python3 scripts/check.py 02_ownership_moves extend append_exclamation`.

2. Reimplement from recall: `shout(s: String) -> String`. Take ownership and return the uppercase version. Preserve punctuation. `"hello!" → "HELLO!"`; an empty string stays empty.

   Check: `python3 scripts/check.py 02_ownership_moves extend shout`.

3. Implement `double_elements(v: Vec<i32>) -> Vec<i32>`. Take ownership and return the values doubled in their original order. `[1, -2, 0] → [2, -4, 0]`; an empty vector stays empty.

   Check: `python3 scripts/check.py 02_ownership_moves extend double_elements`.

4. Implement `sum_and_drop(v: Vec<i32>) -> i32`. Take ownership and return the sum. `[1, 2, 3] → 6`; an empty vector returns 0. Explain why the caller cannot use its vector afterwards.

   Check: `python3 scripts/check.py 02_ownership_moves extend sum_and_drop`.

After all steps: `cargo run -p ex_02_ownership_moves --bin extend`.

Main example (expressions → expected results):

| Expression | Expected result |
| --- | --- |
| `shout(append_exclamation(s))` | `HELLO!` |
| `double_elements(v)` | `[2, 4, 6, 8, 10]` |
| `sum_and_drop(v2)` | `15` |

## Review later

Open `src/bin/revisit.rs`.

Try these a few days later without copying earlier solutions. Continue to the next topic now if you prefer; return here for review.

1. Implement `create_entry(timestamp: String, message: String) -> String`. Consume both strings and return `"[timestamp] message"`, with one space after `]`. Empty inputs produce `"[] "`.

   Check: `python3 scripts/check.py 02_ownership_moves revisit create_entry`.

2. Implement `prefix_entry(prefix: String, entry: String) -> String`. Consume both strings and return `"prefix: entry"`, with one space after the colon.

   Check: `python3 scripts/check.py 02_ownership_moves revisit prefix_entry`.

After all steps: `cargo run -p ex_02_ownership_moves --bin revisit`.

Main example (expressions → expected results):

| Expression | Expected result |
| --- | --- |
| `result` | `INFO: [2024-01-01] Server started` |

Next topic: [03_borrowing_shared](../03_borrowing_shared/README.md). Move on after Write, Predict, Repair, and Extend; keep Review later for another session.
