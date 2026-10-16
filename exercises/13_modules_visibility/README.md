# 13 — Modules and visibility

`mod name { ... }` creates a module. Items are private by default; `pub` makes them visible to the parent. A `use` statement brings a path into scope. Separate files work the same way: `mod geometry;` in a file looks for `geometry.rs` alongside it, and that file becomes the module body.

Complete one step at a time. Run checks from the repository root. Do not change supplied checks to make an incorrect answer pass.

## Write

Open `src/bin/recall.rs`. The module structure is provided — implement the function bodies.

1. `Point::new(x: f64, y: f64)` — store x and y.
2. `Point::distance(&self, other: &Point) -> f64` — Euclidean distance: `√((x2-x1)² + (y2-y1)²)`.

   Check: `python3 scripts/check.py 13_modules_visibility recall point`

3. `Circle::new(center: Point, radius: f64)` — store center and radius.
4. `Circle::area(&self) -> f64` — `π × radius²`. Use `std::f64::consts::PI`.
5. `Circle::contains(&self, p: &Point) -> bool` — true when the point is strictly inside (distance from center < radius).

   Check: `python3 scripts/check.py 13_modules_visibility recall circle`

Run: `cargo run -p ex_13_modules_visibility --bin recall`

| Expression | Expected |
| --- | --- |
| `origin.distance(&far)` | `5` |
| `c.area()` (radius=10) | `314.1592653589793` |
| `c.contains(&Point::new(3.0, 4.0))` | `true` |
| `c.contains(&Point::new(10.0, 10.0))` | `false` |

## Predict

Open `src/bin/predict.rs`.

Predict every printed line. Explain why `main` cannot read `Config.secret_key`, why a child module can read its parent's private fields, and what `pub mod` vs `mod` means.

Run: `cargo run -p ex_13_modules_visibility --bin predict`

## Repair

Open `src/bin/repair.rs`.

Expose `age` through a public getter, keeping the field private. Make `tricks` accessible through `animals`. Use the correct module path or import for `Dog`.

Run: `cargo run -p ex_13_modules_visibility --bin repair`

## Extend

Open `src/bin/extend.rs`. Split the inline module into separate files.

- Create `src/bin/geometry.rs` with the `Point` struct, its `impl`, and `pub mod shapes;`.
- Create `src/bin/geometry/shapes.rs` with the `Circle` struct and its `impl`.
- This file already declares `mod geometry;` — just create the two missing files.

   Check: `python3 scripts/check.py 13_modules_visibility extend modules`

Run: `cargo run -p ex_13_modules_visibility --bin extend`

| Expression | Expected |
| --- | --- |
| `origin.distance(&far)` | `5` |
| `c.area()` | `314.1592653589793` |
| `c.contains(&Point::new(3.0, 4.0))` | `true` |

## Review later

Open `src/bin/revisit.rs`. Try these a few days later without copying earlier solutions.

The `stats` module is defined inline. Implement the private helper first, then use it in the public functions.

1. `sorted_copy(v: &[f64]) -> Vec<f64>` (private) — return a sorted copy, ascending. Assume finite values.

   Check: `python3 scripts/check.py 13_modules_visibility revisit sorted_copy`

2. `mean(v: &[f64]) -> Option<f64>` — arithmetic mean, `None` if empty.

   Check: `python3 scripts/check.py 13_modules_visibility revisit mean`

3. `median(v: &[f64]) -> Option<f64>` — sort a copy. Odd count → middle value. Even count → average of the two middle values.

   Check: `python3 scripts/check.py 13_modules_visibility revisit median`

4. `mode(v: &[i32]) -> Option<i32>` — most frequent value. Ties → smallest. Empty → `None`.

   Check: `python3 scripts/check.py 13_modules_visibility revisit mode`

Run: `cargo run -p ex_13_modules_visibility --bin revisit`

| Expression | Expected |
| --- | --- |
| `stats::mean(&[1.0,2.0,3.0,4.0,5.0])` | `Some(3.0)` |
| `stats::median(&[1.0,2.0,3.0,4.0,5.0])` | `Some(3.0)` |
| `stats::median(&[1.0,2.0,3.0,4.0])` | `Some(2.5)` |
| `stats::mode(&[1,2,2,3,3,3])` | `Some(3)` |
| `stats::mean(&[])` | `None` |

Next: [14_writing_tests](../14_writing_tests/README.md). Move on after Write, Predict, Repair, and Extend. Return here for Review later in a separate session.
