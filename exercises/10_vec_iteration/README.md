# 10 — Vec and iterators

`Vec<T>` is a growable list on the heap. Iterators let you transform and query it without writing index loops: `.iter()` borrows elements, `.filter(|x| ...)` keeps matching ones, `.map(|x| ...)` transforms each, `.collect()` gathers results, `.sum()` totals them. Chain these instead of writing `for i in 0..v.len()`.

Complete one step at a time. Run checks from the repository root. Do not change supplied checks to make an incorrect answer pass.

## Write

Open `src/bin/recall.rs`. Loops are allowed for this section.

1. `sum(v: &[i32]) -> i32` — total of all elements. Empty → 0.

   Check: `python3 scripts/check.py 10_vec_iteration recall sum`

2. `max(v: &[i32]) -> Option<&i32>` — borrow the largest element. Empty → `None`.

   Check: `python3 scripts/check.py 10_vec_iteration recall max`

3. `min(v: &[i32]) -> Option<&i32>` — borrow the smallest element. Empty → `None`.

   Check: `python3 scripts/check.py 10_vec_iteration recall min`

4. `evens(v: &[i32]) -> Vec<i32>` — only even values, in original order, keeping duplicates.

   Check: `python3 scripts/check.py 10_vec_iteration recall evens`

5. `squares(v: &[i32]) -> Vec<i32>` — each element squared, in order. Assume small values.

   Check: `python3 scripts/check.py 10_vec_iteration recall squares`

6. `count_over_four(v: &[i32]) -> usize` — count of elements strictly greater than 4.

   Check: `python3 scripts/check.py 10_vec_iteration recall count_over_four`

Run: `cargo run -p ex_10_vec_iteration --bin recall`

(Data: `[3, 1, 4, 1, 5, 9, 2, 6]`)

| Expression | Expected |
| --- | --- |
| `sum(&v)` | `31` |
| `max(&v)` | `Some(9)` |
| `min(&v)` | `Some(1)` |
| `evens(&v)` | `[4, 2, 6]` |
| `squares(&v)` | `[9, 1, 16, 1, 25, 81, 4, 36]` |
| `count_over_four(&v)` | `3` |

## Predict

Open `src/bin/predict.rs`.

Predict every printed line. Explain the difference between `.iter()`, `.iter_mut()`, and `.into_iter()`. Explain why `v` stays usable after `.iter()` but not after `.into_iter()`.

Run: `cargo run -p ex_10_vec_iteration --bin predict`

## Repair

Open `src/bin/repair.rs`.

Keep `v` usable by borrowing while doubling. Collect owned values for `evens`. End the first-element borrow before mutating `v3`. Give the sum a concrete numeric type.

Run: `cargo run -p ex_10_vec_iteration --bin repair`

## Extend

Open `src/bin/extend.rs`. Use iterator methods — avoid index loops.

1. Reimplement all six functions from recall.

2. `running_totals(v: &[i32]) -> Vec<i32>`. Cumulative sums in order. `[1, 2, 3]` → `[1, 3, 6]`. Empty → `[]`.

   Check: `python3 scripts/check.py 10_vec_iteration extend running_totals`

3. `flatten_vecs(v: &[Vec<i32>]) -> Vec<i32>`. Concatenate all inner vecs, keeping duplicates, skipping empty inner vecs.

   Check: `python3 scripts/check.py 10_vec_iteration extend flatten_vecs`

Run: `cargo run -p ex_10_vec_iteration --bin extend`

| Expression | Expected |
| --- | --- |
| `running_totals(&[1, 2, 3, 4, 5])` | `[1, 3, 6, 10, 15]` |
| `flatten_vecs(&[vec![1,2], vec![3], vec![4,5,6]])` | `[1, 2, 3, 4, 5, 6]` |

## Review later

Open `src/bin/revisit.rs`. Try these a few days later without copying earlier solutions.

(Data: `["apple", "banana", "cherry", "avocado", "blueberry", "fig"]`)

1. `longer_than_five(words: &[&str]) -> usize` — count words with more than 5 Unicode characters.
2. `starts_ab(words: &[&str]) -> Vec<String>` — collect words starting with `'a'` or `'b'` (lowercase), preserving order.
3. `longest<'a>(words: &[&'a str]) -> &'a str` — borrow the longest word (first if tied). Empty slice → `""`.
4. `joined(words: &[&str]) -> String` — join with `", "`. Empty → `""`.
5. `first_letters(words: &[&str]) -> Vec<char>` — first character of each nonempty word, in order. Skip empty strings.
6. `contains_e(words: &[&str]) -> usize` — count words that contain lowercase `'e'`.

   Checks: `python3 scripts/check.py 10_vec_iteration revisit longer_than_five` (and others)

Run: `cargo run -p ex_10_vec_iteration --bin revisit`

| Expression | Expected |
| --- | --- |
| `longer_than_five(&words)` | `4` |
| `starts_ab(&words)` | `["apple", "banana", "avocado", "blueberry"]` |
| `longest(&words)` | `blueberry` |
| `joined(&words)` | `apple, banana, cherry, avocado, blueberry, fig` |
| `first_letters(&words)` | `['a', 'b', 'c', 'a', 'b', 'f']` |
| `contains_e(&words)` | `3` |

Next: [11_closures](../11_closures/README.md). Move on after Write, Predict, Repair, and Extend. Return here for Review later in a separate session.
