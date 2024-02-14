import random

with open ("data/ability_define.txt", "r") as f:
    ability_pool = f.readlines()
for i in range(1,10):
    
# Read your .h file line by line
    with open(f"../src/data/pokemon/species_info/gen_{i}.h", "r", encoding="utf-8") as file:
        lines = file.readlines()

# Iterate through each line
    new_lines = []
    # Variable to check if geodude family
    geodude = False

    for line in lines:
        # Check if the line starts with ".abilities = {"
        if line.startswith("#define KANTONIAN_GEODUDE_FAMILY_INFO"):
            geodude = True
            print("KANTONIAN_GEODUDE_FAMILY_INFO")
        if line.startswith("        .abilities = {"):
            # Extract the current abilities list
            abilities_string = line.split("=")[1].strip().split("}")[0].strip()
            abilities_list = [ability.strip() for ability in abilities_string.split(",")]

            new_abilities_list = random.sample(ability_pool, len(abilities_list))
            new_abilities_list = [ability.strip() for ability in new_abilities_list]

            # Update the line with the new abilities list
            new_line = "        .abilities = {" + ', '.join(new_abilities_list) + "}"
            if not geodude:
                new_line = new_line + ",\t\\\n"
            else:
                new_line = new_line + "\n"
                geodude = False
                print("Geodude line!")
        else:
            # Keep the line unchanged
            new_line = line

        # Add the updated line to the new list
        new_lines.append(new_line)

# Write the modified lines back to the file
    with open(f"../src/data/pokemon/species_info/gen_{i}.h", "w", encoding="utf-8") as file:
        file.writelines(new_lines)

    print(f"Abilities successfully randomized in gen{i}.h!")
