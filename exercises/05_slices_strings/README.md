# 05 — Slices and strings

A string slice (`&str`) is a borrowed view into a string — a pointer and a length, with no heap allocation. `&String` coerces to `&str`, so functions that take `&str` accept both. Slices let you return a piece of the original without copying.

Complete one step at a time. Run checks from the repository root. Do not change supplied checks to make an incorrect answer pass.

## Write

Open `src/bin/recall.rs`.

1. Implement `first_word(s: &str) -> &str`. Return the slice before the first space. No space → return all of `s`. Leading space or empty → return `""`.

   Check: `python3 scripts/check.py 05_slices_strings recall first_word`

2. Implement `last_word(s: &str) -> &str`. Return the slice after the last space. No space → return all of `s`. Trailing space or empty → return `""`.

   Check: `python3 scripts/check.py 05_slices_strings recall last_word`

Run: `cargo run -p ex_05_slices_strings --bin recall`

| Expression | Expected |
| --- | --- |
| `first_word(&owned)` | `the` |
| `last_word(&owned)` | `fox` |
| `first_word("onlyone")` | `onlyone` |
| `last_word("onlyone")` | `onlyone` |

## Predict

Open `src/bin/predict.rs`.

Predict every printed line. Identify the types of `hello` and `world`. Explain the difference between a borrowed slice and an owned `String`, and why byte indices must fall on UTF-8 boundaries.

Run: `cargo run -p ex_05_slices_strings --bin predict`

## Repair

Open `src/bin/repair.rs`.

The first slice deliberately ends inside `é` and panics. Fix it to print `"hé"` using a valid UTF-8 boundary. Print the borrowed first word before mutating `owned`. Keep `portion` borrowed as `&str` and print `"some"`.

Run: `cargo run -p ex_05_slices_strings --bin repair`

## Extend

Open `src/bin/extend.rs`.

1. Reimplement `first_word` and `last_word` from recall.

   Check: `python3 scripts/check.py 05_slices_strings extend first_word` (and `last_word`)

2. Implement `word_count(s: &str) -> usize`. Count whitespace-separated words; ignore leading, trailing, and repeated whitespace. Empty → 0.

   Check: `python3 scripts/check.py 05_slices_strings extend word_count`

3. Implement `reverse_words(s: &str) -> String`. Reverse word order, one space between each. `"the quick fox"` → `"fox quick the"`. Empty → `""`.

   Check: `python3 scripts/check.py 05_slices_strings extend reverse_words`

Run: `cargo run -p ex_05_slices_strings --bin extend`

| Expression | Expected |
| --- | --- |
| `first_word(s)` | `the` |
| `last_word(s)` | `fox` |
| `word_count(s)` | `4` |
| `word_count("  a  b  ")` | `2` |
| `reverse_words(s)` | `fox brown quick the` |
| `reverse_words("one")` | `one` |

## Review later

Open `src/bin/revisit.rs`. Try these a few days later without copying earlier solutions.

1. Implement `truncate_words(s, max_words) -> String`. Return at most the first `max_words` words joined by single spaces. Limit 0 or empty → `""`.

   Check: `python3 scripts/check.py 05_slices_strings revisit truncate_words`

2. Implement `contains_word(text, word) -> bool`. True only when a whitespace-delimited token exactly equals `word`. `"worldwide"` does not contain `"world"`. Empty `word` → false.

   Check: `python3 scripts/check.py 05_slices_strings revisit contains_word`

Run: `cargo run -p ex_05_slices_strings --bin revisit`

| Expression | Expected |
| --- | --- |
| `truncate_words("the quick brown fox", 2)` | `the quick` |
| `truncate_words("short", 5)` | `short` |
| `contains_word("hello world", "world")` | `true` |
| `contains_word("hello worldwide", "world")` | `false` |
| `contains_word("world peace", "world")` | `true` |

Next: [06_structs_impl](../06_structs_impl/README.md). Move on after Write, Predict, Repair, and Extend. Return here for Review later in a separate session.
