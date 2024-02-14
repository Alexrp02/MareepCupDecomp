# Savefile tool for Mareep Cup

Cargo application to read from savefile of MareepCup and retrieve the team in showdown format

## TO-DO
- [X] Util function to get a given section id offset
- [X] Get pokemon personality to get the substructure order
- [X] Get the OT-ID
- [X] Get the decryption key (OT-ID xor Personality)
- [X] Decrypt the data (xor the decryption key 4 bytes a time (data is 48 bytes))
- [ ] Check with checksum if the data is correct
- [ ] Handle substructure order to get data of pokemon
- [ ] Read functions for every data of the pokemon
- [ ] Get all the team data to a class Pokemon Team
- [ ] Format the data to showdown in a txt file