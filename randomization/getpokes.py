# This script extracts all SPECIES_... from a Pokemon .h file

import re

# Open the files from 1 to 9 and extract the species names
lines = []
print(f"Extracting species from species.h")
with open(f"../include/constants/species.h", "r") as f:
    lines += f.readlines()

# with open("./pokeemerald-expansion/src/data/pokemon/species_info/gen_1.h", "r") as f:
#     lines = f.readlines()

species_names = []
for line in lines:
    separator = r"\s+"
    if line.startswith("#define SPECIES_"):
        # print(re.split(separator, line))
        split = re.split(separator, line)
        for word in split:
            if word.isnumeric():
                species_names.append(split[1].split("SPECIES_")[1])

print("Extracted Pokemon species:")
for species_name in species_names:
    print(species_name)
print(f"Total: {len(species_names)}")

# Write the species names to a file
with open("data/species_names.txt", "w") as f:
    for species_name in species_names:
        f.write(species_name + "\n")