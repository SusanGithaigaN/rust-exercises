# Rust practice email reminders

The workflow sends email from your Hostinger mailbox to the same mailbox:

- Daily at 08:00 EAT: a small practice reminder.
- Daily at 20:30 EAT: a completion check-in.

It runs on GitHub's infrastructure; your computer does not need to be on.
Each email includes a daily task, recorded progress, your learning notes, and
upcoming tasks. Record your own progress through the **Record Rust progress**
workflow or a local command; see [PROGRESS_TRACKING.md](PROGRESS_TRACKING.md).
Email replies are not processed.

## Activate on GitHub

1. Push this project, including `.github/workflows/rust-reminders.yml` and
   `scripts/send_reminder.py`, to the repository's default branch.
2. In the repository, open **Settings → Secrets and variables → Actions**.
3. Create repository secrets named `HOSTINGER_EMAIL` and `HOSTINGER_PASSWORD`,
   using the corresponding values from your local `.env`. Never commit `.env`.
4. Open **Actions → Rust practice email reminders → Run workflow**.
   Choose `morning` or `evening` to send a manual test email.
5. Check the workflow result and your inbox (including spam).

GitHub runners cannot read your local `.env`; repository secrets are required.
If workflows are disabled for the repository, enable GitHub Actions first.
The workflows also need permission to commit daily plans and progress to the
selected branch. Protected branches or repository policies can reject these
updates. Learning notes are committed and visible to readers of the repository.

Scheduled runs use UTC, so the cron expressions are `0 5 * * *` and
`30 17 * * *`. GitHub may delay or drop scheduled runs during high load;
delivery at the exact minute is not guaranteed. Schedules run from the default
branch. In public repositories, scheduled workflows are disabled after 60 days
without repository activity.

## Local preview or send

No third-party Python packages are needed. Use Python 3.9 or newer.

```bash
python3 scripts/send_reminder.py morning --dry-run
python3 scripts/send_reminder.py evening --dry-run
```

Dry runs do not load credentials or send email. To send using your local `.env`:

```bash
python3 scripts/send_reminder.py morning
```

Existing environment variables take precedence over simple local `.env` entries.
Credentials are never printed by the script. Failures return a nonzero exit code.

SMTP uses `smtp.hostinger.com`, port 465, with verified TLS, matching
[Hostinger Email settings](https://www.hostinger.com/support/1575756-how-to-get-email-account-configuration-details-for-hostinger-email/).
Scheduling limitations are described in
[GitHub's schedule documentation](https://docs.github.com/en/actions/reference/workflows-and-actions/events-that-trigger-workflows#schedule).
