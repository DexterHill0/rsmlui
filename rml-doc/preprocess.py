import os

# Remap Doxygen @name-group sections (kind="user-defined") to public-func so
# moxygen's default member filter includes them.

xml_dir = os.path.join(os.path.dirname(__file__), "generated", "xml")

for filename in os.listdir(xml_dir):
    if not filename.endswith(".xml"):
        continue

    path = os.path.join(xml_dir, filename)

    with open(path, "r", encoding="utf-8") as f:
        content = f.read()

    updated = content.replace('kind="user-defined"', 'kind="public-func"')

    if updated != content:
        with open(path, "w", encoding="utf-8") as f:
            f.write(updated)

