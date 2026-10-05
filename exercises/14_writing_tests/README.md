# 14 — Write your own tests

The functions in this topic are already implemented. Your task is to write the tests. Only read_first has supplied runnable tests.

## How to write a test

A test calls a function with a chosen input and asserts the expected result. `assert_eq!` compares values; `assert!` checks a boolean. Put your tests inside a `#[cfg(test)] mod tests` block and import the file’s functions with `use super::*;`. Each test function needs `#[test]`. The working example in read_first.rs shows this structure.

Use names that describe behaviour, such as `clamp_keeps_a_value_inside_the_bounds`. Write tests for one function at a time; do not change the implemented functions or reduce the test requirements.

A green result with zero tests means no checks ran. The check command below rejects zero tests and checks the minimum count. The count alone does not prove coverage: compare your tests with every listed case.

## read_first

Open `src/bin/read_first.rs`.

Study the worked tests for multiply and is_even. Run them and expect six passing tests. Explain #[cfg(test)], #[test], use super::*, assert_eq!, and assert!. Then proceed to exercise_01.

Check: `python3 scripts/check.py 14_writing_tests read_first`.

Done when all required cases are covered and at least 6 tests pass.

## exercise_01

Open `src/bin/exercise_01.rs`.

Write at least nine tests: three each for double, clamp, and reverse_string. For double check positive, zero, and negative small inputs. For clamp (assume min <= max), check below, inside, and above the bounds, including equality. Reverse strings by Unicode characters: check "hello" → "olleh", "" → "", and "hé" → "éh". Avoid integer overflow inputs.

Check: `python3 scripts/check.py 14_writing_tests exercise_01`.

Done when all required cases are covered and at least 9 tests pass.

## exercise_02

Open `src/bin/exercise_02.rs`.

Write at least eight tests. For find_even test the first of several even values, no even values, negative even values, and empty input. For parse_positive test "5" → Ok(5), "0" → Err("number must be positive"), "" → Err("input is empty"), and "abc" → Err("'abc' is not a valid positive number"). Input is not trimmed; values must fit in u32.

Check: `python3 scripts/check.py 14_writing_tests exercise_02`.

Done when all required cases are covered and at least 8 tests pass.

## exercise_03

Open `src/bin/exercise_03.rs`.

Write at least six tests for Counter, using nonnegative maxima: new(3) starts at 0 and is not full; increment raises the value and returns true; reaching max makes is_full true; another increment returns false and leaves the value unchanged; reset returns to 0 and permits incrementing again; new(0) is already full and cannot increment. Include a sequence of calls in at least one test.

Check: `python3 scripts/check.py 14_writing_tests exercise_03`.

Done when all required cases are covered and at least 6 tests pass.

## exercise_04

Open `src/bin/exercise_04.rs`.

Write at least seven tests: three non-panicking get_element cases (first, last, and a one-element slice), two #[should_panic] cases (empty slice and index equal to length), and two safe_divide cases (10 / 2 → Some(5), division by zero → None). Use expected = "out of bounds" for at least one panic test. Integer division truncates toward zero. Exclude i32::MIN / -1 overflow.

Check: `python3 scripts/check.py 14_writing_tests exercise_04`.

Done when all required cases are covered and at least 7 tests pass.
