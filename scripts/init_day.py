#!/usr/bin/env python3

import subprocess
import argparse
import inspect


def main():
    args = parse_args()
    day_format = f"day{str(args.day).zfill(2)}"
    tests_folder = f"tests/{day_format}"

    # Create tests folder & add day's input
    subprocess.run(["mkdir", "-p", tests_folder])
    subprocess.run(["aocdl", "-year", "2022", "-day", str(args.day)])
    subprocess.run(["mv", "-f", "input.txt", tests_folder])
    subprocess.run(["git", "add", tests_folder])

    # Create files for part 1 and 2
    for file in [f"src/{day_format}_a.rs", f"src/{day_format}_b.rs"]:
        with open(file, "w") as f:
            f.write(
                inspect.cleandoc(
                    """
            use std::io::{self, BufRead};

            pub fn solve() -> io::Result<()> {
                for line in io::stdin().lock().lines().map(|l| l.unwrap()){
                    // Here's your code
                }

                Ok(())
            }
            """
                )
            )
        subprocess.run(["git", "add", file])

    # Update main file
    main_file = open("src/main.rs", "r").read()
    main_file = f"mod {day_format}_a;\nmod {day_format}_b;\n" + main_file
    main_file = main_file.replace(
        "    match args.problem.as_str() {\n",
        (
            "    match args.problem.as_str() {\n"
            f'        "{day_format}_a" => {day_format}_a::solve().unwrap(),\n'
            f'        "{day_format}_b" => {day_format}_b::solve().unwrap(),\n'
        ),
    )

    with open("src/main.rs", "w") as f:
        f.write(main_file)
    subprocess.run(["git", "add", main_file])


def parse_args():
    parser = argparse.ArgumentParser()
    parser.add_argument("day", type=int)
    args = parser.parse_args()

    if args.day not in range(1, 32):
        raise ValueError(f"Illegal day value: {args.day}")

    return args


if __name__ == "__main__":
    main()
