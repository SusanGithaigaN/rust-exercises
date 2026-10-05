# Small, consistent Rust practice

A Rust learning workspace with 70 exercise files across 14 topics, plus optional
daily email reminders. Work through one small task at a time, practise writing
your own code and tests, and revisit concepts until you can explain them.

Aim for 15–25 minutes per session. Continue an unfinished exercise next time;
there is no deadline for completing a topic. The exercises are intended for
practice without AI-generated solutions. Compiler help, editor tooling, books,
and documentation are encouraged.

## Get started

1. Fork this repository, or use **Use this template** if its owner has enabled
   the GitHub template setting. You can also copy it into your own repository.
2. Clone your repository and open a terminal in its root directory.
3. Install stable Rust and Cargo using [rustup](https://rustup.rs/).
4. Open `exercises/01_variables/src/bin/recall.rs` and read its instructions.
5. Implement a small part of the exercise and run it:

```bash
cargo run -p ex_01_variables --bin recall
```

Read the comments at the top of each file for its task and expected behaviour.
Some files contain unfinished code or deliberate compiler errors. These are
part of the exercise; the entire workspace is not expected to pass every check
before you solve them.

## How the repository works

```text
Cargo.toml                         Workspace containing all 14 topic packages
exercises/
  01_variables/                    Each topic is a separate Cargo package
    Cargo.toml
    src/bin/
      recall.rs                    Write code from scratch
      predict.rs                   Predict behaviour before running
      repair.rs                    Explain and fix a deliberate mistake
      extend.rs                    Add a requirement to an existing example
      revisit.rs                   Practise a variation later
  ...
  14_writing_tests/
    src/bin/read_first.rs          Testing introduction with worked examples
    src/bin/exercise_01.rs         Assignments where you write the tests
    ...
scripts/send_reminder.py           Hostinger email sender
.github/workflows/rust-reminders.yml
.env.example                      Placeholder credentials for local setup
RUST_LEARNING_ROADMAP.md           Roadmap and learning preferences
EMAIL_REMINDERS.md                 Detailed reminder setup
```

Topics 1–13 each contain the five exercise styles above. Topic 14 contains a
testing introduction and four test-writing assignments.

| Topic | Focus |
| --- | --- |
| 01 | Variables, mutability, and functions |
| 02 | Ownership and moves |
| 03 | Shared borrowing |
| 04 | Mutable borrowing |
| 05 | Slices and strings |
| 06 | Structs and implementation blocks |
| 07 | Enums and pattern matching |
| 08 | `Option` |
| 09 | `Result` |
| 10 | Vectors and iteration |
| 11 | Closures |
| 12 | Traits and generics |
| 13 | Modules and visibility |
| 14 | Writing tests |

The [roadmap](RUST_LEARNING_ROADMAP.md) also describes later topics such as
concurrency and async; exercises for those stages are not included yet.

## Run one exercise at a time

From the repository root, specify both the package and binary. Several packages
use the same binary names, so `--bin recall` alone is ambiguous at the root.

```bash
cargo check -p ex_01_variables --bin recall
cargo run -p ex_01_variables --bin recall
cargo run -p ex_02_ownership_moves --bin predict
```

Alternatively, enter a topic directory and use its shorter command:

```bash
cd exercises/01_variables
cargo run --bin recall
```

Before running a prediction exercise, write down what you expect. For a repair
exercise, explain the error before editing the code. After any exercise, try a
small variation and record one sentence about what you learned.

If an exercise feels too large, complete only one function or requirement per
session. If it is easy, try a variation. If it is confusing, practise the same
concept with a smaller example. Revisit earlier exercises regularly.

## Learn to write tests

Topic 14 introduces test modules, assertions, edge cases, and expected panics.
Its `read_first.rs` file includes worked tests; the assignment files leave test
implementation to you.

```bash
cargo test -p ex_14_writing_tests --bin read_first
cargo test -p ex_14_writing_tests --bin exercise_01
```

You can also practise writing a small test for an earlier function once you
understand the introduction. Choose a typical input and an edge case, explain
the expected results, and write your own assertions.

A successful run reporting **zero tests** does not demonstrate correctness:
add your tests before treating a test-writing assignment as complete. Avoid
running workspace-wide tests while unrelated repair exercises are unfinished.

## Optional daily email reminders

The GitHub Actions workflow sends from your Hostinger mailbox to that same
mailbox. It runs on GitHub, even when your computer is off.

| Reminder | East Africa Time (UTC+3) | Workflow cron in UTC |
| --- | --- | --- |
| Morning practice reminder | 08:00 daily | `0 5 * * *` |
| Evening completion check-in | 20:30 daily | `30 17 * * *` |

Emails encourage small practice sessions. They do not select your next exercise,
track completion, or process replies. Reminders are optional; Rust exercises
work without any email configuration or Python installation.

### Activate reminders in your own repository

1. Ensure the script and workflow are committed to your repository's **default
   branch**. For a fork, enable GitHub Actions in the **Actions** tab if prompted.
2. Open **Settings → Secrets and variables → Actions → New repository secret**.
3. Add these two secrets using **your own** Hostinger mailbox credentials:

   | Secret name | Value |
   | --- | --- |
   | `HOSTINGER_EMAIL` | Full mailbox address |
   | `HOSTINGER_PASSWORD` | Mailbox password, not your Hostinger account password |

4. Open **Actions → Rust practice email reminders → Run workflow**. Select
   `morning` or `evening` and run it to send a real test email.
5. Check the run logs and your inbox, including spam. Once configured, both
   daily schedules use the same secrets automatically.

GitHub cannot read the `.env` on your computer. Local credentials and repository
secrets are separate. Never commit `.env` or put credentials directly into the
workflow; `.gitignore` excludes local environment files.

GitHub scheduling is best-effort: runs can be delayed or dropped under load,
and exact-minute delivery is not guaranteed. Public repositories' schedules
can be disabled after 60 days without repository activity. See
[GitHub's schedule documentation](https://docs.github.com/en/actions/reference/workflows-and-actions/events-that-trigger-workflows#schedule).

### Preview or send locally

Use Python 3.9 or newer. The script needs no third-party Python packages.
Preview both messages without credentials or sending:

```bash
python3 scripts/send_reminder.py morning --dry-run
python3 scripts/send_reminder.py evening --dry-run
```

For a real local send, copy `.env.example` to `.env`, fill in your mailbox
credentials, and run:

```bash
cp .env.example .env
# Edit .env before sending. Do not overwrite an existing configured .env.
python3 scripts/send_reminder.py morning
```

Existing environment variables take precedence over `.env` values. SMTP uses
`smtp.hostinger.com` on port 465 with verified TLS, matching
[Hostinger Email settings](https://www.hostinger.com/support/1575756-how-to-get-email-account-configuration-details-for-hostinger-email/).
The script reports SMTP acceptance; this does not guarantee inbox placement.

### Customise or troubleshoot

- Edit the subject and body in `build_message()` in `scripts/send_reminder.py`.
- To change reminder times, update both the cron entries **and** their matching
  conditions in the workflow's send step. The current expressions use UTC.
- Other mail providers require changes to the SMTP host, port, and potentially
  authentication; this template is configured specifically for Hostinger Email.
- If a run fails, check secret names, mailbox credentials, and SMTP settings.
- If no run appears, check Actions is enabled and the workflow is on the default
  branch. Check for a disabled schedule or GitHub service delays.
- To stop reminders, disable this workflow in the Actions tab.

See [EMAIL_REMINDERS.md](EMAIL_REMINDERS.md) for the dedicated setup guide.

## Learning resources

- [The Rust Book](https://doc.rust-lang.org/book/) — read a small section at a time.
- [Rustlings](https://rustlings.rust-lang.org/) — additional focused exercises.
- [Rust by Example](https://doc.rust-lang.org/rust-by-example/) — examples and syntax.
- [Standard library documentation](https://doc.rust-lang.org/std/) — API reference.
- `rustc --explain <error-code>` — explanations for compiler errors.
