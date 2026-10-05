#!/usr/bin/env python3
"""Send a Rust practice reminder using Hostinger SMTP (standard library only)."""

import argparse
from datetime import datetime
from email.message import EmailMessage
import os
from pathlib import Path
import smtplib
import socket
import ssl
import sys
from zoneinfo import ZoneInfo

import progress


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
        message["Subject"] = f"Rust practice: today's plan — {date}"
        greeting = "Good morning! Here is your small Rust practice plan."
    else:
        message["Subject"] = f"Rust check-in: today's progress — {date}"
        greeting = "Did you practise today? Here is your recorded progress."
    tasks, state = progress.load()
    body = greeting + "\n\n" + progress.report(tasks, state, now.date().isoformat())
    body += "\n\nUse documentation and compiler help; keep AI assistance off during practice."
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
    if any(c in mailbox for c in "\r\n") or "@" not in mailbox:
        print("HOSTINGER_EMAIL must contain only the full mailbox address, without quotes or a variable name.", file=sys.stderr)
        return 1
    if password.startswith("HOSTINGER_PASSWORD=") or password[:1] in ("\"", "'"):
        print("Check HOSTINGER_PASSWORD: paste only the password value, without variable names or surrounding quotes.", file=sys.stderr)
        return 1
    stage = "message preparation"
    try:
        message = build_message(args.kind, mailbox, now)
        stage = "connection/TLS"
        with smtplib.SMTP_SSL("smtp.hostinger.com", 465, timeout=30,
                              context=ssl.create_default_context()) as smtp:
            stage = "authentication"
            smtp.login(mailbox, password)
            stage = "message delivery"
            refused = smtp.send_message(message)
            if refused:
                print("SMTP rejected the recipient.", file=sys.stderr)
                return 1
    except (OSError, smtplib.SMTPException, ValueError) as error:
        # Log only fixed explanations, exception class, and numeric SMTP codes.
        # Server responses can contain mailbox addresses or other sensitive data.
        code = getattr(error, "smtp_code", None)
        detail = f" (SMTP {code})" if isinstance(code, int) else ""
        print(f"Reminder failed during {stage}: {type(error).__name__}{detail}.", file=sys.stderr)
        if isinstance(error, smtplib.SMTPAuthenticationError):
            print("Check GitHub secrets against your mailbox login: full email address and mailbox password. Verify login in Hostinger webmail.", file=sys.stderr)
        elif isinstance(error, socket.gaierror):
            print("Runner could not resolve smtp.hostinger.com. Check DNS/network availability.", file=sys.stderr)
        elif isinstance(error, TimeoutError):
            print("SMTP connection or response timed out. Check network access and provider availability.", file=sys.stderr)
        elif isinstance(error, ssl.SSLError):
            print("TLS negotiation or certificate verification failed. Check provider TLS settings; certificate verification remains enabled.", file=sys.stderr)
        else:
            print("Check SMTP access and Hostinger settings for the reported stage.", file=sys.stderr)
        return 1
    print(f"{args.kind.capitalize()} reminder accepted by SMTP.")
    return 0


if __name__ == "__main__":
    sys.exit(main())
