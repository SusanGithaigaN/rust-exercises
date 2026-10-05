#!/usr/bin/env python3
"""Send a Rust practice reminder using Hostinger SMTP (standard library only)."""

import argparse
from datetime import datetime
from email.message import EmailMessage
import os
from pathlib import Path
import smtplib
import ssl
import sys
from zoneinfo import ZoneInfo


def load_local_env():
    """Load simple KEY=value entries locally; existing environment takes precedence."""
    path = Path(__file__).resolve().parents[1] / ".env"
    if not path.exists():
        return
    for line in path.read_text().splitlines():
        line = line.strip()
        if not line or line.startswith("#"):
            continue
        key, separator, value = line.partition("=")
        if separator:
            value = value.strip()
            if len(value) >= 2 and value[0] == value[-1] and value[0] in "\"'":
                value = value[1:-1]
            os.environ.setdefault(key.strip(), value)


def build_message(kind, mailbox, now):
    date = now.strftime("%A, %d %B %Y")
    message = EmailMessage()
    message["From"] = mailbox
    message["To"] = mailbox
    if kind == "morning":
        message["Subject"] = f"Rust practice: one small exercise — {date}"
        body = """Good morning! Time for a small Rust practice session.

Spend 15–25 minutes on your current exercise. You do not need a new topic today.

1. Recall the concept or read one small section of the Rust Book.
2. Write, predict, repair, or extend a small example yourself.
3. Run it, read the compiler errors, and explain what happened.
4. Write one sentence about what you learned.

When practising tests, write your own assertions and include a normal case and
an edge case. Use documentation and compiler help; keep AI assistance off.

If you are unsure where to start, open exercises/01_variables/src/bin/recall.rs.
Otherwise continue where you left off. It is fine to finish next session.
"""
    else:
        message["Subject"] = f"Rust check-in: did you practise today? — {date}"
        body = """Did you complete today's Rust practice?

Take a minute to record:
- What did you try?
- What can you explain now?
- What was confusing?
- What is the next small step?

If you wrote tests, can you explain why those cases matter?
If you did not finish, continue next time. No catching-up marathon needed.

This is a reminder only; it does not automatically track completion or replies.
"""
    message.set_content(f"{date} (East Africa Time)\n\n{body}")
    return message


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("kind", choices=("morning", "evening"))
    parser.add_argument("--dry-run", action="store_true", help="Preview without credentials or sending")
    args = parser.parse_args()
    now = datetime.now(ZoneInfo("Africa/Nairobi"))
    if args.dry_run:
        message = build_message(args.kind, "preview@example.com", now)
        print(message["Subject"])
        print(message.get_content())
        return 0

    load_local_env()
    mailbox = os.environ.get("HOSTINGER_EMAIL", "").strip()
    password = os.environ.get("HOSTINGER_PASSWORD", "")
    if not mailbox or not password:
        print("Missing HOSTINGER_EMAIL or HOSTINGER_PASSWORD. Configure GitHub Actions secrets or local .env.", file=sys.stderr)
        return 1
    try:
        message = build_message(args.kind, mailbox, now)
        with smtplib.SMTP_SSL("smtp.hostinger.com", 465, timeout=30,
                              context=ssl.create_default_context()) as smtp:
            smtp.login(mailbox, password)
            refused = smtp.send_message(message)
            if refused:
                print("SMTP rejected the recipient.", file=sys.stderr)
                return 1
    except (OSError, smtplib.SMTPException, ValueError):
        # Do not print server responses or credentials into Actions logs.
        print("Reminder failed. Check mailbox credentials, SMTP access, and Hostinger settings.", file=sys.stderr)
        return 1
    print(f"{args.kind.capitalize()} reminder accepted by SMTP.")
    return 0


if __name__ == "__main__":
    sys.exit(main())
