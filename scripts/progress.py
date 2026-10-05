#!/usr/bin/env python3
"""Plan one small Rust task and record explicit learning progress."""

import argparse
from datetime import datetime
import json
import os
from pathlib import Path
from zoneinfo import ZoneInfo

ROOT = Path(__file__).resolve().parents[1]
CATALOG = ROOT / "learning/tasks.json"
STATE = ROOT / "learning/progress.json"


def today():
    return datetime.now(ZoneInfo("Africa/Nairobi")).date().isoformat()


def load():
    tasks = json.loads(CATALOG.read_text())
    state = json.loads(STATE.read_text()) if STATE.exists() else {"tasks": {}, "days": {}}
    ids = [task["id"] for task in tasks]
    if len(ids) != len(set(ids)):
        raise ValueError("Task IDs must be unique")
    return tasks, state


def save(state):
    STATE.parent.mkdir(parents=True, exist_ok=True)
    temporary = STATE.with_suffix(".tmp")
    temporary.write_text(json.dumps(state, indent=2, ensure_ascii=False) + "\n")
    temporary.replace(STATE)


def plan(tasks, state, date):
    if date not in state["days"]:
        remaining = [t["id"] for t in tasks
                     if state["tasks"].get(t["id"], {}).get("status") != "done"]
        state["days"][date] = remaining[:1]
    return state["days"][date]


def daily_status(tasks, state, date):
    """Shared data for morning and evening templates, using the same saved plan."""
    ids = plan(tasks, state, date)
    completed = [t for t in tasks if state["tasks"].get(t["id"], {}).get("status") == "done"]
    topics = {}
    for task in tasks:
        topics.setdefault(task.get("topic_id", task["id"].split("/")[0]), []).append(task)
    completed_topics = sum(all(state["tasks"].get(t["id"], {}).get("status") == "done"
                               for t in members) for members in topics.values())
    return {
        "date": date,
        "todays_focus": [t for task_id in ids for t in tasks if t["id"] == task_id],
        "progress_summary": {
            "completed_tasks": len(completed), "total_tasks": len(tasks),
            "completed_topics": completed_topics, "total_topics": len(topics),
            "today_entries": [(t, e) for t in tasks
                              for e in state["tasks"].get(t["id"], {}).get("history", [])
                              if e["date"] == date],
            "recent_completions": sorted(completed,
                                         key=lambda t: state["tasks"][t["id"]]["updated"],
                                         reverse=True)[:5],
        },
        "upcoming_tracker": [t for t in tasks if t["id"] not in ids and
                             state["tasks"].get(t["id"], {}).get("status") != "done"][:2],
    }


def report(tasks, state, date):
    data = daily_status(tasks, state, date)
    lookup = {t["id"]: t for t in tasks}
    ids = [t["id"] for t in data["todays_focus"]]
    summary = data["progress_summary"]
    lines = ["Today's Focus", "==============",
             "One small practice task; read only the relevant part of the chapter."]
    if not ids:
        lines.append("- All planned tasks complete. Review an earlier exercise or rest.")
    for task_id in ids:
        task = lookup[task_id]
        record = state["tasks"].get(task_id, {})
        lines += [f"- [{record.get('status', 'not_started')}] {task_id}: {task['title']}",
                  f"  Topic: {task.get('topic', task['title'])}",
                  f"  Reading: {task.get('chapter_label', 'See the exercise instructions')}",
                  *([f"  Link: {task['resource_url']}"] if task.get('resource_url') else []),
                  f"  Time: approximately {task.get('estimated_minutes', 15)} minutes; continue next time if needed.",
                  f"  Cover: {task['instruction']}", f"  File: {task['path']}",
                  *([f"  Instructions: {task['instructions_path']}"] if task.get('instructions_path') else []),
                  f"  Run: {task['command']}"]
    lines += ["", "Progress Summary", "================",
              f"Completed so far: {summary['completed_tasks']}/{summary['total_tasks']} tasks; "
              f"{summary['completed_topics']}/{summary['total_topics']} topics.",
              "A topic is complete only when all its tasks are recorded done.", "", "Recorded today:"]
    events = summary["today_entries"]
    if not events:
        lines.append("- No progress recorded yet. This does not mean you did not practise.")
    for task, event in events:
        lines.append(f"- {task['title']}: {event['status']}" +
                     (f" — {event['note']}" if event["note"] else ""))
    lines += ["", "Recent completed work:"]
    completed = [(t, state["tasks"][t["id"]]) for t in summary["recent_completions"]]
    if not completed:
        lines.append("- None recorded yet.")
    for task, record in completed:
        lines.append(f"- {record['updated']}: {task['title']}" +
                     (f" — {record['note']}" if record["note"] else ""))
    lines += ["", "Upcoming Tracker", "================"]
    lines += [f"- {t['id']}: {t['title']}\n  Topic: {t.get('topic', t['title'])}\n"
              f"  Reading: {t.get('chapter_label', 'See the exercise instructions')}"
              for t in data["upcoming_tracker"]] or ["- Review or choose a new topic."]
    lines += ["", "Record progress using the 'Record Rust progress' workflow on GitHub,",
              "or: python3 scripts/progress.py update --status in_progress --note \"What I tried\"",
              "When finished: python3 scripts/progress.py update --status done --note \"What I learned\"",
              "Local updates must be committed and pushed before GitHub emails can read them.",
              "Completion is self-reported. Email replies are not processed."]
    repository = os.environ.get("GITHUB_REPOSITORY")
    if repository:
        lines.append(f"Update on GitHub: https://github.com/{repository}/actions/workflows/record-progress.yml")
    return "\n".join(lines)


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    sub = parser.add_subparsers(dest="action", required=True)
    sub.add_parser("status")
    sub.add_parser("plan")
    update = sub.add_parser("update")
    update.add_argument("--task", help="Task ID; defaults to today's first incomplete task")
    update.add_argument("--status", choices=("in_progress", "done", "not_started"), required=True)
    update.add_argument("--note", default="", help="Your learning summary or difficulty")
    args = parser.parse_args()
    tasks, state = load()
    date = today()
    ids = plan(tasks, state, date)
    if args.action == "update":
        task_id = args.task or next((i for i in ids if state["tasks"].get(i, {}).get("status") != "done"), None)
        if task_id not in {t["id"] for t in tasks}:
            parser.error("Choose a valid --task ID from status; today's plan may already be complete.")
        if task_id not in ids:
            ids.append(task_id)
        record = state["tasks"].setdefault(task_id, {"history": []})
        event = {"date": date, "status": args.status, "note": args.note}
        record.update({"status": args.status, "updated": date, "note": args.note})
        record["history"].append(event)
    if args.action in ("plan", "update"):
        save(state)
    print(report(tasks, state, date))


if __name__ == "__main__":
    main()
