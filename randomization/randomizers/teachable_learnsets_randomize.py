import random
# Get the list of all the pokemon species from the file species_names.txt
with open("data/move_define.txt", "r") as f:
    move_names = f.readlines()

lines = []
move_names_copy = move_names.copy()

with open("../src/data/pokemon/teachable_learnsets.h", "r") as f:
        lines += f.readlines()

modified_lines = []

has_to_reset = False

# Check if the line is a move
for line in lines:
    if line.startswith("    MOVE_"):
        has_to_reset = True
        move = random.choice(move_names_copy)
        move_names_copy.remove(move)
        modified_line = f"    {move.strip()},\n"
        modified_lines.append(modified_line)
    else:
        modified_lines.append(line)
        if has_to_reset:
            move_names_copy = move_names.copy()
            has_to_reset = False


# Write to a temp file
with open("../src/data/pokemon/teachable_learnsets.h", "w") as f:
    f.writelines(modified_lines)
