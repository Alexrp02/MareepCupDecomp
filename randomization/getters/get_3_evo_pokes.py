# Open gen 1 file
with open('../src/data/pokemon/species_info/gen_4.h') as f:
    lines = f.readlines()

possible_poke = ""
previous_had_evolution = False
poke_have_evolution = False
evolutions = 1

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
        print(possible_poke + " " + str(evolutions) + " evolutions")
