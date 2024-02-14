import random
# Get the list of all the pokemon species from the file species_names.txt
with open("data/species_names.txt", "r") as f:
    species_names = f.readlines()

lines = []

with open("../src/starter_choose.c", "r") as f:
        lines += f.readlines()

modified_lines = []

# Check if the line is a specie
for line in lines:
    if line.startswith("    SPECIES_"):
        random_specie = random.choice(species_names)
        species_names.remove(random_specie)
        modified_line = f"    SPECIES_{random_specie.strip()},\n"
        modified_lines.append(modified_line)
    else:
        modified_lines.append(line)

# Write to a temp file
with open("../src/starter_choose.c", "w") as f:
    f.writelines(modified_lines)

