lines = []
with open("./raw_data/raw_abilities.txt", "r") as f:
    lines = f.readlines()

move_names = []
for line in lines:
    if len(move_names) == 848:
        break
    if line.startswith("    [ABILITY_"):
        move_names.append(line.strip().split("[")[1].split("]")[0])

with open("./data/ability_define.txt", "w") as f:
    f.write("\n".join(move_names))

