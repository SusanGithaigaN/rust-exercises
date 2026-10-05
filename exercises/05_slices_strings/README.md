# 05 — Slices and strings

Work on one numbered step at a time. Write your code in `src/bin/`; the supplied checks describe expected behaviour and are not solutions.

Run commands from the repository root. A function check runs only that step, so other `todo!()` bodies can remain unfinished. Do not change the supplied checks to make an incorrect answer pass.

The checks cover the listed examples; also explain your result and try one new input before recording a task as done.

## Write

Open `src/bin/recall.rs`.

1. Implement `first_word(s: &str) -> &str`. Return the slice before the first literal space. If there is no space, return all of `s`. Empty input or a leading space returns `""`. Do not allocate a new string.

   Check: `python3 scripts/check.py 05_slices_strings recall first_word`.

2. Implement `last_word(s: &str) -> &str`. Return the slice after the last literal space. If there is no space, return all of `s`. Empty input or a trailing space returns `""`. Do not allocate a new string.

   Check: `python3 scripts/check.py 05_slices_strings recall last_word`.

After all steps: `cargo run -p ex_05_slices_strings --bin recall`.

Main example (expressions → expected results):

| Expression | Expected result |
| --- | --- |
| `first_word(&owned)` | `the` |
| `last_word(&owned)` | `fox` |
| `first_word("onlyone")` | `onlyone` |
| `last_word("onlyone")` | `onlyone` |

## Predict

Open `src/bin/predict.rs`.

Predict every printed line. Identify the types of hello and world. Explain the difference between a borrowed slice and an owned String, and why byte indices must fall on UTF-8 boundaries.

Run: `cargo run -p ex_05_slices_strings --bin predict`.

Done when you have compared the output with your prediction and explained each difference.

## Repair

Open `src/bin/repair.rs`.

The first slice deliberately ends inside é and panics. Make it print "hé" using a valid UTF-8 boundary. Print the borrowed first word before mutating owned. Keep portion borrowed as &str and print "some".

Run: `cargo run -p ex_05_slices_strings --bin repair`.

Done when the program compiles, runs without an unexpected panic, and meets the results above. Explain the cause of each error. Some fixes address logic errors or warnings, not compiler errors.

## Extend

Open `src/bin/extend.rs`.

1. Reimplement from recall: `first_word(s: &str) -> &str`. Return the slice before the first literal space. If there is no space, return all of `s`. Empty input or a leading space returns `""`. Do not allocate a new string.

   Check: `python3 scripts/check.py 05_slices_strings extend first_word`.

2. Reimplement from recall: `last_word(s: &str) -> &str`. Return the slice after the last literal space. If there is no space, return all of `s`. Empty input or a trailing space returns `""`. Do not allocate a new string.

   Check: `python3 scripts/check.py 05_slices_strings extend last_word`.

3. Implement `word_count(s: &str) -> usize`. Count words separated by whitespace, including tabs and newlines. Ignore repeated and surrounding whitespace. Empty input returns 0.

   Check: `python3 scripts/check.py 05_slices_strings extend word_count`.

4. Implement `reverse_words(s: &str) -> String`. Reverse whitespace-separated words, keeping each word intact. Join with one space and discard surrounding whitespace. `"the quick fox" → "fox quick the"`; empty input returns `""`.

   Check: `python3 scripts/check.py 05_slices_strings extend reverse_words`.

After all steps: `cargo run -p ex_05_slices_strings --bin extend`.

Main example (expressions → expected results):

| Expression | Expected result |
| --- | --- |
| `first_word(s)` | `the` |
| `last_word(s)` | `fox` |
| `word_count(s)` | `4` |
| `word_count("  a  b  ")` | `2` |
| `reverse_words(s)` | `fox brown quick the` |
| `reverse_words("one")` | `one` |

## Review later

Open `src/bin/revisit.rs`.

Try these a few days later without copying earlier solutions. Continue to the next topic now if you prefer; return here for review.

1. Implement `truncate_words(s, max_words)`. Return at most the first `max_words` whitespace-separated words, joined with single spaces. A limit of 0 or empty input returns `""`.

   Check: `python3 scripts/check.py 05_slices_strings revisit truncate_words`.

2. Implement `contains_word(text, word)`. Return true if a whitespace-separated token exactly equals `word`. Matching is case-sensitive and punctuation remains part of the token. Empty `word` returns false.

   Check: `python3 scripts/check.py 05_slices_strings revisit contains_word`.

After all steps: `cargo run -p ex_05_slices_strings --bin revisit`.

Main example (expressions → expected results):

| Expression | Expected result |
| --- | --- |
| `truncate_words("the quick brown fox", 2)` | `the quick` |
| `truncate_words("short", 5)` | `short` |
| `contains_word("hello world", "world")` | `true` |
| `contains_word("hello worldwide", "world")` | `false` |
| `contains_word("world peace", "world")` | `true` |

Next topic: [06_structs_impl](../06_structs_impl/README.md). Move on after Write, Predict, Repair, and Extend; keep Review later for another session.
