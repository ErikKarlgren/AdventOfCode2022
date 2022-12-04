#!/usr/bin/env python3

import subprocess
import argparse
import inspect


def main():
    args = parse_args()
    day_format = f"day{str(args.day).zfill(2)}"
    tests_folder = f"tests/{day_format}"

    subprocess.run(["mkdir", "-p", tests_folder])
    subprocess.run(["aocdl", "-year", "2022", "-day", str(args.day)])
    subprocess.run(["mv", "-f", "input.txt", tests_folder])

    for file in [f"src/{day_format}_a.rs", f"src/{day_format}_b.rs"]:
        with open(file, "w") as f:
            f.write(
                inspect.cleandoc(
                    """
            use std::io;

            pub fn solve() -> io::Result<()> {
                for line in io::stdin().lock().lines().map(|l| l.unwrap){
                    // Here's your code
                }

                Ok(())
            }
            """
                )
            )

    main_file = open("src/main.rs", "r").read()
    main_file = f"mod {day_format}_a;\nmod {day_format}_b;\n" + main_file
    main_file = main_file.replace(
        "    match problem {\n",
        (
            "    match problem {\n"
            f'        "{day_format}_a" => {day_format}_a::solve().unwrap(),\n'
            f'        "{day_format}_b" => {day_format}_b::solve().unwrap(),\n'
        ),
    )

    with open("src/main.rs", "w") as f:
        f.write(main_file)


def parse_args():
    parser = argparse.ArgumentParser()
    parser.add_argument("day", type=int)
    args = parser.parse_args()

    if args.day not in range(1, 32):
        raise ValueError(f"Illegal day value: {args.day}")

    return args


if __name__ == "__main__":
    main()
