# 09 — Result and error handling

Work on one numbered step at a time. Write your code in `src/bin/`; the supplied checks describe expected behaviour and are not solutions.

Run commands from the repository root. A function check runs only that step, so other `todo!()` bodies can remain unfinished. Do not change the supplied checks to make an incorrect answer pass.

The checks cover the listed examples; also explain your result and try one new input before recording a task as done.

## Write

Open `src/bin/recall.rs`.

1. Implement `parse_score(s) -> Result<u32, ScoreError>`. Empty text returns EmptyInput. Text that cannot parse as u32 returns NotANumber. Parsed values above 100 return OutOfRange. Accept 0 through 100 inclusive. Do not trim whitespace.

   Check: `python3 scripts/check.py 09_result recall parse_score`.

After all steps: `cargo run -p ex_09_result --bin recall`.

Main example (expressions → expected results):

| Expression | Expected result |
| --- | --- |
| `parse_score("")` | `Err(EmptyInput)` |
| `parse_score("abc")` | `Err(NotANumber)` |
| `parse_score("150")` | `Err(OutOfRange)` |
| `parse_score("85")` | `Ok(85)` |

## Predict

Open `src/bin/predict.rs`.

Predict every printed line. Explain how ? propagates an error and how map and unwrap_or handle success and failure.

Run: `cargo run -p ex_09_result --bin predict`.

Done when you have compared the output with your prediction and explained each difference.

## Repair

Open `src/bin/repair.rs`.

Give print_doubled a Result return type compatible with ?. Handle the invalid parse in main without panicking, using 0 as the fallback. Use or explicitly handle the result of parse_and_double("5"). Fix unwrap_or to use a value, and handle print_doubled("7")’s result.

Run: `cargo run -p ex_09_result --bin repair`.

Done when the program compiles, runs without an unexpected panic, and meets the results above. Explain the cause of each error. Some fixes address logic errors or warnings, not compiler errors.

## Extend

Open `src/bin/extend.rs`.

1. Reimplement from recall: `parse_score(s) -> Result<u32, ScoreError>`. Empty text returns EmptyInput. Text that cannot parse as u32 returns NotANumber. Parsed values above 100 return OutOfRange. Accept 0 through 100 inclusive. Do not trim whitespace.

   Check: `python3 scripts/check.py 09_result extend parse_score`.

2. Implement `parse_and_grade(s)`. Call `parse_score` using `?` to propagate errors. Return A for 90–100, B for 80–89, C for 70–79, D for 60–69, F for 0–59.

   Check: `python3 scripts/check.py 09_result extend parse_and_grade`.

After all steps: `cargo run -p ex_09_result --bin extend`.

Main example (expressions → expected results):

| Expression | Expected result |
| --- | --- |
| `parse_score("85")` | `Ok(85)` |
| `parse_score("abc")` | `Err(NotANumber)` |
| `parse_and_grade("95")` | `Ok("A")` |
| `parse_and_grade("72")` | `Ok("C")` |
| `parse_and_grade("55")` | `Ok("F")` |
| `parse_and_grade("abc")` | `Err(NotANumber)` |

## Review later

Open `src/bin/revisit.rs`.

Try these a few days later without copying earlier solutions. Continue to the next topic now if you prefer; return here for review.

1. Implement `validate_name(name)`. Empty input returns NameEmpty; more than 50 Unicode characters returns NameTooLong. Otherwise return the original borrowed name. Do not trim input.

   Check: `python3 scripts/check.py 09_result revisit validate_name`.

2. Implement `validate_age(age_str)`. Parse as u32 without trimming. Parse failures return AgeNotANumber. Values below 18 return AgeTooYoung; above 120 return AgeTooOld. Accept 18 through 120 inclusive.

   Check: `python3 scripts/check.py 09_result revisit validate_age`.

3. Implement `create_profile(name, age_str)`. Validate the name first, then the age, using `?`. Return `"Name: name, Age: age"` on success. If both inputs are invalid, return the name error first.

   Check: `python3 scripts/check.py 09_result revisit create_profile`.

After all steps: `cargo run -p ex_09_result --bin revisit`.

Main example (expressions → expected results):

| Expression | Expected result |
| --- | --- |
| `create_profile("Alice", "30")` | `Ok("Name: Alice, Age: 30")` |
| `create_profile("", "30")` | `Err(NameEmpty)` |
| `create_profile("Alice", "abc")` | `Err(AgeNotANumber)` |
| `create_profile("Alice", "15")` | `Err(AgeTooYoung)` |

Next topic: [10_vec_iteration](../10_vec_iteration/README.md). Move on after Write, Predict, Repair, and Extend; keep Review later for another session.
