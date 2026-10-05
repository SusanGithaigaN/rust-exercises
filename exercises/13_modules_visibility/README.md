# 13 — Modules and visibility

Work on one numbered step at a time. Write your code in `src/bin/`; the supplied checks describe expected behaviour and are not solutions.

Run commands from the repository root. A function check runs only that step, so other `todo!()` bodies can remain unfinished. Do not change the supplied checks to make an incorrect answer pass.

The checks cover the listed examples; also explain your result and try one new input before recording a task as done.

## Write

Open `src/bin/recall.rs`.

1. Implement `geometry::Point::new` and `distance`. Store x and y; calculate √((x₂−x₁)² + (y₂−y₁)²). Keep the public API inside the geometry module.

   Check: `python3 scripts/check.py 13_modules_visibility recall point`.

2. Implement `geometry::shapes::Circle::new`, `area`, and `contains`. Store the center and radius; area = πr²; contains returns true when distance from the center is <= radius, including the boundary. Assume nonnegative finite radii.

   Check: `python3 scripts/check.py 13_modules_visibility recall circle`.

After all steps: `cargo run -p ex_13_modules_visibility --bin recall`.

Main example (expressions → expected results):

| Expression | Expected result |
| --- | --- |
| `origin.distance(&far)` | `5` |
| `c.area()` | `314.1592653589793` |
| `c.contains(&Point::new(3.0, 4.0))` | `true` |
| `c.contains(&Point::new(10.0, 10.0))` | `false` |

## Predict

Open `src/bin/predict.rs`.

Predict every printed line. Explain why main cannot read Config.secret_key, why a child module can read its parent’s private fields, and why outer::describe is accessible within the crate.

Run: `cargo run -p ex_13_modules_visibility --bin predict`.

Done when you have compared the output with your prediction and explained each difference.

## Repair

Open `src/bin/repair.rs`.

Expose age through a public getter while keeping the field private. Make tricks accessible through animals. Use the correct module path or import for Dog. Preserve Rex (age 3) and Buddy (age 2).

Run: `cargo run -p ex_13_modules_visibility --bin repair`.

Done when the program compiles, runs without an unexpected panic, and meets the results above. Explain the cause of each error. Some fixes address logic errors or warnings, not compiler errors.

## Extend

Open `src/bin/extend.rs`.

1. Copy the completed Point implementation from recall into the supplied `src/bin/geometry/mod.rs` skeleton. Copy Circle into `src/bin/geometry/shapes.rs`. Keep `pub mod shapes;` in mod.rs and `mod geometry;` in extend.rs. Preserve the behaviour from recall.

   Check: `python3 scripts/check.py 13_modules_visibility extend modules`.

After all steps: `cargo run -p ex_13_modules_visibility --bin extend`.

Main example (expressions → expected results):

| Expression | Expected result |
| --- | --- |
| `origin.distance(&far)` | `5` |
| `c.area()` | `314.1592653589793` |
| `c.contains(&Point::new(3.0, 4.0))` | `true` |
| `c.contains(&Point::new(10.0, 10.0))` | `false` |

## Review later

Open `src/bin/revisit.rs`.

Try these a few days later without copying earlier solutions. Continue to the next topic now if you prefer; return here for review.

1. Implement private `stats::sorted_copy(v)`. Return a sorted copy in ascending order, preserving the input. Assume finite values. Empty input returns an empty vector.

   Check: `python3 scripts/check.py 13_modules_visibility revisit sorted_copy`.

2. Implement public `stats::mean(v)`. Return Some(sum / count) for a nonempty slice, otherwise None. Assume finite values.

   Check: `python3 scripts/check.py 13_modules_visibility revisit mean`.

3. Implement public `stats::median(v)`. Sort a copy. For an odd count return the middle value; for an even count return the average of the two middle values. Empty input returns None.

   Check: `python3 scripts/check.py 13_modules_visibility revisit median`.

4. Implement public `stats::mode(v)`. Return the most frequent value. Break ties by choosing the smallest value. Empty input returns None.

   Check: `python3 scripts/check.py 13_modules_visibility revisit mode`.

After all steps: `cargo run -p ex_13_modules_visibility --bin revisit`.

Main example (expressions → expected results):

| Expression | Expected result |
| --- | --- |
| `stats::mean(&[1.0,2.0,3.0,4.0,5.0])` | `Some(3.0)` |
| `stats::median(&[1.0,2.0,3.0,4.0,5.0])` | `Some(3.0)` |
| `stats::median(&[1.0,2.0,3.0,4.0])` | `Some(2.5)` |
| `stats::mode(&[1,2,2,3,3,3])` | `Some(3)` |
| `stats::mean(&[])` | `None` |

Next topic: [14_writing_tests](../14_writing_tests/README.md). Move on after Write, Predict, Repair, and Extend; keep Review later for another session.
