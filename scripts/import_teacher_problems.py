"""Convert the teacher problem ZIP into Supabase SQL and an offline seed."""

from __future__ import annotations

import json
import re
import sys
import zipfile
from pathlib import Path


CATEGORY = "Teacher Problems"


def clean_text(value: str) -> str:
    return value.replace("\ufeff", "").replace("\r\n", "\n").strip()


def entry_text(archive: zipfile.ZipFile, name: str) -> str:
    return clean_text(archive.read(name).decode("utf-8"))


def section(markdown: str, heading: str) -> str:
    match = re.search(
        rf"^##\s+{re.escape(heading)}\s*$([\s\S]*?)(?=^##\s+|\Z)",
        markdown,
        flags=re.MULTILINE,
    )
    if not match:
        raise ValueError(f"Missing section {heading!r}")
    return clean_text(match.group(1))


def sample_block(markdown: str, label: str) -> str:
    match = re.search(
        rf"\*\*{re.escape(label)}\*\*\s*\n```(?:\w+)?\n([\s\S]*?)\n```",
        markdown,
    )
    if not match:
        raise ValueError(f"Missing sample {label!r}")
    return clean_text(match.group(1))


def problem_title(markdown: str, number: int) -> str:
    heading = re.search(r"^#\s+(.+)$", markdown, flags=re.MULTILINE)
    if not heading:
        raise ValueError(f"Missing title for test {number}")
    title = heading.group(1).strip()
    title = re.sub(r"^ข้อที่\s*\d+\s*\([^)]*\):\s*", "", title)
    return title


def sql_literal(value: str) -> str:
    return "'" + value.replace("'", "''") + "'"


def main() -> None:
    if len(sys.argv) != 2:
        raise SystemExit("usage: import_teacher_problems.py <zip>")

    zip_path = Path(sys.argv[1])
    repo_root = Path(__file__).resolve().parents[1]
    json_path = repo_root / "src-tauri" / "teacher_problems.json"
    sql_path = repo_root / "supabase" / "teacher_problems.sql"
    migration_path = repo_root / "supabase" / "migrations" / "20260808000000_add_teacher_problems.sql"

    problems: list[dict[str, str]] = []
    test_cases: list[dict[str, object]] = []

    with zipfile.ZipFile(zip_path) as archive:
        names = set(archive.namelist())
        for number in range(1, 31):
            prefix = f"python_problem_set_30questions/test{number}"
            markdown = entry_text(archive, f"{prefix}.md")
            problem_id = f"teacher_{number:02d}"
            problems.append(
                {
                    "id": problem_id,
                    "title": problem_title(markdown, number),
                    "category": CATEGORY,
                    "description": section(markdown, "โจทย์"),
                    "input_specification": section(markdown, "รูปแบบข้อมูลนำเข้า (Input)"),
                    "output_specification": section(markdown, "รูปแบบข้อมูลส่งออก (Output)"),
                }
            )

            test_cases.append(
                {
                    "id": f"{problem_id}_sample",
                    "problem_id": problem_id,
                    "input": sample_block(markdown, "Input"),
                    "expected_output": sample_block(markdown, "Output"),
                    "visible": True,
                }
            )

            for case_number in range(1, 6):
                input_name = f"{prefix}_case{case_number}.in"
                answer_name = f"{prefix}_case{case_number}.ans"
                if input_name not in names or answer_name not in names:
                    raise ValueError(f"Missing test case {number}/{case_number}")
                test_cases.append(
                    {
                        "id": f"{problem_id}_case_{case_number}",
                        "problem_id": problem_id,
                        "input": entry_text(archive, input_name),
                        "expected_output": entry_text(archive, answer_name),
                        "visible": False,
                    }
                )

    payload = {"category": CATEGORY, "problems": problems, "test_cases": test_cases}
    json_path.write_text(json.dumps(payload, ensure_ascii=False, indent=2) + "\n", encoding="utf-8")

    sql_lines = [
        "-- Generated from python_problem_set_30questions.zip.",
        "-- Teacher Problems is intentionally excluded from the daily quota by default;",
        "-- the app exposes the full catalogue in its dedicated featured section.",
        "begin;",
        "",
        "insert into category_configs (category, target_count)",
        f"values ({sql_literal(CATEGORY)}, 0)",
        "on conflict (category) do update set target_count = excluded.target_count;",
        "",
    ]

    for problem in problems:
        columns = "id, title, category, description, input_specification, output_specification"
        values = ", ".join(sql_literal(str(problem[key])) for key in columns.split(", "))
        sql_lines.extend(
            [
                f"insert into problems ({columns}) values ({values})",
                "on conflict (id) do update set",
                "  title = excluded.title, category = excluded.category, description = excluded.description,",
                "  input_specification = excluded.input_specification, output_specification = excluded.output_specification;",
                "",
            ]
        )

    for test_case in test_cases:
        columns = "id, problem_id, input, expected_output"
        values = ", ".join(sql_literal(str(test_case[key])) for key in columns.split(", "))
        table = "public_test_cases" if test_case["visible"] else "private_test_cases"
        sql_lines.extend(
            [
                f"insert into {table} ({columns}) values ({values})",
                "on conflict (id) do update set",
                "  problem_id = excluded.problem_id, input = excluded.input, expected_output = excluded.expected_output;",
                "",
            ]
        )

    sql_lines.extend(["commit;", ""])
    sql_content = "\n".join(sql_lines)
    sql_path.write_text(sql_content, encoding="utf-8")
    migration_path.write_text(sql_content, encoding="utf-8")
    print(f"Generated {len(problems)} problems and {len(test_cases)} test cases")


if __name__ == "__main__":
    main()
