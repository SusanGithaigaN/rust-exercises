# 12 — Traits and generics

Work on one numbered step at a time. Write your code in `src/bin/`; the supplied checks describe expected behaviour and are not solutions.

Run commands from the repository root. A function check runs only that step, so other `todo!()` bodies can remain unfinished. Do not change the supplied checks to make an incorrect answer pass.

The checks cover the listed examples; also explain your result and try one new input before recording a task as done.

## Write

Open `src/bin/recall.rs`.

1. Implement `Describable::describe` for Book. Return `"Book: 'TITLE' by AUTHOR, PAGES pages"`, substituting the stored fields.

   Check: `python3 scripts/check.py 12_traits_generics recall book`.

2. Implement `Describable::describe` for Movie. Return `"Movie: 'TITLE' directed by DIRECTOR (YEAR)"`.

   Check: `python3 scripts/check.py 12_traits_generics recall movie`.

3. Implement `print_description(item)`. Print the full description on one line, then the default short description on the next. The default keeps the first 30 Unicode characters plus `...` only when longer than 30. Run the file after completing both implementations. Each item must print its full description followed by its shortened description.

   Check: `python3 scripts/check.py 12_traits_generics recall print_description`.

After all steps: `cargo run -p ex_12_traits_generics --bin recall`.

## Predict

Open `src/bin/predict.rs`.

Predict every printed line. Explain why print_pair(3, "hello") would fail, how PartialOrd enables comparison, and which greeting implementation each call chooses.

Run: `cargo run -p ex_12_traits_generics --bin predict`.

Done when you have compared the output with your prediction and explained each difference.

## Repair

Open `src/bin/repair.rs`.

Make Article implement the trait’s required summarize method. Add a formatting bound to print_twice. Add comparison and copying bounds to largest, keeping its value-returning signature. Use the supplied nonempty list.

Run: `cargo run -p ex_12_traits_generics --bin repair`.

Done when the program compiles, runs without an unexpected panic, and meets the results above. Explain the cause of each error. Some fixes address logic errors or warnings, not compiler errors.

## Extend

Open `src/bin/extend.rs`.

1. Reimplement from recall: `Describable::describe` for Book. Return `"Book: 'TITLE' by AUTHOR, PAGES pages"`, substituting the stored fields.

   Check: `python3 scripts/check.py 12_traits_generics extend book`.

2. Reimplement from recall: `Describable::describe` for Movie. Return `"Movie: 'TITLE' directed by DIRECTOR (YEAR)"`.

   Check: `python3 scripts/check.py 12_traits_generics extend movie`.

3. Reimplement from recall: `print_description(item)`. Print the full description on one line, then the default short description on the next. The default keeps the first 30 Unicode characters plus `...` only when longer than 30. Run the file after completing both implementations. Each item must print its full description followed by its shortened description.

   Check: `python3 scripts/check.py 12_traits_generics extend print_description`.

4. Implement `HasTitle::title` for Book and Movie. Borrow each stored title as `&str`; do not allocate or consume the object.

   Check: `python3 scripts/check.py 12_traits_generics extend title`.

5. Implement `find_by_title(items, title)`. Return a reference to the first item with an exactly matching title, or None. Match case-sensitively. Empty input returns None. Keep the generic and lifetime signature.

   Check: `python3 scripts/check.py 12_traits_generics extend find_by_title`.

After all steps: `cargo run -p ex_12_traits_generics --bin extend`.

## Review later

Open `src/bin/revisit.rs`.

Try these a few days later without copying earlier solutions. Continue to the next topic now if you prefer; return here for review.

1. Implement `channel_name` for both types. EmailNotifier returns `"email"`; SlackNotifier returns `"slack"`.

   Check: `python3 scripts/check.py 12_traits_generics revisit channels`.

2. Implement `send` for both types as console output only. Email: `"Email to ADDRESS: MESSAGE"`. Slack: `"Slack #CHANNEL: MESSAGE"`. No real messages are sent.

   Check: `python3 scripts/check.py 12_traits_generics revisit send`.

3. Implement `broadcast(notifiers, message)`. Call send once on each notifier in order. The generic slice holds one concrete notifier type. Empty input prints nothing.

   Check: `python3 scripts/check.py 12_traits_generics revisit broadcast`.

4. Implement `notify_all(notifiers, message)`. Call send once per trait object in order. This slice can contain different notifier types. Empty input prints nothing.

   Check: `python3 scripts/check.py 12_traits_generics revisit notify_all`.

After all steps: `cargo run -p ex_12_traits_generics --bin revisit`.

Next topic: [13_modules_visibility](../13_modules_visibility/README.md). Move on after Write, Predict, Repair, and Extend; keep Review later for another session.
