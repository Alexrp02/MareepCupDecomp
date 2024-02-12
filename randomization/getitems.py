lines = []
with open('./raw_data/raw_items.txt', 'r') as f:
    lines = f.readlines()

move_names = []
for line in lines:
    if line.startswith("#define ITEM_"):
        move_names.append(line.strip().split(' ')[1].split('ITEM_')[1])

with open("./data/item_names.txt", "w") as f:
    f.write('\n'.join(move_names))