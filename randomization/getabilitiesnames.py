lines = []
with open('./raw_data/raw_abilities.txt', 'r') as f:
    lines = f.readlines()

ability_names = []
for line in lines:
    if line.startswith("    [ABILITY_"):
        ability_names.append(line.strip().split('"')[1])

with open("./data/ability_names.txt", "w") as f:
    f.write('\n'.join(ability_names))