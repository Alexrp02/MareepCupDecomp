# This script extracts all SPECIES_... from a Pokemon .h file

# Open the files from 1 to 9 and extract the species names
lines = []
for i in range(1, 10):
    print(f"Extracting species from gen_{i}.h")
    with open(f"./pokeemerald-expansion/src/data/pokemon/species_info/gen_{i}.h", "r") as f:
        lines += f.readlines()

# with open("./pokeemerald-expansion/src/data/pokemon/species_info/gen_1.h", "r") as f:
#     lines = f.readlines()

species_names = []
for line in lines:
    if line.startswith("    [SPECIES_"):
        species_names.append(line.strip().split("[")[1].split("]")[0])

print("Extracted Pokemon species:")
for species_name in species_names:
    print(species_name)
print(f"Total: {len(species_names)}")

# Write the species names to a file
with open("species_names.txt", "w") as f:
    for species_name in species_names:
        f.write(species_name + "\n")