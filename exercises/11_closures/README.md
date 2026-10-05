# 11 — Closures

Work on one numbered step at a time. Write your code in `src/bin/`; the supplied checks describe expected behaviour and are not solutions.

Run commands from the repository root. A function check runs only that step, so other `todo!()` bodies can remain unfinished. Do not change the supplied checks to make an incorrect answer pass.

The checks cover the listed examples; also explain your result and try one new input before recording a task as done.

## Write

Open `src/bin/recall.rs`.

1. Implement `apply_twice(f, x)`. Apply `f` to x, then apply it to that result. Adding 10 twice to 3 returns 23. In main, create a closure named `add_ten` and pass it to this function.

   Check: `python3 scripts/check.py 11_closures recall apply_twice`.

2. Implement `apply_to_each(v, f)`. Apply the closure to each input and collect a new vector in order. Leave the input unchanged. Empty input stays empty.

   Check: `python3 scripts/check.py 11_closures recall apply_to_each`.

After all steps: `cargo run -p ex_11_closures --bin recall`.

Main example (expressions → expected results):

| Expression | Expected result |
| --- | --- |
| `apply_twice(\|x\| x + 10, 3)` | `23` |
| `apply_to_each(&[1,2,3,4,5], \|x\| x * x)` | `[1, 4, 9, 16, 25]` |
| `apply_to_each(&[1,2,3,4,5], \|x\| x + n)` | `[6, 7, 8, 9, 10]` |

## Predict

Open `src/bin/predict.rs`.

Predict every printed line. Explain how add_x captures x, how increment borrows count mutably, and why count can be printed after increment’s final use. Explain why greet can run twice although it captures s by move.

Run: `cargo run -p ex_11_closures --bin predict`.

Done when you have compared the output with your prediction and explained each difference.

## Repair

Open `src/bin/repair.rs`.

Capture n by value in make_adder. Call the consuming closure once. Print count only after inc’s final call. Allow apply to accept and call a closure that mutates captured state. Preserve the accumulated result of 5.

Run: `cargo run -p ex_11_closures --bin repair`.

Done when the program compiles, runs without an unexpected panic, and meets the results above. Explain the cause of each error. Some fixes address logic errors or warnings, not compiler errors.

## Extend

Open `src/bin/extend.rs`.

1. Reimplement from recall: `apply_twice(f, x)`. Apply `f` to x, then apply it to that result. Adding 10 twice to 3 returns 23. In main, create a closure named `add_ten` and pass it to this function.

   Check: `python3 scripts/check.py 11_closures extend apply_twice`.

2. Reimplement from recall: `apply_to_each(v, f)`. Apply the closure to each input and collect a new vector in order. Leave the input unchanged. Empty input stays empty.

   Check: `python3 scripts/check.py 11_closures extend apply_to_each`.

3. Implement `compose(f, g)`. Return a closure that applies g first and f second: `f(g(x))`. Capture both functions by value using the provided `move` closure.

   Check: `python3 scripts/check.py 11_closures extend compose`.

After all steps: `cargo run -p ex_11_closures --bin extend`.

Main example (expressions → expected results):

| Expression | Expected result |
| --- | --- |
| `apply_twice(\|x\| x + 10, 3)` | `23` |
| `apply_to_each(&[1,2,3,4,5], \|x\| x * x)` | `[1, 4, 9, 16, 25]` |
| `apply_to_each(&[1,2,3,4,5], double_then_add1)` | `[3, 5, 7, 9, 11]` |

## Review later

Open `src/bin/revisit.rs`.

Try these a few days later without copying earlier solutions. Continue to the next topic now if you prefer; return here for review.

1. Implement `make_adder(n)`. Return a closure that adds captured n to its argument. It must still work after the function returns.

   Check: `python3 scripts/check.py 11_closures revisit make_adder`.

2. Implement `make_multiplier(n)`. Return a closure that multiplies its argument by captured n.

   Check: `python3 scripts/check.py 11_closures revisit make_multiplier`.

3. Implement `pipeline(v, ops)`. For each value, apply the operations in slice order and collect the results. No operations returns a copy of v; empty input stays empty.

   Check: `python3 scripts/check.py 11_closures revisit pipeline`.

After all steps: `cargo run -p ex_11_closures --bin revisit`.

Main example (expressions → expected results):

| Expression | Expected result |
| --- | --- |
| `add5(10)` | `15` |
| `mul3(4)` | `12` |
| `pipeline(&[1,2,3,4,5], &ops)` | `[18, 21, 24, 27, 30]` |

Next topic: [12_traits_generics](../12_traits_generics/README.md). Move on after Write, Predict, Repair, and Extend; keep Review later for another session.
