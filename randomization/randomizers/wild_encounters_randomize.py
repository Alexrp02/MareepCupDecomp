import json
import random

# Take species list from species_names.txt
with open("../data/species_names.txt", "r") as f:
  species_pool = f.readlines()
  species_pool = [specie.strip() for specie in species_pool]

def modify_species(data):
  # Recursively iterate through the JSON data
  if isinstance(data, dict):
    for key, value in data.items():
      # Replace "species" fields with random specie
      if key == "species":
        data[key] = random.choice(species_pool)
      # Recursively modify nested dictionaries
      modify_species(value)
  elif isinstance(data, list):
    # Iterate through list elements and modify nested dictionaries
    for item in data:
      modify_species(item)

# Load JSON data
with open("../../src/data/wild_encounters.json", "r") as f:
  data = json.load(f)

# Modify species
modify_species(data)

# Save modified data
with open("../../src/data/wild_encounters.json", "w") as f:
  json.dump(data, f, indent=4)

print("Successfully modified species in your JSON file!")