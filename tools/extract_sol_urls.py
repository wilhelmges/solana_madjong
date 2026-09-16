import re

with open(
    r"C:\Users\user4\.local\share\opencode\tool-output\tool_0a9656df10019KOeklDY8bo2v8",
    encoding="utf-8",
    errors="replace",
) as f:
    t = f.read()

pat = re.compile(r"https://[^\"' >]*?(?:logo|mark|brand)[^\"' >]*?\.(?:svg|png)", re.I)
for u in sorted(set(pat.findall(t)))[:40]:
    print(u)
print("total:", len(set(pat.findall(t))))
