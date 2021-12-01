listOfEntries = []
with open('input.txt') as inputFile:
    lines = inputFile.readlines()
    for line in lines:
        listOfEntries.append(int(line.strip()))

breaker = False
for x in listOfEntries:
    for y in listOfEntries:
        if (x + y == 2020 ):
            print(f"Answer!! x:{x} and y:{y} and multiplied they are {x*y}")
            breaker = True
            break
    if breaker:
        break