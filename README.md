# Learn Rust one step at a time

Start with variables, then build up to functions, ownership, and larger programs.
Each topic has a short instruction page. It tells you what to create, the exact
behaviour expected, and how to check one step before moving on.

## Start here

1. Install Rust and Cargo using [rustup](https://rustup.rs/).
2. Open [Topic 01: variables and functions](exercises/01_variables/README.md).
3. Start with **Variables**, in `exercises/01_variables/src/bin/variables.rs`.
4. Write your code, then run this from the repository root:

```bash
python3 scripts/check.py 01_variables variables
```

The first task is to create a mutable variable named `a`, add to it, and print
its new value. The instruction page gives the names, types, values, and expected
output. No solution is supplied.

The check helper needs Python 3.9 or newer and no extra packages. If you prefer
Cargo directly, use `cargo run -p ex_01_variables --bin variables` and compare
its output with the instructions yourself.

## Your practice routine

1. Read one numbered step in the topic README.
2. Write the required code in `src/bin/`.
3. Run that step's check command. Read any compiler error or failed assertion.
4. Explain why your result is correct and try one new input.
5. Continue to the next step. Aim for 15–25 minutes per session.

Function checks run in isolation. For example:

```bash
python3 scripts/check.py 01_variables recall double
```

This checks only `double`; `describe_sign` and the temperature converter can
remain unfinished. A `todo!()` means you still need to implement that function.
Once all functions in a file are complete, run its main example:

```bash
cargo run -p ex_01_variables --bin recall
```

Checks for variables and shadowing compare printed output. You must also follow
the required variable names, types, and use of mutability; output alone cannot
verify those choices. Function checks cover the listed examples and boundaries,
but you should still try your own inputs.

## Topic order

Work through **Write → Predict → Repair → Extend** within each topic.
Topic 01 starts with **Variables → Shadowing → Write → Loops** first.
Save **Review later** for a later session; it does not block the next topic.

| Topic | What you practise |
| --- | --- |
| [01](exercises/01_variables/README.md) | Variables, mutability, shadowing, functions, conditions, loops |
| [02](exercises/02_ownership_moves/README.md) | Ownership, moves, and copies |
| [03](exercises/03_borrowing_shared/README.md) | Shared borrowing |
| [04](exercises/04_borrowing_mutable/README.md) | Changing values through mutable references |
| [05](exercises/05_slices_strings/README.md) | String slices and words |
| [06](exercises/06_structs_impl/README.md) | Structs, constructors, and methods |
| [07](exercises/07_enums_match/README.md) | Enums and pattern matching |
| [08](exercises/08_option/README.md) | Missing values with Option |
| [09](exercises/09_result/README.md) | Errors with Result and ? |
| [10](exercises/10_vec_iteration/README.md) | Vectors, loops, and iterator examples |
| [11](exercises/11_closures/README.md) | Closures and captured values |
| [12](exercises/12_traits_generics/README.md) | Traits and generics |
| [13](exercises/13_modules_visibility/README.md) | Modules and visibility |
| [14](exercises/14_writing_tests/README.md) | Writing your own tests |

**Predict:** write down the output before running the example, then explain any
differences. **Repair:** fix the deliberate errors to meet the specified result.
These files are not expected to pass until you finish the repair.

Keep your practice code in `src/bin/`. Topics 1–13 keep supplied checks in
`checks/`, so the code you edit stays short. Leave those checks unchanged.
Topic 14 provides implemented functions and asks you to write your own tests.
Its check commands reject a successful run with zero tests and require the
listed minimum number of tests. You must also cover every required case.

Avoid workspace-wide checks while repair exercises are unfinished. Always
specify a package and binary, or use the check command in the topic README.
To format your current file, use `rustfmt path/to/exercise.rs`.

## Optional progress and reminders

The practice exercises work without email setup. To see your next planned task:

```bash
python3 scripts/progress.py status
```

To record it:

```bash
python3 scripts/progress.py update --status in_progress --note "What I tried"
python3 scripts/progress.py update --status done --note "What I learned"
```

The reminder catalog follows the same steps and check commands as the topic
pages. Reviews come after the main sequence. Existing task IDs and recorded
progress are preserved; a saved daily plan stays unchanged for that day.
Completion is self-reported, and passing a check does not update progress.

See [progress tracking](PROGRESS_TRACKING.md) for local and GitHub updates, and
[email reminders](EMAIL_REMINDERS.md) for the optional morning and evening
reminders. Credentials belong in local `.env` files or GitHub secrets.

## When you get stuck

Read the compiler error fully, check the task's expected behaviour, and try a
smaller example. Consult the matching Rust Book section linked in your planned
task, or use `rustc --explain <error-code>`.

- [The Rust Book](https://doc.rust-lang.org/book/)
- [Rust by Example](https://doc.rust-lang.org/rust-by-example/)
- [Standard library reference](https://doc.rust-lang.org/std/)
- [Longer learning roadmap](RUST_LEARNING_ROADMAP.md)

AI can help prepare prompts, but your practice files remain yours to implement.
