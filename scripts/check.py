#!/usr/bin/env python3
"""Check one Rust exercise or one function from the repository root."""

import argparse
import json
from pathlib import Path
import re
import subprocess
import sys

ROOT = Path(__file__).resolve().parents[1]
MINIMUM_TESTS = {
    "read_first": 6,
    "exercise_01": 9,
    "exercise_02": 8,
    "exercise_03": 6,
    "exercise_04": 7,
}
OUTPUT_EXERCISES = {
    ("01_variables", "variables"): "8\nRust\ntrue\n",
    ("01_variables", "shadowing"): "6\n3\n",
}
PRINT_CHECKS = {
    ("12_traits_generics", "recall", "print_description"):
        ["Book: '" + "é" * 40 + "' by A, 1 pages", "Book: '" + "é" * 23 + "..."],
    ("12_traits_generics", "extend", "print_description"):
        ["Book: '" + "é" * 40 + "' by A, 1 pages", "Book: '" + "é" * 23 + "..."],
    ("12_traits_generics", "revisit", "send"):
        ["Email to a@x.com: ping", "Slack #ops: ping"],
    ("12_traits_generics", "revisit", "broadcast"):
        ["Email to a@x.com: ping"],
    ("12_traits_generics", "revisit", "notify_all"):
        ["Email to a@x.com: restart", "Slack #ops: restart"],
}


def run(command):
    result = subprocess.run(command, cwd=ROOT, text=True, capture_output=True)
    if result.stderr:
        print(result.stderr, end="", file=sys.stderr)
    if result.stdout:
        print(result.stdout, end="")
    return result


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("topic", help="Folder name, such as 01_variables")
    parser.add_argument("exercise", help="Binary name, such as recall")
    parser.add_argument("step", nargs="?", help="Function check, such as double")
    args = parser.parse_args()
    tasks = json.loads((ROOT / "learning/tasks.json").read_text())
    if not any(t["topic_id"] == args.topic and
               Path(t["path"]).stem == args.exercise for t in tasks):
        parser.error("Unknown topic or exercise. See the topic README for commands.")
    cargo = ["cargo", "test", "-p", f"ex_{args.topic}", "--bin", args.exercise]
    key = (args.topic, args.exercise)
    if key in OUTPUT_EXERCISES:
        if args.step:
            parser.error("This exercise has one output check; omit the step.")
        result = run(["cargo", "run", "--quiet", "-p", f"ex_{args.topic}",
                      "--bin", args.exercise])
        if result.returncode:
            return result.returncode
        if result.stdout != OUTPUT_EXERCISES[key]:
            print("Output does not match. Expected these lines:")
            print(OUTPUT_EXERCISES[key], end="")
            return 1
        print("Output check passed. Also check the required variable names and types.")
        return 0
    if args.step:
        listing = subprocess.run(cargo + ["--", "--list"], cwd=ROOT,
                                 text=True, capture_output=True)
        if listing.returncode:
            print(listing.stderr, end="", file=sys.stderr)
            return listing.returncode
        names = [line.removesuffix(": test") for line in listing.stdout.splitlines()
                 if line.endswith(f"::check_{args.step}: test")]
        if len(names) != 1:
            print("No unique check found for this step. Copy the command from the topic README.")
            return 1
        cargo += [names[0], "--", "--exact", "--nocapture", "--test-threads=1", "--format", "terse"]
    else:
        cargo += ["--", "--test-threads=1"]
    result = run(cargo)
    if result.returncode:
        return result.returncode
    counts = re.findall(r"test result: ok\. (\d+) passed;", result.stdout)
    passed = sum(int(count) for count in counts)
    minimum = MINIMUM_TESTS.get(args.exercise, 1) if args.topic == "14_writing_tests" else 1
    if passed < minimum:
        print(f"Incomplete: {passed} tests passed; this exercise requires at least {minimum}.")
        return 1
    expected = PRINT_CHECKS.get((*key, args.step))
    if expected:
        match = re.search(r"running 1 test\n(.*?)\ntest result:", result.stdout, re.S)
        actual = match.group(1).strip().removesuffix(".").strip() if match else ""
        if actual.splitlines() != expected:
            print("Printed lines do not match the required output:")
            print("\n".join(expected))
            return 1
    print("Check passed. Explain the result and try one new input before recording completion.")
    return 0


if __name__ == "__main__":
    sys.exit(main())
