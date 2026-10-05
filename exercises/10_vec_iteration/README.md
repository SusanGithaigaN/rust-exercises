# 10 — Vectors and iteration

Work on one numbered step at a time. Write your code in `src/bin/`; the supplied checks describe expected behaviour and are not solutions.

Run commands from the repository root. A function check runs only that step, so other `todo!()` bodies can remain unfinished. Do not change the supplied checks to make an incorrect answer pass.

The checks cover the listed examples; also explain your result and try one new input before recording a task as done.

## Before iterator examples

Loops are allowed in the writing exercises. In the prediction and repair examples, `|x| x * 2` is a closure: a small unnamed function with parameter x. `map` transforms elements, `filter` keeps matching elements, and `collect` builds a collection. Topic 11 practises closures in detail.

## Write

Open `src/bin/recall.rs`.

1. Implement `sum(v)`. Return the sum of all values; empty input returns 0. A loop or iterator is allowed.

   Check: `python3 scripts/check.py 10_vec_iteration recall sum`.

2. Implement `max(v) -> Option<&i32>`. Borrow the largest element. Return None for an empty slice.

   Check: `python3 scripts/check.py 10_vec_iteration recall max`.

3. Implement `min(v) -> Option<&i32>`. Borrow the smallest element. Return None for an empty slice.

   Check: `python3 scripts/check.py 10_vec_iteration recall min`.

4. Implement `evens(v)`. Return a new vector of even values in original order, keeping duplicates. Empty input returns an empty vector.

   Check: `python3 scripts/check.py 10_vec_iteration recall evens`.

5. Implement `squares(v)`. Return a new vector containing each input squared, preserving order. Assume small values. Empty input stays empty.

   Check: `python3 scripts/check.py 10_vec_iteration recall squares`.

6. Implement `count_over_four(v)`. Count values strictly greater than 4. Empty input returns 0.

   Check: `python3 scripts/check.py 10_vec_iteration recall count_over_four`.

After all steps: `cargo run -p ex_10_vec_iteration --bin recall`.

Main example (expressions → expected results):

| Expression | Expected result |
| --- | --- |
| `sum(&v)` | `31` |
| `max(&v)` | `Some(9)` |
| `min(&v)` | `Some(1)` |
| `evens(&v)` | `[4, 2, 6]` |
| `squares(&v)` | `[9, 1, 16, 1, 25, 81, 4, 36]` |
| `count_over_four(&v)` | `3` |

## Predict

Open `src/bin/predict.rs`.

First read the short closure introduction below. Predict each printed line. Compare iter, iter_mut, and into_iter. Explain why v remains usable but the commented print of v2 would fail.

Run: `cargo run -p ex_10_vec_iteration --bin predict`.

Done when you have compared the output with your prediction and explained each difference.

## Repair

Open `src/bin/repair.rs`.

Keep v usable by borrowing while doubling. Collect owned values for evens. End the first-element borrow before mutating v3; print the original first value before the mutation. Give the sum a concrete numeric type.

Run: `cargo run -p ex_10_vec_iteration --bin repair`.

Done when the program compiles, runs without an unexpected panic, and meets the results above. Explain the cause of each error. Some fixes address logic errors or warnings, not compiler errors.

## Extend

Open `src/bin/extend.rs`.

1. Reimplement from recall: `sum(v)`. Return the sum of all values; empty input returns 0. A loop or iterator is allowed.

   Check: `python3 scripts/check.py 10_vec_iteration extend sum`.

2. Reimplement from recall: `max(v) -> Option<&i32>`. Borrow the largest element. Return None for an empty slice.

   Check: `python3 scripts/check.py 10_vec_iteration extend max`.

3. Reimplement from recall: `min(v) -> Option<&i32>`. Borrow the smallest element. Return None for an empty slice.

   Check: `python3 scripts/check.py 10_vec_iteration extend min`.

4. Reimplement from recall: `evens(v)`. Return a new vector of even values in original order, keeping duplicates. Empty input returns an empty vector.

   Check: `python3 scripts/check.py 10_vec_iteration extend evens`.

5. Reimplement from recall: `squares(v)`. Return a new vector containing each input squared, preserving order. Assume small values. Empty input stays empty.

   Check: `python3 scripts/check.py 10_vec_iteration extend squares`.

6. Reimplement from recall: `count_over_four(v)`. Count values strictly greater than 4. Empty input returns 0.

   Check: `python3 scripts/check.py 10_vec_iteration extend count_over_four`.

7. Implement `running_totals(v)`. Return cumulative sums in order. `[1, 2, 3] → [1, 3, 6]`; empty input stays empty. Loops are allowed.

   Check: `python3 scripts/check.py 10_vec_iteration extend running_totals`.

8. Implement `flatten_vecs(v)`. Return the contents of each vector in order, keeping duplicates and skipping empty vectors. Do not consume the inputs.

   Check: `python3 scripts/check.py 10_vec_iteration extend flatten_vecs`.

After all steps: `cargo run -p ex_10_vec_iteration --bin extend`.

Main example (expressions → expected results):

| Expression | Expected result |
| --- | --- |
| `sum(&v)` | `31` |
| `evens(&v)` | `[4, 2, 6]` |
| `running_totals(&[1, 2, 3, 4, 5])` | `[1, 3, 6, 10, 15]` |
| `running_totals(&[5, 5, 5])` | `[5, 10, 15]` |
| `flatten_vecs(&[vec![1,2], vec![3], vec![4,5,6]])` | `[1,2,3,4,5,6]` |

## Review later

Open `src/bin/revisit.rs`.

Try these a few days later without copying earlier solutions. Continue to the next topic now if you prefer; return here for review.

1. Implement `longer_than_five(words)`. Count words with more than 5 Unicode characters. Empty input returns 0.

   Check: `python3 scripts/check.py 10_vec_iteration revisit longer_than_five`.

2. Implement `starts_ab(words)`. Return owned copies of words beginning with lowercase a or b, preserving order. Empty strings do not match.

   Check: `python3 scripts/check.py 10_vec_iteration revisit starts_ab`.

3. Implement `longest(words)`. Borrow the longest word by Unicode character count. On ties return the first. Empty input returns `""`. Keep the lifetime signature.

   Check: `python3 scripts/check.py 10_vec_iteration revisit longest`.

4. Implement `joined(words)`. Join all entries with `", "`, preserving empty entries. Empty input returns `""`.

   Check: `python3 scripts/check.py 10_vec_iteration revisit joined`.

5. Implement `first_letters(words)`. Return the first Unicode character of each nonempty word, preserving order. Skip empty strings.

   Check: `python3 scripts/check.py 10_vec_iteration revisit first_letters`.

6. Implement `contains_e(words)`. Count words containing lowercase e at least once; repeated e characters count only once per word.

   Check: `python3 scripts/check.py 10_vec_iteration revisit contains_e`.

After all steps: `cargo run -p ex_10_vec_iteration --bin revisit`.

Main example (expressions → expected results):

| Expression | Expected result |
| --- | --- |
| `longer_than_five(&words)` | `4` |
| `starts_ab(&words)` | `["apple", "banana", "avocado", "blueberry"]` |
| `longest(&words)` | `blueberry` |
| `joined(&words)` | `apple, banana, cherry, avocado, blueberry, fig` |
| `first_letters(&words)` | `['a', 'b', 'c', 'a', 'b', 'f']` |
| `contains_e(&words)` | `3` |

Next topic: [11_closures](../11_closures/README.md). Move on after Write, Predict, Repair, and Extend; keep Review later for another session.
