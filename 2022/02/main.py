
with open("input.txt") as f:
    lines = f.readlines()

lines = [line.strip() for line in lines]

rock_1 = 'A'
paper_1 = 'B'
scissors_1 = 'C'

rock_2 = 'X'
paper_2 = 'Y'
scissors_2 = 'Z'

total = 0

for line in lines:
    if line[0] == rock_1 and line[2] == rock_2:
        total += 3
    elif line[0] == paper_1 and line[2] == paper_2:
        total += 3
    elif line[0] == scissors_1 and line[2] == scissors_2:
        total += 3
    elif line[0] == rock_1 and line[2] == paper_2:
        total += 6
    elif line[0] == paper_1 and line[2] == scissors_2:
        total += 6
    elif line[0] == scissors_1 and line[2] == rock_2:
        total += 6
    
    if line[2] == rock_2:
        total += 1
    elif line[2] == paper_2:
        total += 2
    elif line[2] == scissors_2:
        total += 3

print(total)

total = 0

for line in lines:
        
    if line[2] == rock_2:
        
        if line[0] == rock_1:
            total += 3
        elif line[0] == paper_1:
            total += 1
        elif line[0] == scissors_1:
            total += 2

    elif line[2] == paper_2:
        total += 3


        if line[0] == rock_1:
            total += 1
        elif line[0] == paper_1:
            total += 2
        elif line[0] == scissors_1:
            total += 3

    elif line[2] == scissors_2:
        total += 6



        if line[0] == rock_1:
            total += 2
        elif line[0] == paper_1:
            total += 3
        elif line[0] == scissors_1:
            total += 1

print(total)