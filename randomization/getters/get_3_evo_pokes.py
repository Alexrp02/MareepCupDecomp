lines = []
for i in range(1, 10):
    with open(f'../src/data/pokemon/species_info/gen_{i}.h') as f:
        lines += f.readlines()

possible_poke = ""
previous_had_evolution = False
poke_have_evolution = False
evolutions = 1
poke_with_3_evolutions = []

for line in lines:
    if (line.startswith('    [SPECIES_') and not previous_had_evolution):
        possible_poke = line.split('[')[1].split(']')[0]
        evolutions = 1
    elif (line.startswith('    [SPECIES_') and previous_had_evolution):
        evolutions += 1

    if line.startswith('        .evolutions'):
        poke_have_evolution = True

    if line.startswith('    }') and poke_have_evolution:
        poke_have_evolution = False
        previous_had_evolution = True
    elif line.startswith('    }') and not poke_have_evolution:
        previous_had_evolution = False
        if evolutions == 3:
            poke_with_3_evolutions.append(possible_poke)

# Save the list of pokemon with 3 evolutions
with open('data/poke_with_3_evolutions.txt', 'w') as f:
    f.write('\n'.join(poke_with_3_evolutions))
