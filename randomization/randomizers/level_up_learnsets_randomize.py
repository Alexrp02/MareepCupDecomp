import random

with open("data/move_define.txt", "r") as f:
    move_names = f.readlines()

lines = []

with open("../../src/data/pokemon/level_up_learnsets.h", "r") as f:
        lines += f.readlines()

modified_lines = []
# Temporal array of moves in order to not have repeated moves
temporal_move_names = move_names.copy()
for line in lines:
    # Handle not repeated moves
    has_to_reset = True
    if line.startswith("    LEVEL_UP_MOVE("):
        if (line.split("(")[1].split(",")[0].strip().isnumeric()):
            if (int(line.split("(")[1].split(",")[0].strip()) <= 1 and has_to_reset):
                temporal_move_names = move_names.copy()
                has_to_reset = False
            elif (int(line.split("(")[1].split(",")[0].strip()) > 1):
                has_to_reset = True

        split_line = line.split(",")
        move = random.choice(temporal_move_names)
        temporal_move_names.remove(move)
        split_line[1] = f" {move.strip()})"
        modified_lines.append(",".join(split_line))
    else:
        modified_lines.append(line)

# Write to a temp file
with open("../../src/data/pokemon/level_up_learnsets.h", "w") as f:
    f.writelines(modified_lines)