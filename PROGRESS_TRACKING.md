# Plans and learning progress

Both emails use the same three sections: **Today's Focus**, **Progress Summary**,
and **Upcoming Tracker**. Focus lists the topic, Rust Book chapter link, and one
small exercise. Summary shows completed task/topic counts and recorded notes.
Upcoming lists the next two tasks with their topics and reading sections.
A chapter is reference material: you do not need to finish it in one session.

The tracker uses repository JSON rather than a separate database. Task metadata
includes `topic_id`, `topic`, `chapter_label`, `resource_url`, and
`estimated_minutes`, plus `instructions_path` for the topic guide. `daily_status()` creates the shared data used by both
email templates. A topic counts as complete only when all its tasks are done.
Existing progress records and task IDs remain compatible.

Each email includes today's task, its file and command, recorded activity for
the day, the five most recently completed tasks and your notes, and two upcoming
tasks. The catalog splits multi-function exercises into smaller steps; other
files can span multiple sessions. Function tasks use isolated checks, so other
unfinished functions do not run. Review tasks come after the main topic sequence.
Only one new task is planned per day.

No past work is assumed complete. Record anything you have already finished
using its task ID. Completion means you have followed the topic instructions, passed the required
checks, and can explain the requirement; the tracker does not grade code or infer mastery.

## Update from GitHub

Once the changes are on your repository's default branch:

1. Open **Actions → Record Rust progress → Run workflow**.
2. Choose the same branch used by your email reminders.
3. Select `in_progress`, `done`, or `not_started` (to reopen a task).
4. Leave **task** blank to select today's incomplete task, or enter an ID from
   your email, such as `01_variables/recall/1`.
5. Write a short note about what you learned or found confusing and run it.

The workflow commits your update to `learning/progress.json`. The next email
uses that saved progress. An email already sent cannot reflect later updates;
manually run an evening reminder again to receive a refreshed summary.
If today's task is already done, specify a task ID for another update.

The reminder workflow also saves the day's plan before sending, keeping morning
and evening assignments consistent. An unfinished task remains the next day's
task. Finishing it does not automatically assign more work that same day.

Both workflows require `contents: write`. Repository or organisation policies
and protected branches may prevent the bot from committing. If a push is
rejected, use a branch where bot updates are permitted or use local updates
through your usual review process. The workflows deliberately fail if saving
fails, rather than sending a plan that was not persisted.

## Update locally

```bash
python3 scripts/progress.py status
python3 scripts/progress.py plan
python3 scripts/progress.py update --status in_progress --note "Implemented double; still checking negative inputs"
python3 scripts/progress.py update --status done --note "Can explain the parameter and return value"
```

`status` only displays progress. `plan` saves today's selection. `update` records
your status and note, saving today's selection if needed. To choose a particular
task or reopen one:

```bash
python3 scripts/progress.py update --task 01_variables/recall/1 --status not_started --note "Want more practice"
```

GitHub emails read the repository's version, not your local files. Pull bot
updates before editing progress locally, then commit and push your changes:

```bash
git pull --ff-only
# Run the progress update command here.
git add learning/progress.json
git commit -m "Record Rust practice progress"
git push
```

If local progress is already modified, preserve it and reconcile it with the
bot's changes before committing; avoid replacing either history blindly.

## Personalise the plan

- `learning/tasks.json` defines ordered task IDs, instructions, files, and commands.
- `learning/progress.json` stores statuses, daily plans, and dated note history.
- You can shorten or reorder the catalog, or add review tasks with unique IDs.
- Keep IDs stable once progress refers to them. Do not delete referenced tasks
  without updating their saved plans and records too.
- Progress dates use `Africa/Nairobi` (EAT).

Notes are committed to your repository and visible to anyone who can read it.
Email replies are not processed. All summaries reflect your own recorded notes,
not an AI evaluation or an assumption that running code means completion.

Preview an email without sending:

```bash
python3 scripts/send_reminder.py morning --dry-run
python3 scripts/send_reminder.py evening --dry-run
```

Infrastructure checks (separate from Rust test-writing assignments):

```bash
python3 -m unittest discover -s tests -v
```
