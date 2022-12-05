#!/usr/bin/env python3

import subprocess
import argparse
import os


def main():
    args = parse_args()
    day_format = f"day{str(args.day).zfill(2)}"
    tests_folder = f"tests/{day_format}"

    if args.release:
        subprocess.run(["cargo", "build", "--release"])
        executable = "target/release/aoc2022"
    else:
        subprocess.run(["cargo", "build"])
        executable = "target/debug/aoc2022"

    subproblem = f"{day_format}_{args.subproblem}"

    if not args.my_test_cases:
        subprocess.run(
            f"{executable} {subproblem} < {tests_folder}/input.txt", shell=True
        )
        exit(0)

    for testcase in {
        f"{tests_folder}/{test.replace('.in', '')}"
        for test in os.listdir(tests_folder)
        if test.endswith(".in")
    }:
        in_file = testcase + ".in"
        out_file = testcase + ".out"
        tmp_file = "/tmp/output.aoc"
        subprocess.run(f"{executable} {subproblem} < {in_file} > {tmp_file}", shell=True)
        print(f">>> diff for {out_file}")
        subprocess.run(["diff", tmp_file, out_file])


def parse_args():
    parser = argparse.ArgumentParser()
    parser.add_argument("day", type=int, help="Which day's problem to execute")
    parser.add_argument(
        "subproblem", choices=["a", "b"], help="Which subproblem to execute"
    )
    parser.add_argument(
        "-r", "--release", action="store_true", help="Run on release mode if set"
    )
    parser.add_argument(
        "--my-test-cases",
        action="store_true",
        help="If set, run on my test cases (if unset, on input.txt)",
    )
    args = parser.parse_args()

    if args.day not in range(1, 32):
        raise ValueError(f"Illegal day value: {args.day}")

    return args


if __name__ == "__main__":
    main()
