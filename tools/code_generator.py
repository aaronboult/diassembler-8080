# Retrieved opcodes.txt from http://www.emulator101.com/reference/8080-by-opcode.html

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

    if data[i][2] == "1":

        output += f"{data[i][0]} => write_buffer.write_fmt(format_args!(\"0x{{:04x}}: 0x{{:02x}} {data[i][1]}\\n\", program_counter, buffer[program_counter])).expect(\"Failed to write to file\"),\n"
    
    else:

        lastRegister = data[i][1].split(" ")[-1]

        if data[i][2] == "2":

            println = "0x{:04x}: 0x{:02x} " + data[i][1].replace(lastRegister, "{:02x}") + "\\n\", program_counter, buffer[program_counter], buffer[program_counter + 1]"
        
        else:

            println = "0x{:04x}: 0x{:02x} " + data[i][1].replace(lastRegister, "{:02x}{:02x}") + "\\n\", program_counter, buffer[program_counter], buffer[program_counter + 1], buffer[program_counter + 2]"

        output += f"{data[i][0]} => {{\n\twrite_buffer.write_fmt(format_args!(\"{println})).expect(\"Failed to write to file\");\n\tread_bytes = {int(data[i][2])};\n}},\n"

with open("code.txt", "w") as file:

    file.write(output)