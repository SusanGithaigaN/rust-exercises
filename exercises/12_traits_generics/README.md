# 12 — Traits and generics

A trait names a set of methods a type must provide. `impl Trait for Type` adds those methods to the type. Generic functions like `fn f<T: SomeTrait>(x: T)` accept any type that satisfies the bound. `&impl Trait` is static dispatch (resolved at compile time); `&dyn Trait` is dynamic dispatch (resolved at runtime).

Complete one step at a time. Run checks from the repository root. Do not change supplied checks to make an incorrect answer pass.

## Write

Open `src/bin/recall.rs`. The `Describable` trait has one required method (`describe`) and one default method (`short_description`). Implement `describe` and get `short_description` for free.

1. Implement `Describable::describe` for `Book`. Return `"Book: 'TITLE' by AUTHOR, PAGES pages"`.

   Check: `python3 scripts/check.py 12_traits_generics recall book`

2. Implement `Describable::describe` for `Movie`. Return `"Movie: 'TITLE' directed by DIRECTOR (YEAR)"`.

   Check: `python3 scripts/check.py 12_traits_generics recall movie`

3. Implement `print_description(item: &impl Describable)`. Print `describe()` on one line, then `short_description()` on the next.

   Check: `python3 scripts/check.py 12_traits_generics recall print_description`

Run: `cargo run -p ex_12_traits_generics --bin recall`

| Output |
| --- |
| `Book: 'The Rust Programming Language' by Steve Klabnik, 560 pages` |
| `Book: 'The Rust Programming L...` |
| `Movie: 'Inception' directed by Christopher Nolan (2010)` |
| `Movie: 'Inception' directed b...` |

## Predict

Open `src/bin/predict.rs`.

Predict every printed line. Explain why `print_pair(3, "hello")` would fail, how `PartialOrd` enables comparison, and which `greeting` implementation each call chooses.

Run: `cargo run -p ex_12_traits_generics --bin predict`

## Repair

Open `src/bin/repair.rs`.

Make `Article` implement the required `summarize` method. Add a formatting bound to `print_twice`. Add comparison and copying bounds to `largest`, keeping its value-returning signature.

Run: `cargo run -p ex_12_traits_generics --bin repair`

## Extend

Open `src/bin/extend.rs`.

1. Reimplement `describe` for both types and `print_description` from recall.

   Checks: `python3 scripts/check.py 12_traits_generics extend book` (and `movie`, `print_description`)

2. Implement `HasTitle::title` for `Book` and `Movie`. Return the title field as `&str` — do not allocate or consume the struct.

   Check: `python3 scripts/check.py 12_traits_generics extend title`

3. Implement `find_by_title<'a, T: HasTitle>(items: &'a [T], title: &str) -> Option<&'a T>`. Return a reference to the first item whose `title()` exactly equals `title` (case-sensitive), or `None`.

   Check: `python3 scripts/check.py 12_traits_generics extend find_by_title`

Run: `cargo run -p ex_12_traits_generics --bin extend`

| Expression | Expected |
| --- | --- |
| `find_by_title(&books, "Rust in Action").map(\|b\| b.title())` | `Some("Rust in Action")` |
| `find_by_title(&books, "Missing").map(\|b\| b.title())` | `None` |

## Review later

Open `src/bin/revisit.rs`. Try these a few days later without copying earlier solutions.

1. Implement `channel_name` for `EmailNotifier` (returns `"email"`) and `SlackNotifier` (returns `"slack"`).

   Check: `python3 scripts/check.py 12_traits_generics revisit channels`

2. Implement `send` for both types. `Email to ADDRESS: MESSAGE`. `Slack #CHANNEL: MESSAGE`.

   Check: `python3 scripts/check.py 12_traits_generics revisit send`

3. Implement `broadcast<T: Notifiable>(notifiers: &[T], message: &str)`. Call `send` on each notifier. One concrete type per slice.

   Check: `python3 scripts/check.py 12_traits_generics revisit broadcast`

4. Implement `notify_all(notifiers: &[&dyn Notifiable], message: &str)`. Same but accepts mixed types via trait objects.

   Check: `python3 scripts/check.py 12_traits_generics revisit notify_all`

Run: `cargo run -p ex_12_traits_generics --bin revisit`

Next: [13_modules_visibility](../13_modules_visibility/README.md). Move on after Write, Predict, Repair, and Extend. Return here for Review later in a separate session.
