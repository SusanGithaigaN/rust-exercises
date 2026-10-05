# 06 — Structs and methods

Work on one numbered step at a time. Write your code in `src/bin/`; the supplied checks describe expected behaviour and are not solutions.

Run commands from the repository root. A function check runs only that step, so other `todo!()` bodies can remain unfinished. Do not change the supplied checks to make an incorrect answer pass.

The checks cover the listed examples; also explain your result and try one new input before recording a task as done.

## Write

Open `src/bin/recall.rs`.

1. Implement `Rectangle::new(width, height)`. Return a rectangle storing both supplied dimensions. Assume nonnegative finite dimensions.

   Check: `python3 scripts/check.py 06_structs_impl recall new`.

2. Implement `Rectangle::area(&self)`. Return width × height without consuming the rectangle.

   Check: `python3 scripts/check.py 06_structs_impl recall area`.

3. Implement `Rectangle::perimeter(&self)`. Return 2 × (width + height).

   Check: `python3 scripts/check.py 06_structs_impl recall perimeter`.

4. Implement `Rectangle::is_square(&self)`. Return true when width and height are equal. Use exact equality for the supplied dimensions.

   Check: `python3 scripts/check.py 06_structs_impl recall is_square`.

5. Implement `Rectangle::scale(&self, factor)`. Return a new rectangle with both dimensions multiplied by a nonnegative factor. Leave the original unchanged.

   Check: `python3 scripts/check.py 06_structs_impl recall scale`.

After all steps: `cargo run -p ex_06_structs_impl --bin recall`.

Main example (expressions → expected results):

| Expression | Expected result |
| --- | --- |
| `r` | `Rectangle { width: 4.0, height: 6.0 }` |
| `r.area()` | `24` |
| `r.perimeter()` | `20` |
| `r.is_square()` | `false` |
| `r.scale(2.0)` | `Rectangle { width: 8.0, height: 12.0 }` |
| `sq.is_square()` | `true` |

## Predict

Open `src/bin/predict.rs`.

Predict every printed line. Explain Debug formatting, the difference between an associated constructor and an instance method, and which methods mutate or borrow the value.

Run: `cargo run -p ex_06_structs_impl --bin predict`.

Done when you have compared the output with your prediction and explained each difference.

## Repair

Open `src/bin/repair.rs`.

Make Circle::new(5.0) an associated constructor. Make area borrow the circle and compute πr². Make describe return "Circle with radius 5" using an existing field. Both calls must work on the same circle.

Run: `cargo run -p ex_06_structs_impl --bin repair`.

Done when the program compiles, runs without an unexpected panic, and meets the results above. Explain the cause of each error. Some fixes address logic errors or warnings, not compiler errors.

## Extend

Open `src/bin/extend.rs`.

1. Reimplement from recall: `Rectangle::new(width, height)`. Return a rectangle storing both supplied dimensions. Assume nonnegative finite dimensions.

   Check: `python3 scripts/check.py 06_structs_impl extend new`.

2. Reimplement from recall: `Rectangle::area(&self)`. Return width × height without consuming the rectangle.

   Check: `python3 scripts/check.py 06_structs_impl extend area`.

3. Reimplement from recall: `Rectangle::perimeter(&self)`. Return 2 × (width + height).

   Check: `python3 scripts/check.py 06_structs_impl extend perimeter`.

4. Reimplement from recall: `Rectangle::is_square(&self)`. Return true when width and height are equal. Use exact equality for the supplied dimensions.

   Check: `python3 scripts/check.py 06_structs_impl extend is_square`.

5. Reimplement from recall: `Rectangle::scale(&self, factor)`. Return a new rectangle with both dimensions multiplied by a nonnegative factor. Leave the original unchanged.

   Check: `python3 scripts/check.py 06_structs_impl extend scale`.

6. Implement `Rectangle::can_contain(&self, other)`. Return true only if both dimensions are strictly larger than the corresponding dimensions of `other`. Do not rotate rectangles. Equal dimensions do not fit.

   Check: `python3 scripts/check.py 06_structs_impl extend can_contain`.

7. Implement `Circle::new`, `Circle::area`, and `Circle::circumference`. Store the radius; calculate π × radius² and 2 × π × radius. Use `std::f64::consts::PI`. Assume nonnegative radii.

   Check: `python3 scripts/check.py 06_structs_impl extend circle`.

After all steps: `cargo run -p ex_06_structs_impl --bin extend`.

Main example (expressions → expected results):

| Expression | Expected result |
| --- | --- |
| `big.can_contain(&small)` | `true` |
| `small.can_contain(&big)` | `false` |
| `c.area()` | `28.274333882308138` |
| `c.circumference()` | `18.84955592153876` |

## Review later

Open `src/bin/revisit.rs`.

Try these a few days later without copying earlier solutions. Continue to the next topic now if you prefer; return here for review.

1. Implement `BankAccount::new(owner, initial_balance)`. Store an owned copy of the name and the initial balance. Assume a nonnegative finite initial balance.

   Check: `python3 scripts/check.py 06_structs_impl revisit new`.

2. Implement `deposit(&mut self, amount)`. Add positive finite amounts to the balance. Ignore zero and negative amounts.

   Check: `python3 scripts/check.py 06_structs_impl revisit deposit`.

3. Implement `withdraw(&mut self, amount) -> bool`. For positive finite amounts no larger than the balance, subtract the amount and return true. Otherwise return false without changing the balance.

   Check: `python3 scripts/check.py 06_structs_impl revisit withdraw`.

4. Implement `balance(&self) -> f64`. Return the current balance without consuming the account.

   Check: `python3 scripts/check.py 06_structs_impl revisit balance`.

5. Implement `statement(&self) -> String`. Return `"owner: $balance"`, formatting the balance with exactly two decimal places. Example: `"Alice: $70.00"`.

   Check: `python3 scripts/check.py 06_structs_impl revisit statement`.

After all steps: `cargo run -p ex_06_structs_impl --bin revisit`.

Main example (expressions → expected results):

| Expression | Expected result |
| --- | --- |
| `acc.statement()` | `Alice: $100.00` |
| `acc.statement()` | `Alice: $150.00` |
| `acc.withdraw(200.0)` | `false` |
| `acc.withdraw(80.0)` | `true` |
| `acc.statement()` | `Alice: $70.00` |

Next topic: [07_enums_match](../07_enums_match/README.md). Move on after Write, Predict, Repair, and Extend; keep Review later for another session.
