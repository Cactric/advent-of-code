file = open("input.txt")
lines = file.readlines()

position = 50
zeros = 0
clicks = 0

for instruction in lines:
    if len(instruction) < 2:
        continue
    
    direction = instruction[0]
    amount = int(instruction[1:])
    
    if direction == 'L':
        while amount > 0:
            amount -= 1
            position -= 1
            if position < 0:
                position += 100
            if position == 0:
                clicks += 1
    elif direction == 'R':
        while amount > 0:
            amount -= 1
            position += 1
            if position > 99:
                position -= 100
            if position == 0:
                clicks += 1
    else:
        print("Error")
        exit(1)
    
    if position == 0:
        zeros += 1

print(f"Final position: {position}, zeros: {zeros}, clicks: {clicks}")