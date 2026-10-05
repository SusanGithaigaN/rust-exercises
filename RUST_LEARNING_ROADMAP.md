# Rust learning roadmap and preferences

## Learning preferences

- I have learned Rust before, but want to rebuild my foundations and fill gaps.
- I learn through small, consistent practices and want plenty of exercises customised to my needs.
- Avoid overwhelming schedules, large assignments, and pressure to advance quickly.
- I want to learn without AI assistance during practice so I understand what I am doing.
- AI may help prepare exercise prompts beforehand, but should not supply solutions or write my practice code unless I explicitly request it.
- Compiler diagnostics, rust-analyzer, documentation, books, and human help are welcome.

## Daily practice

Aim for 15–25 minutes. Extend a session only when desired. An unfinished exercise can continue next session.

1. Recall one concept, or read a small relevant section of the Rust Book.
2. Close the example and attempt one small exercise from scratch.
3. Predict the result, run the code, and read compiler errors fully.
4. Explain the important lines and write one sentence about what I learned.

Spend several sessions on one concept when useful. Include regular review and rest; a new concept every day is unnecessary. Documentation is allowed: independent problem solving does not require memorising every API.

## How to customise exercises

| Experience with the exercise | Next step |
| --- | --- |
| Easy and clearly explained | Try a slightly harder variation |
| Completed with confusion or documentation | Practise the same concept with a different example |
| Too difficult | Reduce the problem to fewer concepts and smaller steps |
| Comfortable in an earlier session | Revisit with a short recall exercise |

Rotate between writing, predicting output or compilation, repairing one deliberate error, extending a previous exercise, and revisiting a concept later.

Use mistakes to identify gaps rather than assuming their cause. Move forward when I can solve a fresh variation and explain the central idea, while continuing to review. Perfect recall is not required.

## Learning sequence

Use this as a background guide. Focus only on the current topic and next exercise; there are no fixed deadlines.

### 1. Foundations

- Cargo: creating, checking, running, and formatting projects.
- Variables, mutability, types, functions, expressions, and control flow.
- Small tests for simple functions.
- Exercises: larger of two numbers, temperature conversion, loop-based calculations.
- Checkpoint: write a small program from a blank project and explain it.

### 2. Ownership and borrowing

- Scope, moves, `Copy`, and `Clone`.
- Shared and mutable references.
- Strings, slices, and `String` versus `&str`.
- Exercises across separate sessions: predict a move, repair a moved-value error, borrow a string, modify through a mutable reference, return a slice.
- Checkpoint: explain ownership and choose between moving and borrowing deliberately.

### 3. Data and error handling

- Vectors, iteration, maps, structs, methods, and enums.
- Pattern matching, `Option`, `Result`, and `?`.
- Modules and visibility; unit tests.
- Exercises: safely access a vector item, represent task status, validate input, handle missing data.
- Checkpoint: model a small problem and handle expected failures without blindly using `unwrap()`.

### 4. Abstraction

- Closures and iterators.
- Basic traits and generics.
- Lifetimes and lifetime elision.
- Exercises: filter records, implement `Display`, write a small generic function, explain a borrowed return value.
- Checkpoint: understand common type signatures and justify borrowing choices.

### 5. Complete small applications

- File input/output, dependencies, command-line arguments, and custom errors.
- Integration tests, documentation, and Clippy.
- Gradually combine familiar exercises into a text analyser or task manager; later add persistence.
- Learn standard error handling before convenience crates.
- Checkpoint: another person can use the program and its errors are understandable.

### 6. Memory and concurrency

- `Box`, `Rc`, `Arc`, `Weak`, `RefCell`, and resource cleanup.
- Threads, channels, locks, `Send`, and `Sync`.
- Begin with a sequential file analyser; introduce concurrency in small increments.
- Checkpoint: explain shared ownership, communication, and synchronization choices.

### 7. Async and networking

- Futures, async/await, runtimes, tasks, and blocking work.
- Timeouts, cancellation, bounded concurrency, and partial failures.
- Gradually build an endpoint checker.
- Checkpoint: explain suspension and handle slow or failed requests.

### 8. Advanced topics and specialisation

- Explore API design, advanced traits, profiling, macros, and async internals as projects require them.
- Unsafe Rust, FFI, variance, and self-referential structures are optional areas, not prerequisites for competence.
- Choose a direction such as backend development, systems, developer tools, embedded, or WebAssembly.
- Build a substantial project through small increments; read related code and consider a small contribution.
- Checkpoint: justify design decisions and maintain unfamiliar or larger code confidently.

## Resources

- Main text: [The Rust Book](https://doc.rust-lang.org/book/).
- Matching exercises: [Rustlings](https://rustlings.rust-lang.org/).
- Additional examples: [Rust by Example](https://doc.rust-lang.org/rust-by-example/).
- Reference: [Standard library](https://doc.rust-lang.org/std/) and `rustc --explain <error-code>`.
- Later: [Cargo Book](https://doc.rust-lang.org/cargo/), [Rust Reference](https://doc.rust-lang.org/reference/), and [Rustonomicon](https://doc.rust-lang.org/nomicon/) for unsafe study.

## When stuck

Read the error, explain the expected behaviour, reduce the example, and consult documentation. After a focused attempt, seek a hint or human explanation. Avoid copying complete solutions; if one is consulted, close it and rebuild a variation independently.

The aim is a sustainable habit of understanding small things deeply, not rushing to advanced Rust.
