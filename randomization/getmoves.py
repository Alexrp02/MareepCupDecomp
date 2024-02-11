lines = []
with open('./raw_data/raw_moves.txt', 'r') as f:
    lines = f.readlines()

move_names = []
for line in lines:
    if line.startswith("    [MOVE_"):
        move_names.append(line.strip().split('"')[1])

with open("./data/move_names.txt", "w") as f:
    f.write('\n'.join(move_names))