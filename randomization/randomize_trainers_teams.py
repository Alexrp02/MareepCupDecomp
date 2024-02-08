import random
# Get the list of all the pokemon species from the file species_names.txt
with open("species_names.txt", "r") as f:
    species_names = f.readlines()

lines = []

with open("pokeemerald-expansion/src/data/trainer_parties.h", "r") as f:
        lines += f.readlines()

modified_lines = []

# Check if the line is a specie
for line in lines:
    if line.startswith("    .species"):
        modified_line = f"    .species = {random.choice(species_names).strip()},\n"
        modified_lines.append(modified_line)
    else:
        modified_lines.append(line)

# Write to a temp file
with open("pokeemerald-expansion/src/data/trainer_parties.h", "w") as f:
    f.writelines(modified_lines)