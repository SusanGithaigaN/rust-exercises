# 08 — Option

`Option<T>` is either `Some(value)` or `None`. It replaces null. Use it when a value might be absent — an empty slice has no maximum, dividing by zero has no result. Common methods: `.map`, `.and_then`, `.unwrap_or`.

Complete one step at a time. Run checks from the repository root. Do not change supplied checks to make an incorrect answer pass.

## Write

Open `src/bin/recall.rs`.

1. Implement `first_even(v: &[i32]) -> Option<i32>`. Return a copy of the first even value (including zero and negative even values). Return `None` when there is no even value or the slice is empty.

   Check: `python3 scripts/check.py 08_option recall first_even`

2. Implement `safe_divide(a: f64, b: f64) -> Option<f64>`. Return `Some(a/b)`, or `None` when `b == 0.0`. Assume finite inputs.

   Check: `python3 scripts/check.py 08_option recall safe_divide`

3. Implement `head(v: &[i32]) -> Option<i32>`. Return a copy of the first element, or `None` if empty.

   Check: `python3 scripts/check.py 08_option recall head`

Run: `cargo run -p ex_08_option --bin recall`

| Expression | Expected |
| --- | --- |
| `first_even(&[1, 3, 4, 7])` | `Some(4)` |
| `first_even(&[1, 3, 5, 7])` | `None` |
| `safe_divide(10.0, 2.0)` | `Some(5.0)` |
| `safe_divide(10.0, 0.0)` | `None` |
| `head(&[9, 2, 3])` | `Some(9)` |
| `head(&[])` | `None` |

## Predict

Open `src/bin/predict.rs`.

Predict every printed line. Explain what `.map` does to `Some` and `None`, what `.unwrap_or` supplies, and when the `if let` body executes.

Run: `cargo run -p ex_08_option --bin predict`

## Repair

Open `src/bin/repair.rs`.

Handle the missing negative value without panicking — print `"no negative number"`. Compare `maybe` with `Some(5)`. Double `x` only when it contains a number, printing `84`. Add a `None` branch to the name match that prints `"no name"`.

Run: `cargo run -p ex_08_option --bin repair`

## Extend

Open `src/bin/extend.rs`.

1. Reimplement `first_even`, `safe_divide`, and `head` from recall.

2. Implement `find_index(v: &[i32], target: i32) -> Option<usize>`. Return the zero-based index of the first matching element, or `None`.

   Check: `python3 scripts/check.py 08_option extend find_index`

3. Implement `last_element(v: &[i32]) -> Option<i32>`. Return a copy of the last element, or `None` if empty.

   Check: `python3 scripts/check.py 08_option extend last_element`

Run: `cargo run -p ex_08_option --bin extend`

| Expression | Expected |
| --- | --- |
| `find_index(&[1, 3, 5, 7, 9], 5)` | `Some(2)` |
| `find_index(&[1, 3, 5, 7, 9], 4)` | `None` |
| `last_element(&[9, 2, 3])` | `Some(3)` |
| `last_element(&[])` | `None` |

## Review later

Open `src/bin/revisit.rs`. Try these a few days later without copying earlier solutions.

1. `parse_age(s: &str) -> Option<u32>`. Accept integers 0–150 inclusive. Return `None` for empty input, non-numeric text, surrounding whitespace, or values above 150.

   Check: `python3 scripts/check.py 08_option revisit parse_age`

2. `greet(name: Option<&str>) -> String`. `Some("Alice")` → `"Hello, Alice!"`. `None` → `"Hello, stranger!"`.

   Check: `python3 scripts/check.py 08_option revisit greet`

3. `find_matching<'a>(v: &'a [String], prefix: &str) -> Option<&'a String>`. Return the first string starting with `prefix` (case-sensitive), or `None`. Empty prefix matches the first string; empty slice returns `None`.

   Check: `python3 scripts/check.py 08_option revisit find_matching`

Run: `cargo run -p ex_08_option --bin revisit`

| Expression | Expected |
| --- | --- |
| `parse_age("25")` | `Some(25)` |
| `parse_age("abc")` | `None` |
| `parse_age("200")` | `None` |
| `greet(Some("Alice"))` | `Hello, Alice!` |
| `greet(None)` | `Hello, stranger!` |
| `find_matching(&words, "ru")` | `Some("rust")` |
| `find_matching(&words, "py")` | `None` |

Next: [09_result](../09_result/README.md). Move on after Write, Predict, Repair, and Extend. Return here for Review later in a separate session.
