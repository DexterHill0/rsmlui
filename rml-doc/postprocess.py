import os
import re

NOTE_REGEX = re.compile("(?P<note_start>:::(?P<kind>note|tip|info|warning|danger))(?P<content>.*?)(?P<note_end>:::)", flags=re.RegexFlag.S)

# Replaces docasaurus callout syntax with html notes

md_dir = os.path.join(os.path.dirname(__file__), "generated", "md")

for filename in os.listdir(md_dir):
    if not filename.endswith(".md"):
        continue

    path = os.path.join(md_dir, filename)

    with open(path, "r", encoding="utf-8") as f:
        content = f.read()

    changed = False

    while True:
        match = NOTE_REGEX.search(content)
        if match is None:
            break
        
        changed = True

        kind = match.group("kind")
        inner = match.group("content")
        (note_start_span, note_end_span) = (match.start("note_start"), match.end("note_end"))

        # currently only warning is supported in rustdoc
        newkind = {
            "note": "warning",
            "tip": "warning",
            "info": "warning",
            "warning": "warning",
            "danger": "warning",
        }[kind]

        content = content[:note_start_span] + f"<div class=\"{newkind}\">\n{inner}</div>\n" + content[note_end_span:]

    if changed:
        with open(path, "w", encoding="utf-8") as f:
            f.write(content)
