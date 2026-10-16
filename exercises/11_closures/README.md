# 11 — Closures

A closure is an anonymous function that can capture variables from the surrounding scope. `|x| x + 1` is a closure. Closures implement `Fn`, `FnMut`, or `FnOnce` depending on how they use captured values. Use `move` to take ownership of captured variables instead of borrowing.

Complete one step at a time. Run checks from the repository root. Do not change supplied checks to make an incorrect answer pass.

## Write

Open `src/bin/recall.rs`.

1. Implement `apply_twice(f: impl Fn(i32) -> i32, x: i32) -> i32`. Call `f` on `x`, then call `f` on the result. `apply_twice(|x| x + 10, 3)` → `23`.

   Check: `python3 scripts/check.py 11_closures recall apply_twice`

2. Implement `apply_to_each(v: &[i32], f: impl Fn(i32) -> i32) -> Vec<i32>`. Apply `f` to each element and return the results in a new `Vec`. Leave the input unchanged.

   Check: `python3 scripts/check.py 11_closures recall apply_to_each`

Run: `cargo run -p ex_11_closures --bin recall`

| Expression | Expected |
| --- | --- |
| `apply_twice(\|x\| x + 10, 3)` | `23` |
| `apply_to_each(&[1,2,3,4,5], \|x\| x * x)` | `[1, 4, 9, 16, 25]` |
| `apply_to_each(&[1,2,3,4,5], \|x\| x + n)` | `[6, 7, 8, 9, 10]` |

## Predict

Open `src/bin/predict.rs`.

Predict every printed line. Explain how `add_x` captures `x`, how `increment` borrows `count` mutably, and why `greet` can run twice even though it captures `s` by move.

Run: `cargo run -p ex_11_closures --bin predict`

## Repair

Open `src/bin/repair.rs`.

Capture `n` by value in `make_adder`. Call the consuming closure only once. Print `count` only after `inc`'s final call. Allow `apply` to accept a closure that mutates captured state.

Run: `cargo run -p ex_11_closures --bin repair`

## Extend

Open `src/bin/extend.rs`.

1. Reimplement `apply_twice` and `apply_to_each` from recall.

2. Implement `compose`. It takes two functions `f` and `g` and returns a new closure that applies `g` first, then `f`: `compose(f, g)(x) == f(g(x))`. The stub already has `move |x|` — fill in the body.

   Check: `python3 scripts/check.py 11_closures extend compose`

Run: `cargo run -p ex_11_closures --bin extend`

| Expression | Expected |
| --- | --- |
| `apply_to_each(&[1,2,3,4,5], double_then_add1)` | `[3, 5, 7, 9, 11]` |

## Review later

Open `src/bin/revisit.rs`. Try these a few days later without copying earlier solutions.

The stubs for `make_adder` and `make_multiplier` already have `move |x|` — fill in the body.

1. `make_adder(n: i32) -> impl Fn(i32) -> i32` — return a closure that adds `n` to its argument.

   Check: `python3 scripts/check.py 11_closures revisit make_adder`

2. `make_multiplier(n: i32) -> impl Fn(i32) -> i32` — return a closure that multiplies by `n`.

   Check: `python3 scripts/check.py 11_closures revisit make_multiplier`

3. `pipeline(v: &[i32], ops: &[&dyn Fn(i32) -> i32]) -> Vec<i32>` — for each value, apply every op in order and collect results. No ops returns a copy of `v`.

   Check: `python3 scripts/check.py 11_closures revisit pipeline`

Run: `cargo run -p ex_11_closures --bin revisit`

| Expression | Expected |
| --- | --- |
| `add5(10)` | `15` |
| `mul3(4)` | `12` |
| `pipeline(&[1,2,3,4,5], &ops)` | `[18, 21, 24, 27, 30]` |

Next: [12_traits_generics](../12_traits_generics/README.md). Move on after Write, Predict, Repair, and Extend. Return here for Review later in a separate session.
