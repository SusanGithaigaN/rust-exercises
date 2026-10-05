"""Verify exercise commands without completing the learner's practice files."""

import contextlib
import importlib.util
import io
import json
from pathlib import Path
import re
import shutil
import tempfile
import unittest
from unittest.mock import patch

ROOT = Path(__file__).resolve().parents[1]
spec = importlib.util.spec_from_file_location("exercise_check", ROOT / "scripts/check.py")
check = importlib.util.module_from_spec(spec)
spec.loader.exec_module(check)


class CatalogTests(unittest.TestCase):
    def test_every_task_has_instructions_and_a_valid_command(self):
        tasks = json.loads((ROOT / "learning/tasks.json").read_text())
        for task in tasks:
            with self.subTest(task=task["id"]):
                guide = ROOT / task["instructions_path"]
                self.assertTrue(guide.is_file())
                self.assertTrue((ROOT / task["path"]).is_file())
                self.assertIn(task["command"], guide.read_text())
                if not task["command"].startswith("python3 scripts/check.py"):
                    continue
                arguments = task["command"].split()[2:]
                topic, binary = arguments[:2]
                self.assertEqual(topic, task["topic_id"])
                self.assertEqual(binary, Path(task["path"]).stem)
                if len(arguments) == 3:
                    checks = ROOT / "exercises" / topic / "checks" / (binary + ".rs")
                    self.assertRegex(checks.read_text(), r"fn check_" + re.escape(arguments[2]) + r"\(")

    def test_basics_start_with_variables_and_reviews_follow_the_main_sequence(self):
        tasks = json.loads((ROOT / "learning/tasks.json").read_text())
        self.assertEqual(Path(tasks[0]["path"]).stem, "variables")
        self.assertEqual(Path(tasks[1]["path"]).stem, "shadowing")
        first_review = next(i for i, t in enumerate(tasks) if Path(t["path"]).stem == "revisit")
        self.assertTrue(all(Path(t["path"]).stem == "revisit" for t in tasks[first_review:]))


@unittest.skipUnless(shutil.which("cargo"), "Cargo is required for exercise integration checks")
class CheckCommandTests(unittest.TestCase):
    def setUp(self):
        self.directory = tempfile.TemporaryDirectory()
        self.addCleanup(self.directory.cleanup)
        self.root = Path(self.directory.name)
        (self.root / "learning").mkdir()
        (self.root / "Cargo.toml").write_text('[package]\nname = "ex_fixture"\nversion = "0.1.0"\nedition = "2021"\n')
        (self.root / "src/bin").mkdir(parents=True)

    def invoke(self, source, step=None, topic="fixture", binary="recall"):
        manifest = '[package]\nname = "ex_' + topic + '"\nversion = "0.1.0"\nedition = "2021"\n'
        (self.root / "Cargo.toml").write_text(manifest)
        (self.root / "src/bin" / (binary + ".rs")).write_text(source)
        (self.root / "learning/tasks.json").write_text(json.dumps([
            {"topic_id": topic, "path": "src/bin/" + binary + ".rs"}
        ]))
        output = io.StringIO()
        argv = ["check", topic, binary] + ([step] if step else [])
        with patch.object(check, "ROOT", self.root), patch("sys.argv", argv), \
                contextlib.redirect_stdout(output), contextlib.redirect_stderr(output):
            result = check.main()
        return result, output.getvalue()

    def test_one_function_check_does_not_run_an_unfinished_sibling(self):
        source = '''fn main() {}
        mod tests {
            #[test] fn check_finished() { assert_eq!(2 + 2, 4); }
            #[test] fn check_unfinished() { todo!(); }
        }'''
        result, output = self.invoke(source, "finished")
        self.assertEqual(result, 0, output)
        result, output = self.invoke(source, "unfinished")
        self.assertNotEqual(result, 0)
        self.assertIn("not yet implemented", output)

    def test_missing_filter_and_zero_tests_do_not_pass(self):
        for step in (None, "missing"):
            result, output = self.invoke("fn main() {}", step)
            self.assertNotEqual(result, 0, output)

    def test_test_writing_requires_the_minimum_count(self):
        source = "fn main() {}\n#[test] fn a_single_test() { assert!(true); }"
        result, output = self.invoke(source, topic="14_writing_tests", binary="exercise_01")
        self.assertNotEqual(result, 0)
        self.assertIn("requires at least 9", output)
        source = "fn main() {}\n" + "\n".join(
            f"#[test] fn case_{i}() {{ assert_eq!({i}, {i}); }}" for i in range(9)
        )
        result, output = self.invoke(source, topic="14_writing_tests", binary="exercise_01")
        self.assertEqual(result, 0, output)

    def test_variables_require_the_expected_output(self):
        result, output = self.invoke('fn main() { println!("8\\nRust\\ntrue"); }',
                                     topic="01_variables", binary="variables")
        self.assertEqual(result, 0, output)
        result, output = self.invoke('fn main() { println!("5"); }',
                                     topic="01_variables", binary="variables")
        self.assertNotEqual(result, 0)
        self.assertIn("Output does not match", output)

    def test_console_checks_verify_text_even_when_the_test_passes(self):
        for text, expected in (("Email to a@x.com: ping", 0), ("wrong text", 1)):
            source = ('fn main() {}\nmod tests {\n#[test] fn check_broadcast() { '
                      'println!("' + text + '"); }\n}')
            result, output = self.invoke(source, "broadcast", "12_traits_generics", "revisit")
            self.assertEqual(result, expected, output)


if __name__ == "__main__":
    unittest.main()
