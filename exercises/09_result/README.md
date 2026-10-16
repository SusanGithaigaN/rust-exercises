# 09 — Result

`Result<T, E>` is either `Ok(value)` or `Err(reason)`. Use it when an operation can fail with a meaningful error. The `?` operator propagates an `Err` automatically — instead of writing `match`, you write `let x = something()?;` and the function returns early on failure.

Complete one step at a time. Run checks from the repository root. Do not change supplied checks to make an incorrect answer pass.

## Write

Open `src/bin/recall.rs`.

1. Implement `parse_score(s: &str) -> Result<u32, ScoreError>`. Empty text → `EmptyInput`. Non-numeric text → `NotANumber`. Values above 100 → `OutOfRange`. Accept 0–100 inclusive. Do not trim whitespace.

   Check: `python3 scripts/check.py 09_result recall parse_score`

Run: `cargo run -p ex_09_result --bin recall`

| Expression | Expected |
| --- | --- |
| `parse_score("")` | `Err(EmptyInput)` |
| `parse_score("abc")` | `Err(NotANumber)` |
| `parse_score("150")` | `Err(OutOfRange)` |
| `parse_score("85")` | `Ok(85)` |

## Predict

Open `src/bin/predict.rs`.

Predict every printed line. Explain how `?` propagates an error, and how `.map` and `.unwrap_or` handle success and failure.

Run: `cargo run -p ex_09_result --bin predict`

## Repair

Open `src/bin/repair.rs`.

Give `print_doubled` a `Result` return type compatible with `?`. Handle the invalid parse in `main` without panicking, using 0 as the fallback. Fix `unwrap_or` to use a value. Handle the result of `print_doubled("7")`.

Run: `cargo run -p ex_09_result --bin repair`

## Extend

Open `src/bin/extend.rs`.

1. Reimplement `parse_score` from recall.

2. Implement `parse_and_grade(s: &str) -> Result<&'static str, ScoreError>`. Call `parse_score` with `?`, then map the score: ≥90 → `"A"`, ≥80 → `"B"`, ≥70 → `"C"`, ≥60 → `"D"`, else `"F"`.

   Check: `python3 scripts/check.py 09_result extend parse_and_grade`

Run: `cargo run -p ex_09_result --bin extend`

| Expression | Expected |
| --- | --- |
| `parse_score("85")` | `Ok(85)` |
| `parse_score("abc")` | `Err(NotANumber)` |
| `parse_and_grade("95")` | `Ok("A")` |
| `parse_and_grade("72")` | `Ok("C")` |
| `parse_and_grade("55")` | `Ok("F")` |
| `parse_and_grade("abc")` | `Err(NotANumber)` |

## Review later

Open `src/bin/revisit.rs`. Try these a few days later without copying earlier solutions.

1. `validate_name(name: &str) -> Result<&str, ValidationError>`. Empty → `NameEmpty`. Over 50 Unicode characters → `NameTooLong`. Do not trim input.

   Check: `python3 scripts/check.py 09_result revisit validate_name`

2. `validate_age(age_str: &str) -> Result<u32, ValidationError>`. Parse as u32 without trimming. Parse failure → `AgeNotANumber`. Below 18 → `AgeTooYoung`. Above 120 → `AgeTooOld`. Accept 18–120 inclusive.

   Check: `python3 scripts/check.py 09_result revisit validate_age`

3. `create_profile(name: &str, age_str: &str) -> Result<String, ValidationError>`. Validate name first, then age, using `?`. Return `"Name: Alice, Age: 30"` on success.

   Check: `python3 scripts/check.py 09_result revisit create_profile`

Run: `cargo run -p ex_09_result --bin revisit`

| Expression | Expected |
| --- | --- |
| `create_profile("Alice", "30")` | `Ok("Name: Alice, Age: 30")` |
| `create_profile("", "30")` | `Err(NameEmpty)` |
| `create_profile("Alice", "abc")` | `Err(AgeNotANumber)` |
| `create_profile("Alice", "15")` | `Err(AgeTooYoung)` |

Next: [10_vec_iteration](../10_vec_iteration/README.md). Move on after Write, Predict, Repair, and Extend. Return here for Review later in a separate session.
