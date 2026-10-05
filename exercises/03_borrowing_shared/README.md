# 03 — Shared borrowing

Work on one numbered step at a time. Write your code in `src/bin/`; the supplied checks describe expected behaviour and are not solutions.

Run commands from the repository root. A function check runs only that step, so other `todo!()` bodies can remain unfinished. Do not change the supplied checks to make an incorrect answer pass.

The checks cover the listed examples; also explain your result and try one new input before recording a task as done.

## Small syntax reference

`String::from("hello")` creates an owned string. `&s` lends shared access; `&mut s` lends mutable access. A vector such as `vec![1, 2, 3]` is a growable list; `for value in values` visits its elements. A signature tells you whether an input is owned or borrowed. More vector practice comes in topic 10.

## Write

Open `src/bin/recall.rs`.

1. Implement `char_count(s: &String) -> usize`. Borrow the string and count Unicode scalar values using characters, rather than bytes. `"hé" → 2`; empty input returns 0. Leave the original string usable.

   Check: `python3 scripts/check.py 03_borrowing_shared recall char_count`.

2. Implement `starts_with_vowel(s: &String) -> bool`. Return true only if the first character is one of lowercase `a`, `e`, `i`, `o`, `u`. Empty input and uppercase vowels return false.

   Check: `python3 scripts/check.py 03_borrowing_shared recall starts_with_vowel`.

3. Implement `count_spaces(s: &String) -> usize`. Count literal space characters (`' ') only. Tabs and newlines do not count. Empty input returns 0.

   Check: `python3 scripts/check.py 03_borrowing_shared recall count_spaces`.

After all steps: `cargo run -p ex_03_borrowing_shared --bin recall`.

Main example (expressions → expected results):

| Expression | Expected result |
| --- | --- |
| `char_count(&s)` | `13` |
| `starts_with_vowel(&s)` | `true` |
| `count_spaces(&s)` | `2` |
| `s` | `open the door  (still usable)` |

## Predict

Open `src/bin/predict.rs`.

Predict every printed line. Identify which variables own their strings and which only borrow them. Explain why multiple shared references can coexist.

Run: `cargo run -p ex_03_borrowing_shared --bin predict`.

Done when you have compared the output with your prediction and explained each difference.

## Repair

Open `src/bin/repair.rs`.

Borrow at the get_length and print_twice call sites, leaving s1 and s2 usable. Uncomment the make_greeting block; it cannot return a reference to its local String, so change that function to return an owned String. Keep the other two function signatures.

Run: `cargo run -p ex_03_borrowing_shared --bin repair`.

Done when the program compiles, runs without an unexpected panic, and meets the results above. Explain the cause of each error. Some fixes address logic errors or warnings, not compiler errors.

## Extend

Open `src/bin/extend.rs`.

1. Reimplement from recall: `char_count(s: &String) -> usize`. Borrow the string and count Unicode scalar values using characters, rather than bytes. `"hé" → 2`; empty input returns 0. Leave the original string usable.

   Check: `python3 scripts/check.py 03_borrowing_shared extend char_count`.

2. Reimplement from recall: `starts_with_vowel(s: &String) -> bool`. Return true only if the first character is one of lowercase `a`, `e`, `i`, `o`, `u`. Empty input and uppercase vowels return false.

   Check: `python3 scripts/check.py 03_borrowing_shared extend starts_with_vowel`.

3. Reimplement from recall: `count_spaces(s: &String) -> usize`. Count literal space characters (`' ') only. Tabs and newlines do not count. Empty input returns 0.

   Check: `python3 scripts/check.py 03_borrowing_shared extend count_spaces`.

4. Implement `count_vowels(s: &str) -> usize`. Count lowercase `a`, `e`, `i`, `o`, `u` anywhere in the string. Uppercase letters do not count. Empty input returns 0.

   Check: `python3 scripts/check.py 03_borrowing_shared extend count_vowels`.

5. Implement `is_palindrome(s: &str) -> bool`. Compare characters forwards and backwards exactly, including case, spaces, and punctuation. Empty input and a single character are palindromes.

   Check: `python3 scripts/check.py 03_borrowing_shared extend is_palindrome`.

After all steps: `cargo run -p ex_03_borrowing_shared --bin extend`.

Main example (expressions → expected results):

| Expression | Expected result |
| --- | --- |
| `char_count(&s)` | `13` |
| `starts_with_vowel(&s)` | `true` |
| `count_spaces(&s)` | `2` |
| `count_vowels("hello world")` | `3` |
| `count_vowels("rhythm")` | `0` |
| `is_palindrome("racecar")` | `true` |
| `is_palindrome("hello")` | `false` |

## Review later

Open `src/bin/revisit.rs`.

Try these a few days later without copying earlier solutions. Continue to the next topic now if you prefer; return here for review.

1. Implement `analyze(text: &str) -> (usize, usize, usize, bool)`. Return, in order: whitespace-separated word count; Unicode character count including spaces; count of individual `.`, `!`, and `?` characters; whether any `?` occurs. Empty input returns `(0, 0, 0, false)`.

   Check: `python3 scripts/check.py 03_borrowing_shared revisit analyze`.

After all steps: `cargo run -p ex_03_borrowing_shared --bin revisit`.

Main example (expressions → expected results):

| Expression | Expected result |
| --- | --- |
| `words` | `5` |
| `chars` | `24` |
| `sentences` | `2` |
| `question` | `true` |
| `s` | `still usable` |

Next topic: [04_borrowing_mutable](../04_borrowing_mutable/README.md). Move on after Write, Predict, Repair, and Extend; keep Review later for another session.
