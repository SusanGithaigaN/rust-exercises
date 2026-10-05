# 07 — Enums and pattern matching

Work on one numbered step at a time. Write your code in `src/bin/`; the supplied checks describe expected behaviour and are not solutions.

Run commands from the repository root. A function check runs only that step, so other `todo!()` bodies can remain unfinished. Do not change the supplied checks to make an incorrect answer pass.

The checks cover the listed examples; also explain your result and try one new input before recording a task as done.

## Write

Open `src/bin/recall.rs`.

1. Implement `opposite(d)`. Map North ↔ South and East ↔ West using `match`.

   Check: `python3 scripts/check.py 07_enums_match recall opposite`.

2. Implement `is_horizontal(&d)`. Return true for East and West, false for North and South. Borrow the direction.

   Check: `python3 scripts/check.py 07_enums_match recall is_horizontal`.

3. Implement `describe(&d)`. Return `"heading north"`, `"heading south"`, `"heading east"`, or `"heading west"` for the corresponding variant.

   Check: `python3 scripts/check.py 07_enums_match recall describe`.

4. Implement `value_in_cents(coin)`. Penny = 1, Nickel = 5, Dime = 10, Quarter = 25.

   Check: `python3 scripts/check.py 07_enums_match recall value_in_cents`.

After all steps: `cargo run -p ex_07_enums_match --bin recall`.

Main example (expressions → expected results):

| Expression | Expected result |
| --- | --- |
| `opposite(Direction::North)` | `South` |
| `opposite(Direction::East)` | `West` |
| `is_horizontal(&Direction::East)` | `true` |
| `is_horizontal(&Direction::North)` | `false` |
| `describe(&Direction::South)` | `heading south` |
| `value_in_cents(Coin::Quarter)` | `25` |
| `value_in_cents(Coin::Penny)` | `1` |

## Predict

Open `src/bin/predict.rs`.

Predict every call to process. Identify which match patterns extract data from variants. Explain why all four variants must be covered and how if let handles only the Write variant.

Run: `cargo run -p ex_07_enums_match --bin predict`.

Done when you have compared the output with your prediction and explained each difference.

## Repair

Open `src/bin/repair.rs`.

Add a Winter branch returning "cold". Match Circle and Rectangle using their tuple-variant shapes. Qualify the Circle constructor with its enum name. The circle of radius 3 must have area π × 9.

Run: `cargo run -p ex_07_enums_match --bin repair`.

Done when the program compiles, runs without an unexpected panic, and meets the results above. Explain the cause of each error. Some fixes address logic errors or warnings, not compiler errors.

## Extend

Open `src/bin/extend.rs`.

1. Reimplement from recall: `opposite(d)`. Map North ↔ South and East ↔ West using `match`.

   Check: `python3 scripts/check.py 07_enums_match extend opposite`.

2. Reimplement from recall: `is_horizontal(&d)`. Return true for East and West, false for North and South. Borrow the direction.

   Check: `python3 scripts/check.py 07_enums_match extend is_horizontal`.

3. Reimplement from recall: `describe(&d)`. Return `"heading north"`, `"heading south"`, `"heading east"`, or `"heading west"` for the corresponding variant.

   Check: `python3 scripts/check.py 07_enums_match extend describe`.

4. Reimplement from recall: `value_in_cents(coin)`. Penny = 1, Nickel = 5, Dime = 10, Quarter = 25.

   Check: `python3 scripts/check.py 07_enums_match extend value_in_cents`.

5. Implement `turn_right(d)`. Rotate clockwise: North → East → South → West → North.

   Check: `python3 scripts/check.py 07_enums_match extend turn_right`.

6. Implement `duration_seconds(&light)`. Red lasts 30 seconds, Green 45 seconds, and Yellow 5 seconds.

   Check: `python3 scripts/check.py 07_enums_match extend duration_seconds`.

7. Implement `next_light(light)`. Cycle Red → Green → Yellow → Red.

   Check: `python3 scripts/check.py 07_enums_match extend next_light`.

After all steps: `cargo run -p ex_07_enums_match --bin extend`.

Main example (expressions → expected results):

| Expression | Expected result |
| --- | --- |
| `opposite(Direction::North)` | `South` |
| `is_horizontal(&Direction::East)` | `true` |
| `turn_right(Direction::North)` | `East` |
| `turn_right(Direction::West)` | `North` |
| `duration_seconds(&TrafficLight::Red)` | `30` |
| `next_light(TrafficLight::Red)` | `Green` |
| `next_light(TrafficLight::Green)` | `Yellow` |

## Review later

Open `src/bin/revisit.rs`.

Try these a few days later without copying earlier solutions. Continue to the next topic now if you prefer; return here for review.

1. Implement `area(&shape)`. Circle: πr². Rectangle: width × height. Triangle: Heron’s formula, where `s = (a+b+c)/2` and area = √(s(s−a)(s−b)(s−c)). Call area only for valid shapes.

   Check: `python3 scripts/check.py 07_enums_match revisit area`.

2. Implement `describe(&shape)`. Return `"Circle with radius R"`, `"Rectangle W x H"`, or `"Triangle with sides A, B, C"`. Use ordinary number formatting.

   Check: `python3 scripts/check.py 07_enums_match revisit describe`.

3. Implement `is_valid(&shape)`. Require positive finite dimensions. For triangles, also require each pair of sides to sum to more than the third side. Equality is invalid.

   Check: `python3 scripts/check.py 07_enums_match revisit is_valid`.

After all steps: `cargo run -p ex_07_enums_match --bin revisit`.

Next topic: [08_option](../08_option/README.md). Move on after Write, Predict, Repair, and Extend; keep Review later for another session.
