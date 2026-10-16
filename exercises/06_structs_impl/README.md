# 06 — Structs and methods

A struct groups related data into a named type. Methods are functions tied to a struct — they either borrow the struct (`&self`), borrow it mutably (`&mut self`), or consume it (`self`). Associated functions like `new` live in the `impl` block but don't take `self`.

Complete one step at a time. Run checks from the repository root. Do not change supplied checks to make an incorrect answer pass.

## Write

Open `src/bin/recall.rs`.

1. Implement `Rectangle::new(width, height)`. Store both dimensions and return the rectangle.

   Check: `python3 scripts/check.py 06_structs_impl recall new`

2. Implement `Rectangle::area(&self)`. Return `width × height`.

   Check: `python3 scripts/check.py 06_structs_impl recall area`

3. Implement `Rectangle::perimeter(&self)`. Return `2 × (width + height)`.

   Check: `python3 scripts/check.py 06_structs_impl recall perimeter`

4. Implement `Rectangle::is_square(&self)`. Return true when `width == height`.

   Check: `python3 scripts/check.py 06_structs_impl recall is_square`

5. Implement `Rectangle::scale(&self, factor)`. Return a new rectangle with both dimensions multiplied by `factor`. Leave the original unchanged.

   Check: `python3 scripts/check.py 06_structs_impl recall scale`

Run: `cargo run -p ex_06_structs_impl --bin recall`

| Expression | Expected |
| --- | --- |
| `r` | `Rectangle { width: 4.0, height: 6.0 }` |
| `r.area()` | `24` |
| `r.perimeter()` | `20` |
| `r.is_square()` | `false` |
| `r.scale(2.0)` | `Rectangle { width: 8.0, height: 12.0 }` |
| `sq.is_square()` | `true` |

## Predict

Open `src/bin/predict.rs`.

Predict every printed line. Explain `Debug` formatting, the difference between an associated function and an instance method, and which methods mutate or borrow.

Run: `cargo run -p ex_06_structs_impl --bin predict`

## Repair

Open `src/bin/repair.rs`.

Make `Circle::new(5.0)` an associated constructor. Make `area` borrow the circle and compute `π × r²`. Make `describe` return `"Circle with radius 5"` using the stored field. Both calls must work on the same circle.

Run: `cargo run -p ex_06_structs_impl --bin repair`

## Extend

Open `src/bin/extend.rs`.

1. Reimplement all five Rectangle methods from recall.

   Checks: `python3 scripts/check.py 06_structs_impl extend new` (and `area`, `perimeter`, `is_square`, `scale`)

2. Implement `Rectangle::can_contain(&self, other)`. Return true only if both dimensions of `self` are **strictly** larger than the corresponding dimensions of `other`. Equal sizes do not fit.

   Check: `python3 scripts/check.py 06_structs_impl extend can_contain`

3. Implement `Circle::new`, `Circle::area`, and `Circle::circumference`. Use `std::f64::consts::PI`. Formulas: `π × r²` and `2 × π × r`.

   Check: `python3 scripts/check.py 06_structs_impl extend circle`

Run: `cargo run -p ex_06_structs_impl --bin extend`

| Expression | Expected |
| --- | --- |
| `big.can_contain(&small)` | `true` |
| `small.can_contain(&big)` | `false` |
| `c.area()` | `28.274333882308138` |
| `c.circumference()` | `18.84955592153876` |

## Review later

Open `src/bin/revisit.rs`. Try these a few days later without copying earlier solutions.

1. `BankAccount::new(owner, initial_balance)` — store owner name and balance.
2. `deposit(&mut self, amount)` — add to balance.
3. `withdraw(&mut self, amount) -> bool` — subtract if funds are sufficient; return false otherwise.
4. `balance(&self) -> f64` — return current balance.
5. `statement(&self) -> String` — return `"Alice: $70.00"` (two decimal places).

   Checks: `python3 scripts/check.py 06_structs_impl revisit new` (and `deposit`, `withdraw`, `balance`, `statement`)

Run: `cargo run -p ex_06_structs_impl --bin revisit`

| Expression | Expected |
| --- | --- |
| `acc.statement()` | `Alice: $100.00` |
| `acc.withdraw(200.0)` | `false` |
| `acc.withdraw(80.0)` | `true` |
| `acc.statement()` | `Alice: $70.00` |

Next: [07_enums_match](../07_enums_match/README.md). Move on after Write, Predict, Repair, and Extend. Return here for Review later in a separate session.
