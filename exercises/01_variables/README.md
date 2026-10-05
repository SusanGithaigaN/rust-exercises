# 01 — Variables, functions, and control flow

Work on one numbered step at a time. Write your code in `src/bin/`; the supplied checks describe expected behaviour and are not solutions.

Run commands from the repository root. A function check runs only that step, so other `todo!()` bodies can remain unfinished. Do not change the supplied checks to make an incorrect answer pass.

The checks cover the listed examples; also explain your result and try one new input before recording a task as done.

## Variables

Open `src/bin/variables.rs`.

1. In main, create a mutable variable named `a` with type `i32` and value 5.
2. Add 3 to a. Print its value on its own line: `8`.
3. Create an immutable variable named `name` with value `"Rust"`. Print it on the next line: `Rust`.
4. Create an immutable boolean named `is_learning` with value true. Print it on the next line: `true`.

Check: `python3 scripts/check.py 01_variables variables`.

Done when the three lines match and you can explain why a needs mut.

## Shadowing

Open `src/bin/shadowing.rs`.

1. Create an immutable variable named `a` with value 5.
2. Shadow it with another `let a` whose value is the previous a plus 1. Print `6`.
3. Create `spaces` with value `"   "` (three spaces). Shadow it with its length. Print `3`.
4. Explain how shadowing differs from assigning to a mutable variable.

Check: `python3 scripts/check.py 01_variables shadowing`.

## Write

A function receives inputs called parameters and returns a result. In the supplied
signatures, `i32` is an integer type, `f64` is a floating point type, and the type
after `->` is the return type. `&'static str` here means returning a string literal
such as `"positive"`. Replace `todo!()` with your function body.

Open `src/bin/recall.rs`.

1. Implement `double(n: i32) -> i32`. Return twice `n`. For example, `5 → 10`, `0 → 0`, and `-3 → -6`. Use small numbers; overflow handling is outside this exercise.

   Check: `python3 scripts/check.py 01_variables recall double`.

2. Implement `describe_sign(n: i32) -> &'static str`. Return exactly `"positive"` when `n > 0`, `"negative"` when `n < 0`, and `"zero"` otherwise.

   Check: `python3 scripts/check.py 01_variables recall describe_sign`.

3. Implement `celsius_to_fahrenheit(c: f64) -> f64`. Return `(c × 9 / 5) + 32`. Use floating point arithmetic. `0 → 32`, `100 → 212`, and `-40 → -40`.

   Check: `python3 scripts/check.py 01_variables recall celsius_to_fahrenheit`.

After all steps: `cargo run -p ex_01_variables --bin recall`.

Main example (expressions → expected results):

| Expression | Expected result |
| --- | --- |
| `double(5)` | `10` |
| `double(-3)` | `-6` |
| `describe_sign(7)` | `positive` |
| `describe_sign(0)` | `zero` |
| `describe_sign(-4)` | `negative` |
| `celsius_to_fahrenheit(0.0)` | `32` |
| `celsius_to_fahrenheit(100.0)` | `212` |

## Loops

A `for` loop repeats its body for each value in a collection or range. An
accumulator is a mutable variable that keeps the running total.

Open `src/bin/loops.rs`.

1. Implement `sum_to(n: u32) -> u32` using a `for` loop. Return the sum of the integers from 1 through `n`, including `n`. `0 → 0`, `1 → 1`, `5 → 15`. Assume `n <= 1000`.

   Check: `python3 scripts/check.py 01_variables loops sum_to`.

After all steps: `cargo run -p ex_01_variables --bin loops`.

## Predict

Open `src/bin/predict.rs`.

Before running, write down every printed line. Explain which x each print uses, why shadowing can change a type, and why count needs mut.

Run: `cargo run -p ex_01_variables --bin predict`.

Done when you have compared the output with your prediction and explained each difference.

## Repair

Open `src/bin/repair.rs`.

Keep the intended results: x = 20; y = 3.14; result = 7; sum = 3; label = "positive". Fix mutability, the numeric type, initialization order, argument count, and label type.

Run: `cargo run -p ex_01_variables --bin repair`.

Done when the program compiles, runs without an unexpected panic, and meets the results above. Explain the cause of each error. Some fixes address logic errors or warnings, not compiler errors.

## Extend

Open `src/bin/extend.rs`.

1. Reimplement from recall: `double(n: i32) -> i32`. Return twice `n`. For example, `5 → 10`, `0 → 0`, and `-3 → -6`. Use small numbers; overflow handling is outside this exercise.

   Check: `python3 scripts/check.py 01_variables extend double`.

2. Reimplement from recall: `describe_sign(n: i32) -> &'static str`. Return exactly `"positive"` when `n > 0`, `"negative"` when `n < 0`, and `"zero"` otherwise.

   Check: `python3 scripts/check.py 01_variables extend describe_sign`.

3. Reimplement from recall: `celsius_to_fahrenheit(c: f64) -> f64`. Return `(c × 9 / 5) + 32`. Use floating point arithmetic. `0 → 32`, `100 → 212`, and `-40 → -40`.

   Check: `python3 scripts/check.py 01_variables extend celsius_to_fahrenheit`.

4. Implement `swap(a, b)`. Return a tuple with `b` first and `a` second. `(3, 7) → (7, 3)`; equal values remain equal.

   Check: `python3 scripts/check.py 01_variables extend swap`.

5. Implement `is_between(value, low, high)`. Return true when `low <= value <= high`, including both endpoints. Assume `low <= high`.

   Check: `python3 scripts/check.py 01_variables extend is_between`.

After all steps: `cargo run -p ex_01_variables --bin extend`.

Main example (expressions → expected results):

| Expression | Expected result |
| --- | --- |
| `double(5)` | `10` |
| `describe_sign(-3)` | `negative` |
| `celsius_to_fahrenheit(100.0)` | `212` |
| `swap(3, 7)` | `(7, 3)` |
| `swap(-1, 5)` | `(5, -1)` |
| `is_between(5, 1, 10)` | `true` |
| `is_between(11, 1, 10)` | `false` |
| `is_between(1, 1, 10)` | `true` |
| `is_between(0, 1, 10)` | `false` |

## Review later

Open `src/bin/revisit.rs`.

Try these a few days later without copying earlier solutions. Continue to the next topic now if you prefer; return here for review.

1. Implement `bmi_category(weight_kg, height_m)`. Calculate `weight_kg / (height_m × height_m)`. Return `"underweight"` below 18.5, `"normal"` from 18.5 to below 25, `"overweight"` from 25 to below 30, and `"obese"` at 30 or above. Assume positive height and weight.

   Check: `python3 scripts/check.py 01_variables revisit bmi_category`.

2. Implement `describe_temp(celsius)`. Return `"freezing"` below 0, `"cold"` from 0 to below 15, `"warm"` from 15 to below 25, and `"hot"` at 25 or above.

   Check: `python3 scripts/check.py 01_variables revisit describe_temp`.

After all steps: `cargo run -p ex_01_variables --bin revisit`.

Main example (expressions → expected results):

| Expression | Expected result |
| --- | --- |
| `bmi_category(70.0, 1.75)` | `normal` |
| `bmi_category(50.0, 1.75)` | `underweight` |
| `bmi_category(100.0, 1.75)` | `obese` |
| `describe_temp(-5.0)` | `freezing` |
| `describe_temp(10.0)` | `cold` |
| `describe_temp(30.0)` | `hot` |

Next topic: [02_ownership_moves](../02_ownership_moves/README.md). Move on after Write, Predict, Repair, and Extend; keep Review later for another session.
