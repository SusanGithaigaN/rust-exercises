# 07 — Enums and match

An enum names a fixed set of variants. `match` forces you to handle every one — the compiler rejects a match that misses a variant. Data can be attached to variants: `Shape::Circle(f64)` carries a radius, `Shape::Triangle(f64, f64, f64)` carries three sides.

Complete one step at a time. Run checks from the repository root. Do not change supplied checks to make an incorrect answer pass.

## Write

Open `src/bin/recall.rs`.

1. Implement `opposite(d: Direction) -> Direction`. North↔South, East↔West.

   Check: `python3 scripts/check.py 07_enums_match recall opposite`

2. Implement `is_horizontal(d: &Direction) -> bool`. True for East and West.

   Check: `python3 scripts/check.py 07_enums_match recall is_horizontal`

3. Implement `describe(d: &Direction) -> &'static str`. Return `"heading north"`, `"heading south"`, `"heading east"`, or `"heading west"`.

   Check: `python3 scripts/check.py 07_enums_match recall describe`

4. Implement `value_in_cents(coin: Coin) -> u32`. Penny=1, Nickel=5, Dime=10, Quarter=25.

   Check: `python3 scripts/check.py 07_enums_match recall value_in_cents`

Run: `cargo run -p ex_07_enums_match --bin recall`

| Expression | Expected |
| --- | --- |
| `opposite(Direction::North)` | `South` |
| `opposite(Direction::East)` | `West` |
| `is_horizontal(&Direction::East)` | `true` |
| `is_horizontal(&Direction::North)` | `false` |
| `describe(&Direction::South)` | `heading south` |
| `value_in_cents(Coin::Quarter)` | `25` |

## Predict

Open `src/bin/predict.rs`.

Predict every printed line. Explain why all four variants must be covered, how `if let` handles only one variant, and which match arms extract data.

Run: `cargo run -p ex_07_enums_match --bin predict`

## Repair

Open `src/bin/repair.rs`.

Add the missing `Season::Winter` arm returning `"cold"`. Fix the `Shape` match to use tuple-variant syntax, not struct syntax. Make the `Circle` constructor an associated function.

Run: `cargo run -p ex_07_enums_match --bin repair`

## Extend

Open `src/bin/extend.rs`.

1. Reimplement `opposite`, `is_horizontal`, `describe`, and `value_in_cents` from recall.

2. Implement `turn_right(d: Direction) -> Direction`. Clockwise: North→East, East→South, South→West, West→North.

   Check: `python3 scripts/check.py 07_enums_match extend turn_right`

3. Implement `duration_seconds(light: &TrafficLight) -> u32`. Red=30, Yellow=5, Green=25.

   Check: `python3 scripts/check.py 07_enums_match extend duration_seconds`

4. Implement `next_light(light: TrafficLight) -> TrafficLight`. Sequence: Red→Green→Yellow→Red.

   Check: `python3 scripts/check.py 07_enums_match extend next_light`

Run: `cargo run -p ex_07_enums_match --bin extend`

| Expression | Expected |
| --- | --- |
| `turn_right(Direction::North)` | `East` |
| `turn_right(Direction::West)` | `North` |
| `duration_seconds(&TrafficLight::Red)` | `30` |
| `next_light(TrafficLight::Red)` | `Green` |
| `next_light(TrafficLight::Green)` | `Yellow` |

## Review later

Open `src/bin/revisit.rs`. Try these a few days later without copying earlier solutions.

The `Shape` enum carries data: `Circle(f64)` is a radius, `Rectangle(f64, f64)` is width and height, `Triangle(f64, f64, f64)` is three sides.

1. `area(shape: &Shape) -> f64`. Circle: `π×r²`. Rectangle: `w×h`. Triangle: Heron's formula.

   Check: `python3 scripts/check.py 07_enums_match revisit area`

2. `describe(shape: &Shape) -> String`. Return `"Circle with radius 3"`, `"Rectangle 4 x 6"`, or `"Triangle with sides 3, 4, 5"`. Use plain number formatting.

   Check: `python3 scripts/check.py 07_enums_match revisit describe`

3. `is_valid(shape: &Shape) -> bool`. Circle: radius > 0. Rectangle: both sides > 0. Triangle: each pair of sides must sum to **strictly more** than the third side.

   Check: `python3 scripts/check.py 07_enums_match revisit is_valid`

Run: `cargo run -p ex_07_enums_match --bin revisit`

| Expression | Expected |
| --- | --- |
| `area(&Shape::Circle(3.0))` | `28.27...` |
| `area(&Shape::Rectangle(4.0, 6.0))` | `24` |
| `area(&Shape::Triangle(3.0, 4.0, 5.0))` | `6` |
| `is_valid(&Shape::Triangle(1.0, 2.0, 10.0))` | `false` |

Next: [08_option](../08_option/README.md). Move on after Write, Predict, Repair, and Extend. Return here for Review later in a separate session.
