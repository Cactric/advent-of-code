#!/usr/bin/python3

def strListToIntList(strList):
    intList = list()
    for item in map(int, strList):
        intList.append(item)
    return intList

def parseMonkey(monkey):
    cheekyData = dict()
    monkeyLines = monkey.split("\n")
    cheekyData["Name"] = monkeyLines[0].split(":")[0]
    # e.g. “Monkey 0”
    cheekyData["Items"] = strListToIntList(monkeyLines[1].split(": ")[1].split(", "))
    # e.g. [79, 98]
    cheekyData["Operation"] = monkeyLines[2].split(": ")[1]
    # e.g. "new = old + 6"
    cheekyData["Test"] = monkeyLines[3].split(": ")[1]
    # e.g. "divisible by 23"
    cheekyData["IfTrueTo"] = int(monkeyLines[4].split("monkey ")[1])
    cheekyData["IfFalseTo"] = int(monkeyLines[5].split("monkey ")[1])
    cheekyData["InspectionCount"] = 0
    return cheekyData

def getMonkeyData(contents):
    monkeys = contents.split("\n\n")
    monkeyData = list()
    for monkeyDict in map(parseMonkey, monkeys):
        monkeyData.append(monkeyDict)
    return monkeyData

def doOp(operation, item):
    symbols = operation.split(" ")
    # symbols[0] should be 'new', symbols[1] should be '='
    # symbols[2] should be 'old'
    if symbols[3] == '*':
        if symbols[4] == 'old':
            return item*item
        else:
            return item*int(symbols[4])
    elif symbols[3] == '+':
        if symbols[4] == 'old':
            return item+item
        else:
            return item+int(symbols[4])
    else:
        print("Uh oh!! Unknown symbol: " + symbols[3])
        exit()

def playRound(monkeyData):
    for monkey in monkeyData:
        #print(monkeyData)
        for x in range(len(monkey["Items"])):
            result = doOp(monkey["Operation"], monkey["Items"][0])
            result = int(result / 3)
            #monkey["Items"][x] = result
            divisor = int(monkey["Test"].split("by ")[1])
            if(result % divisor == 0):
                monkeyData[monkey["IfTrueTo"]]["Items"].append(result)
            else:
                monkeyData[monkey["IfFalseTo"]]["Items"].append(result)
            monkey["InspectionCount"] += 1
            monkey["Items"].pop(0)
    return monkeyData

def panicRound(monkeyData):
    modulo = 1
    for monkey in monkeyData:
        modulo *= int(monkey["Test"].split("by ")[1])
    for monkey in monkeyData:
        #print(monkeyData)
        for x in range(len(monkey["Items"])):
            result = doOp(monkey["Operation"], monkey["Items"][0])
            result %= modulo
            #monkey["Items"][x] = result
            divisor = int(monkey["Test"].split("by ")[1])
            if(result % divisor == 0):
                monkeyData[monkey["IfTrueTo"]]["Items"].append(result)
            else:
                monkeyData[monkey["IfFalseTo"]]["Items"].append(result)
            monkey["InspectionCount"] += 1
            monkey["Items"].pop(0)
    return monkeyData

def main():
    with open("input") as f:
        contents = f.read()
    monkeyData = getMonkeyData(contents)
    for x in range(20):
        monkeyData = playRound(monkeyData)
    
    for monkey in monkeyData:
        print("{} inspected items {} times".format(monkey["Name"], monkey["InspectionCount"]))
    print("Part one is the inspection count of the two most active monkeys multiplied together.\nDo it manually, I'm lazy.")
    print("-"*40)
    
    del monkeyData
    monkeyData = getMonkeyData(contents)
    for x in range(10000):
        monkeyData = panicRound(monkeyData)
        print(f"Round {x}/10000", end="\r", flush=True)

    for monkey in monkeyData:
        print("{} inspected items {} times".format(monkey["Name"], monkey["InspectionCount"]))
    print("Part two is the inspection count of the two most active monkeys multiplied together.\nDo it manually, I'm lazy.")
    

if __name__ == "__main__":
    main()
