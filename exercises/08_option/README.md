# 08 — Option and missing values

Work on one numbered step at a time. Write your code in `src/bin/`; the supplied checks describe expected behaviour and are not solutions.

Run commands from the repository root. A function check runs only that step, so other `todo!()` bodies can remain unfinished. Do not change the supplied checks to make an incorrect answer pass.

The checks cover the listed examples; also explain your result and try one new input before recording a task as done.

## Write

Open `src/bin/recall.rs`.

1. Implement `first_even(v) -> Option<i32>`. Return a copy of the first even value, including zero or negative even values. Return None when there is no even value or the slice is empty.

   Check: `python3 scripts/check.py 08_option recall first_even`.

2. Implement `safe_divide(a: f64, b: f64) -> Option<f64>`. Return None when `b == 0.0`; otherwise return Some(a / b). Assume finite inputs.

   Check: `python3 scripts/check.py 08_option recall safe_divide`.

3. Implement `head(v) -> Option<i32>`. Return a copy of the first element or None for an empty slice.

   Check: `python3 scripts/check.py 08_option recall head`.

After all steps: `cargo run -p ex_08_option --bin recall`.

Main example (expressions → expected results):

| Expression | Expected result |
| --- | --- |
| `first_even(&[1, 3, 4, 7])` | `Some(4)` |
| `first_even(&[1, 3, 5, 7])` | `None` |
| `safe_divide(10.0, 2.0)` | `Some(5.0)` |
| `safe_divide(10.0, 0.0)` | `None` |
| `head(&[9, 2, 3])` | `Some(9)` |
| `head(&[])` | `None` |

## Predict

Open `src/bin/predict.rs`.

Predict every printed line. Explain what map does to Some and None, what unwrap_or supplies, and when the if let body executes.

Run: `cargo run -p ex_08_option --bin predict`.

Done when you have compared the output with your prediction and explained each difference.

## Repair

Open `src/bin/repair.rs`.

Handle the missing negative value without panicking: print "no negative number". Compare maybe with Some(5). Double x only when it contains a number, printing 84. Add a None branch to the name match that prints "no name".

Run: `cargo run -p ex_08_option --bin repair`.

Done when the program compiles, runs without an unexpected panic, and meets the results above. Explain the cause of each error. Some fixes address logic errors or warnings, not compiler errors.

## Extend

Open `src/bin/extend.rs`.

1. Reimplement from recall: `first_even(v) -> Option<i32>`. Return a copy of the first even value, including zero or negative even values. Return None when there is no even value or the slice is empty.

   Check: `python3 scripts/check.py 08_option extend first_even`.

2. Reimplement from recall: `safe_divide(a: f64, b: f64) -> Option<f64>`. Return None when `b == 0.0`; otherwise return Some(a / b). Assume finite inputs.

   Check: `python3 scripts/check.py 08_option extend safe_divide`.

3. Reimplement from recall: `head(v) -> Option<i32>`. Return a copy of the first element or None for an empty slice.

   Check: `python3 scripts/check.py 08_option extend head`.

4. Implement `find_index(v, target) -> Option<usize>`. Return the zero-based index of the first matching element. Missing values and empty input return None.

   Check: `python3 scripts/check.py 08_option extend find_index`.

5. Implement `last_element(v) -> Option<i32>`. Return a copy of the last element or None for empty input.

   Check: `python3 scripts/check.py 08_option extend last_element`.

After all steps: `cargo run -p ex_08_option --bin extend`.

Main example (expressions → expected results):

| Expression | Expected result |
| --- | --- |
| `first_even(&[1, 3, 4, 7])` | `Some(4)` |
| `safe_divide(10.0, 0.0)` | `None` |
| `head(&[])` | `None` |
| `find_index(&[1, 3, 5, 7, 9], 5)` | `Some(2)` |
| `find_index(&[1, 3, 5, 7, 9], 4)` | `None` |
| `last_element(&[9, 2, 3])` | `Some(3)` |
| `last_element(&[])` | `None` |
| `result` | `Some(25)` |

## Review later

Open `src/bin/revisit.rs`.

Try these a few days later without copying earlier solutions. Continue to the next topic now if you prefer; return here for review.

1. Implement `parse_age(s) -> Option<u32>`. Accept integer text from 0 through 150 inclusive. Return None for empty input, invalid unsigned integer text, surrounding whitespace, or a value above 150.

   Check: `python3 scripts/check.py 08_option revisit parse_age`.

2. Implement `greet(name: Option<&str>) -> String`. Some(name) returns `"Hello, name!"`; None returns `"Hello, stranger!"`. Some("") returns `"Hello, !"`.

   Check: `python3 scripts/check.py 08_option revisit greet`.

3. Implement `find_matching(v, prefix)`. Return a shared reference to the first string starting with the case-sensitive prefix, or None. An empty prefix matches the first string; an empty slice returns None. Keep the provided lifetime signature.

   Check: `python3 scripts/check.py 08_option revisit find_matching`.

After all steps: `cargo run -p ex_08_option --bin revisit`.

Main example (expressions → expected results):

| Expression | Expected result |
| --- | --- |
| `parse_age("25")` | `Some(25)` |
| `parse_age("abc")` | `None` |
| `parse_age("200")` | `None` |
| `greet(Some("Alice"))` | `Hello, Alice!` |
| `greet(None)` | `Hello, stranger!` |
| `find_matching(&words, "ru")` | `Some("rust")` |
| `find_matching(&words, "py")` | `None` |

Next topic: [09_result](../09_result/README.md). Move on after Write, Predict, Repair, and Extend; keep Review later for another session.
