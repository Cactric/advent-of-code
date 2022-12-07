#!/usr/bin/python3

def splitCommands(outputList):
    commandsAndData = []
    for line in outputList:
        if line.startswith('$'):
            # It's a command
            commandsAndData.append([line])
        else:
            # Add it to the end of the previous command
            commandsAndData[-1].append(line)
    return commandsAndData

def parentDir(path):
    toReturn = path[0:path.rfind("/")]
    if(toReturn == ""):
        return "/"
    return toReturn

def addSizeToTree(directories, filePath, fileSize):
    dirPath = filePath
    while dirPath != "/":
        dirPath = parentDir(dirPath)
        #print("adding " + fileSize + " to " + dirPath)
        # try:
        directories[dirPath] += int(fileSize)
        # except:
            # directories[parentDir(dirPath)] = int(fileSize)

def runCommands(commands):
    currentDirectory = "/"
    directories = {"/" : 0}
    for command in commands:
        commandWords = command[0].strip().split(" ")
        #print(command)
        if commandWords[1] == "cd":
            if commandWords[2] == "/":
                currentDirectory = "/"
            elif commandWords[2] == "..":
                currentDirectory = parentDir(currentDirectory)
            else:
                currentDirectory += "/" + commandWords[2]
        elif commandWords[1] == "ls":
            for filePlusNewline in command[1:]:
                file = filePlusNewline.strip().split(" ")
                fileSize = file[0]
                filePath = (currentDirectory + "/" + file[1])[1:]   # Remove the first / to avoid //
                #print(filePath, end=" ")
                if(fileSize == "dir"):
                    #print("[DIR]")
                    directories[filePath] = 0
                else:
                    #print("Size: " + fileSize)
                    addSizeToTree(directories, filePath, fileSize)
            #print(directories)
    return directories


def main():
    fileContents = open("input", 'r')
    outputList = fileContents.readlines()
    fileContents.close()
    
    commands = splitCommands(outputList)
    # Now let's run the commands
    fileTree = runCommands(commands)
    
    count = 0
    for size in fileTree.values():
        if(int(size) <= 100000):
            count += size
    print("Part one: " + str(count))
    
    smallest = 70000000 # Size of the disk
    smallestPath = "/"
    requiredByUpdate = 30000000 - (smallest - fileTree["/"])
    for dirPath in fileTree:
        if int(fileTree[dirPath]) >= requiredByUpdate:
            if int(fileTree[dirPath]) < smallest:
                smallest = int(fileTree[dirPath])
                smallestPath = dirPath
                print("New smallest dir: " + dirPath + " of size " + str(smallest))
    print("Part two: " + str(smallest))

if __name__ == "__main__":
    main()
