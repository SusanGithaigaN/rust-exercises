"""Infrastructure checks; these do not supply answers to Rust assignments."""

from datetime import datetime
import importlib
import json
from pathlib import Path
import sys
import tempfile
import unittest
from unittest.mock import patch
from zoneinfo import ZoneInfo

sys.path.insert(0, str(Path(__file__).resolve().parents[1] / "scripts"))
progress = importlib.import_module("progress")
reminder = importlib.import_module("send_reminder")


class ProgressTests(unittest.TestCase):
    def setUp(self):
        self.tasks = [dict(id=str(i), title=f"Task {i}", instruction="Small requirement",
                           path="example.rs", command="cargo run") for i in range(3)]
        self.state = {"tasks": {}, "days": {}}

    def test_unfinished_work_carries_forward_and_completed_work_advances(self):
        self.assertEqual(progress.plan(self.tasks, self.state, "2026-10-05"), ["0"])
        self.state["tasks"]["0"] = {"status": "in_progress"}
        self.assertEqual(progress.plan(self.tasks, self.state, "2026-10-06"), ["0"])
        self.state["tasks"]["0"]["status"] = "done"
        self.assertEqual(progress.plan(self.tasks, self.state, "2026-10-07"), ["1"])

    def test_same_day_plan_does_not_change_after_completion(self):
        progress.plan(self.tasks, self.state, "2026-10-05")
        self.state["tasks"]["0"] = {"status": "done"}
        self.assertEqual(progress.plan(self.tasks, self.state, "2026-10-05"), ["0"])

    def test_reports_no_activity_without_claiming_completion(self):
        text = progress.report(self.tasks, self.state, "2026-10-05")
        self.assertIn("0/3", text)
        self.assertIn("No progress recorded yet", text)
        self.assertIn("not_started", text)

    def test_saved_progress_and_history_appear_in_both_emails(self):
        with tempfile.TemporaryDirectory() as directory:
            catalog, state = Path(directory)/"tasks.json", Path(directory)/"progress.json"
            catalog.write_text(json.dumps(self.tasks))
            with patch.object(progress, "CATALOG", catalog), patch.object(progress, "STATE", state), \
                 patch.object(progress, "today", return_value="2026-10-05"), \
                 patch("builtins.print"):
                with patch("sys.argv", ["progress", "update", "--status", "in_progress", "--note", "Borrowing still confusing"]):
                    progress.main()
                with patch("sys.argv", ["progress", "update", "--status", "done", "--note", "Explained ownership"]):
                    progress.main()
                _, saved = progress.load()
                self.assertEqual(len(saved["tasks"]["0"]["history"]), 2)
                now = datetime(2026, 10, 5, 20, 30, tzinfo=ZoneInfo("Africa/Nairobi"))
                for kind in ("morning", "evening"):
                    body = reminder.build_message(kind, "example@example.com", now).get_content()
                    self.assertIn("Explained ownership", body)
                    self.assertIn("Borrowing still confusing", body)
                    self.assertIn("1/3", body)
                    self.assertIn("Task 1", body)

    def test_completed_catalog_has_no_new_assignment(self):
        for task in self.tasks:
            self.state["tasks"][task["id"]] = {"status": "done", "updated": "2026-10-05", "note": ""}
        self.assertEqual(progress.plan(self.tasks, self.state, "2026-10-06"), [])

    def test_topic_completion_requires_every_task_and_reopening_reverses_count(self):
        for task in self.tasks:
            task['topic_id'] = 'foundations'
        for task in self.tasks[:2]:
            self.state['tasks'][task['id']] = {'status': 'done', 'updated': '2026-10-05', 'note': ''}
        self.assertEqual(progress.daily_status(self.tasks, self.state, '2026-10-05')['progress_summary']['completed_topics'], 0)
        self.state['tasks']['2'] = {'status': 'done', 'updated': '2026-10-05', 'note': ''}
        self.assertEqual(progress.daily_status(self.tasks, self.state, '2026-10-05')['progress_summary']['completed_topics'], 1)
        self.state['tasks']['0']['status'] = 'in_progress'
        self.assertEqual(progress.daily_status(self.tasks, self.state, '2026-10-05')['progress_summary']['completed_topics'], 0)

    def test_three_sections_appear_in_order_with_reading_metadata(self):
        self.tasks[0].update(topic='Variables', chapter_label='Chapter 3', resource_url='https://example.com/book', estimated_minutes=15)
        text = progress.report(self.tasks, self.state, '2026-10-05')
        headings = ["Today's Focus", 'Progress Summary', 'Upcoming Tracker']
        positions = [text.index(h) for h in headings]
        self.assertEqual(positions, sorted(positions))
        for value in ('Variables', 'Chapter 3', 'https://example.com/book'):
            self.assertIn(value, text)

    def test_real_catalog_paths_and_ids_are_valid(self):
        tasks, _ = progress.load()
        self.assertEqual(len({t["id"] for t in tasks}), len(tasks))
        for task in tasks:
            self.assertTrue((progress.ROOT/task["path"]).is_file())


if __name__ == "__main__":
    unittest.main()
