with open("opcodes.txt", "r") as file:

    data = file.readlines()

output = ""

for lineIndex in range(len(data)):

    data[lineIndex] = data[lineIndex].replace("\n", "").replace("<-", "").replace(",", " ").replace("byte ", "byte").split("\t")
    for portion in data[lineIndex]:
        portion = ' '.join(portion.split())

for line in data:

    line[0] = int(line[0], 16)

for i in range(len(data)):

    lastRegister = data[i][1].split(" ")[-1]

    if lastRegister.isalpha() or lastRegister == "0":

        output += f"{data[i][0]} => println!(\"{data[i][1]}\"),\n"
    
    else:

        if len(data[i]) == 2:

            numberOfBytes = 8
        
        else:

            numberOfBytes = data[i][2].count("byte") * 8

        if numberOfBytes == 8:

            println = data[i][1].replace(lastRegister, "{:02x}") + "\", buffer[pc + 1]"
        
        else:

            println = data[i][1].replace(lastRegister, "{:02x}{:02x}")

            opcodeLocs = data[i][2].split()

            if opcodeLocs.index("byte2") < opcodeLocs.index("byte3"): # If byte2 comes before byte3

                println += "\", buffer[pc + 1], buffer[pc + 2]"
            
            else:

                println += "\", buffer[pc + 2], buffer[pc + 1]"

        output += f"{data[i][0]} => {{\n\tprintln!(\"{println});\n\tread_bytes = {int(numberOfBytes / 8 + 1)};\n}},\n"

with open("code.txt", "w") as file:

    file.write(output)