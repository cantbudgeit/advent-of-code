listOfPasswords = []
with open('input.txt') as inputFile:
    lines = inputFile.readlines()
    for line in lines:
        listOfPasswords.append(line.strip())

avalidPasswords = 0

for line in listOfPasswords:
    arangeStart = int(line.split('-')[0])
    arangeEnd = int(line.split('-')[1].split()[0])
    apolicyLetter = line.split('-')[1].split()[1].split(':')[0]
    apassword = line.split()[2]
    
