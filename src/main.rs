use std::fs::File;
use std::io::Read;
use std::io::{BufWriter, Write};

fn main() {

    let mut file = File::open("invaders/invaders.h").expect("File not found");

    let mut buffer = vec![];

    for _ in 0..file.metadata().expect("Failed to read file").len(){

        buffer.push(0);

    } // Initialize the buffer, inflating it to the size of the file

    file.read_exact(&mut buffer).expect("Failed to read file");

    let mut program_counter = 0;

    let mut file_writer = BufWriter::new(File::create("disassembled.txt").expect("Unable to create file"));
    
    while program_counter < buffer.len(){

        let bytes_read = check_opcode_8080(program_counter, &buffer, &mut file_writer);

        program_counter = program_counter + bytes_read;

    }

}

fn check_opcode_8080(program_counter: usize, buffer: &Vec<u8>, write_buffer: &mut BufWriter<File>) -> usize {
    
    let mut read_bytes = 1;
    
    match buffer[program_counter] {
        0 => write_buffer.write_fmt(format_args!("NOP\n")).expect("Failed to write to file"),
        1 => {
            write_buffer.write_fmt(format_args!("LXI B {:02x} {:02x}\n", buffer[program_counter + 2], buffer[program_counter + 1])).expect("Failed to write to file");
            read_bytes = 3;
        },
        2 => write_buffer.write_fmt(format_args!("STAX B\n")).expect("Failed to write to file"),
        3 => write_buffer.write_fmt(format_args!("INX B\n")).expect("Failed to write to file"),
        4 => write_buffer.write_fmt(format_args!("INR B\n")).expect("Failed to write to file"),
        5 => write_buffer.write_fmt(format_args!("DCR B\n")).expect("Failed to write to file"),
        6 => {
            write_buffer.write_fmt(format_args!("MVI B  {:02x}\n", buffer[program_counter + 1])).expect("Failed to write to file");
            read_bytes = 2;
        },
        7 => write_buffer.write_fmt(format_args!("RLC\n")).expect("Failed to write to file"),
        9 => write_buffer.write_fmt(format_args!("DAD B\n")).expect("Failed to write to file"),
        10 => write_buffer.write_fmt(format_args!("LDAX B\n")).expect("Failed to write to file"),
        11 => write_buffer.write_fmt(format_args!("DCX B\n")).expect("Failed to write to file"),
        12 => write_buffer.write_fmt(format_args!("INR C\n")).expect("Failed to write to file"),
        13 => write_buffer.write_fmt(format_args!("DCR C\n")).expect("Failed to write to file"),
        14 => {
            write_buffer.write_fmt(format_args!("MVI C {:02x}\n", buffer[program_counter + 1])).expect("Failed to write to file");
            read_bytes = 2;
        },
        15 => write_buffer.write_fmt(format_args!("RRC\n")).expect("Failed to write to file"),
        17 => {
            write_buffer.write_fmt(format_args!("LXI D {:02x} {:02x}\n", buffer[program_counter + 2], buffer[program_counter + 1])).expect("Failed to write to file");
            read_bytes = 3;
        },
        18 => write_buffer.write_fmt(format_args!("STAX D\n")).expect("Failed to write to file"),
        19 => write_buffer.write_fmt(format_args!("INX D\n")).expect("Failed to write to file"),
        20 => write_buffer.write_fmt(format_args!("INR D\n")).expect("Failed to write to file"),
        21 => write_buffer.write_fmt(format_args!("DCR D\n")).expect("Failed to write to file"),
        22 => {
            write_buffer.write_fmt(format_args!("MVI D  {:02x}\n", buffer[program_counter + 1])).expect("Failed to write to file");
            read_bytes = 2;
        },
        23 => write_buffer.write_fmt(format_args!("RAL\n")).expect("Failed to write to file"),
        25 => write_buffer.write_fmt(format_args!("DAD D\n")).expect("Failed to write to file"),
        26 => write_buffer.write_fmt(format_args!("LDAX D\n")).expect("Failed to write to file"),
        27 => write_buffer.write_fmt(format_args!("DCX D\n")).expect("Failed to write to file"),
        28 => write_buffer.write_fmt(format_args!("INR E\n")).expect("Failed to write to file"),
        29 => write_buffer.write_fmt(format_args!("DCR E\n")).expect("Failed to write to file"),
        30 => {
            write_buffer.write_fmt(format_args!("MVI E {:02x}\n", buffer[program_counter + 1])).expect("Failed to write to file");
            read_bytes = 2;
        },
        31 => write_buffer.write_fmt(format_args!("RAR\n")).expect("Failed to write to file"),
        33 => {
            write_buffer.write_fmt(format_args!("LXI H {:02x} {:02x}\n", buffer[program_counter + 2], buffer[program_counter + 1])).expect("Failed to write to file");
            read_bytes = 3;
        },
        34 => {
            write_buffer.write_fmt(format_args!("SHLD {:02x} {:02x}\n", buffer[program_counter + 1], buffer[program_counter + 2])).expect("Failed to write to file");
            read_bytes = 3;
        },
        35 => write_buffer.write_fmt(format_args!("INX H\n")).expect("Failed to write to file"),
        36 => write_buffer.write_fmt(format_args!("INR H\n")).expect("Failed to write to file"),
        37 => write_buffer.write_fmt(format_args!("DCR H\n")).expect("Failed to write to file"),
        38 => {
            write_buffer.write_fmt(format_args!("MVI H {:02x}\n", buffer[program_counter + 1])).expect("Failed to write to file");
            read_bytes = 2;
        },
        39 => write_buffer.write_fmt(format_args!("DAA\n")).expect("Failed to write to file"),
        41 => write_buffer.write_fmt(format_args!("DAD H\n")).expect("Failed to write to file"),
        42 => {
            write_buffer.write_fmt(format_args!("LHLD {:02x} {:02x}\n", buffer[program_counter + 1], buffer[program_counter + 2])).expect("Failed to write to file");
            read_bytes = 3;
        },
        43 => write_buffer.write_fmt(format_args!("DCX H\n")).expect("Failed to write to file"),
        44 => write_buffer.write_fmt(format_args!("INR L\n")).expect("Failed to write to file"),
        45 => write_buffer.write_fmt(format_args!("DCR L\n")).expect("Failed to write to file"),
        46 => {
            write_buffer.write_fmt(format_args!("MVI L  {:02x}\n", buffer[program_counter + 1])).expect("Failed to write to file");
            read_bytes = 2;
        },
        47 => write_buffer.write_fmt(format_args!("CMA\n")).expect("Failed to write to file"),
        49 => {
            write_buffer.write_fmt(format_args!("LXI SP  {:02x} {:02x}\n", buffer[program_counter + 2], buffer[program_counter + 1])).expect("Failed to write to file");
            read_bytes = 3;
        },
        50 => {
            write_buffer.write_fmt(format_args!("STA {:02x} {:02x}\n", buffer[program_counter + 1], buffer[program_counter + 2])).expect("Failed to write to file");
            read_bytes = 3;
        },
        51 => write_buffer.write_fmt(format_args!("INX SP\n")).expect("Failed to write to file"),
        52 => write_buffer.write_fmt(format_args!("INR M\n")).expect("Failed to write to file"),
        53 => write_buffer.write_fmt(format_args!("DCR M\n")).expect("Failed to write to file"),
        54 => {
            write_buffer.write_fmt(format_args!("MVI M {:02x}\n", buffer[program_counter + 1])).expect("Failed to write to file");
            read_bytes = 2;
        },
        55 => write_buffer.write_fmt(format_args!("STC\n")).expect("Failed to write to file"),
        57 => write_buffer.write_fmt(format_args!("DAD SP\n")).expect("Failed to write to file"),
        58 => {
            write_buffer.write_fmt(format_args!("LDA {:02x} {:02x}\n", buffer[program_counter + 1], buffer[program_counter + 2])).expect("Failed to write to file");
            read_bytes = 3;
        },
        59 => write_buffer.write_fmt(format_args!("DCX SP\n")).expect("Failed to write to file"),
        60 => write_buffer.write_fmt(format_args!("INR A\n")).expect("Failed to write to file"),
        61 => write_buffer.write_fmt(format_args!("DCR A\n")).expect("Failed to write to file"),
        62 => {
            write_buffer.write_fmt(format_args!("MVI A {:02x}\n", buffer[program_counter + 1])).expect("Failed to write to file");
            read_bytes = 2;
        },
        63 => write_buffer.write_fmt(format_args!("CMC\n")).expect("Failed to write to file"),
        64 => write_buffer.write_fmt(format_args!("MOV B B\n")).expect("Failed to write to file"),
        65 => write_buffer.write_fmt(format_args!("MOV B C\n")).expect("Failed to write to file"),
        66 => write_buffer.write_fmt(format_args!("MOV B D\n")).expect("Failed to write to file"),
        67 => write_buffer.write_fmt(format_args!("MOV B E\n")).expect("Failed to write to file"),
        68 => write_buffer.write_fmt(format_args!("MOV B H\n")).expect("Failed to write to file"),
        69 => write_buffer.write_fmt(format_args!("MOV B L\n")).expect("Failed to write to file"),
        70 => write_buffer.write_fmt(format_args!("MOV B M\n")).expect("Failed to write to file"),
        71 => write_buffer.write_fmt(format_args!("MOV B A\n")).expect("Failed to write to file"),
        72 => write_buffer.write_fmt(format_args!("MOV C B\n")).expect("Failed to write to file"),
        73 => write_buffer.write_fmt(format_args!("MOV C C\n")).expect("Failed to write to file"),
        74 => write_buffer.write_fmt(format_args!("MOV C D\n")).expect("Failed to write to file"),
        75 => write_buffer.write_fmt(format_args!("MOV C E\n")).expect("Failed to write to file"),
        76 => write_buffer.write_fmt(format_args!("MOV C H\n")).expect("Failed to write to file"),
        77 => write_buffer.write_fmt(format_args!("MOV C L\n")).expect("Failed to write to file"),
        78 => write_buffer.write_fmt(format_args!("MOV C M\n")).expect("Failed to write to file"),
        79 => write_buffer.write_fmt(format_args!("MOV C A\n")).expect("Failed to write to file"),
        80 => write_buffer.write_fmt(format_args!("MOV D B\n")).expect("Failed to write to file"),
        81 => write_buffer.write_fmt(format_args!("MOV D C\n")).expect("Failed to write to file"),
        82 => write_buffer.write_fmt(format_args!("MOV D D\n")).expect("Failed to write to file"),
        83 => write_buffer.write_fmt(format_args!("MOV D E\n")).expect("Failed to write to file"),
        84 => write_buffer.write_fmt(format_args!("MOV D H\n")).expect("Failed to write to file"),
        85 => write_buffer.write_fmt(format_args!("MOV D L\n")).expect("Failed to write to file"),
        86 => write_buffer.write_fmt(format_args!("MOV D M\n")).expect("Failed to write to file"),
        87 => write_buffer.write_fmt(format_args!("MOV D A\n")).expect("Failed to write to file"),
        88 => write_buffer.write_fmt(format_args!("MOV E B\n")).expect("Failed to write to file"),
        89 => write_buffer.write_fmt(format_args!("MOV E C\n")).expect("Failed to write to file"),
        90 => write_buffer.write_fmt(format_args!("MOV E D\n")).expect("Failed to write to file"),
        91 => write_buffer.write_fmt(format_args!("MOV E E\n")).expect("Failed to write to file"),
        92 => write_buffer.write_fmt(format_args!("MOV E H\n")).expect("Failed to write to file"),
        93 => write_buffer.write_fmt(format_args!("MOV E L\n")).expect("Failed to write to file"),
        94 => write_buffer.write_fmt(format_args!("MOV E M\n")).expect("Failed to write to file"),
        95 => write_buffer.write_fmt(format_args!("MOV E A\n")).expect("Failed to write to file"),
        96 => write_buffer.write_fmt(format_args!("MOV H B\n")).expect("Failed to write to file"),
        97 => write_buffer.write_fmt(format_args!("MOV H C\n")).expect("Failed to write to file"),
        98 => write_buffer.write_fmt(format_args!("MOV H D\n")).expect("Failed to write to file"),
        99 => write_buffer.write_fmt(format_args!("MOV H E\n")).expect("Failed to write to file"),
        100 => write_buffer.write_fmt(format_args!("MOV H H\n")).expect("Failed to write to file"),
        101 => write_buffer.write_fmt(format_args!("MOV H L\n")).expect("Failed to write to file"),
        102 => write_buffer.write_fmt(format_args!("MOV H M\n")).expect("Failed to write to file"),
        103 => write_buffer.write_fmt(format_args!("MOV H A\n")).expect("Failed to write to file"),
        104 => write_buffer.write_fmt(format_args!("MOV L B\n")).expect("Failed to write to file"),
        105 => write_buffer.write_fmt(format_args!("MOV L C\n")).expect("Failed to write to file"),
        106 => write_buffer.write_fmt(format_args!("MOV L D\n")).expect("Failed to write to file"),
        107 => write_buffer.write_fmt(format_args!("MOV L E\n")).expect("Failed to write to file"),
        108 => write_buffer.write_fmt(format_args!("MOV L H\n")).expect("Failed to write to file"),
        109 => write_buffer.write_fmt(format_args!("MOV L L\n")).expect("Failed to write to file"),
        110 => write_buffer.write_fmt(format_args!("MOV L M\n")).expect("Failed to write to file"),
        111 => write_buffer.write_fmt(format_args!("MOV L A\n")).expect("Failed to write to file"),
        112 => write_buffer.write_fmt(format_args!("MOV M B\n")).expect("Failed to write to file"),
        113 => write_buffer.write_fmt(format_args!("MOV M C\n")).expect("Failed to write to file"),
        114 => write_buffer.write_fmt(format_args!("MOV M D\n")).expect("Failed to write to file"),
        115 => write_buffer.write_fmt(format_args!("MOV M E\n")).expect("Failed to write to file"),
        116 => write_buffer.write_fmt(format_args!("MOV M H\n")).expect("Failed to write to file"),
        117 => write_buffer.write_fmt(format_args!("MOV M L\n")).expect("Failed to write to file"),
        118 => write_buffer.write_fmt(format_args!("HLT\n")).expect("Failed to write to file"),
        119 => write_buffer.write_fmt(format_args!("MOV M A\n")).expect("Failed to write to file"),
        120 => write_buffer.write_fmt(format_args!("MOV A B\n")).expect("Failed to write to file"),
        121 => write_buffer.write_fmt(format_args!("MOV A C\n")).expect("Failed to write to file"),
        122 => write_buffer.write_fmt(format_args!("MOV A D\n")).expect("Failed to write to file"),
        123 => write_buffer.write_fmt(format_args!("MOV A E\n")).expect("Failed to write to file"),
        124 => write_buffer.write_fmt(format_args!("MOV A H\n")).expect("Failed to write to file"),
        125 => write_buffer.write_fmt(format_args!("MOV A L\n")).expect("Failed to write to file"),
        126 => write_buffer.write_fmt(format_args!("MOV A M\n")).expect("Failed to write to file"),
        127 => write_buffer.write_fmt(format_args!("MOV A A\n")).expect("Failed to write to file"),
        128 => write_buffer.write_fmt(format_args!("ADD B\n")).expect("Failed to write to file"),
        129 => write_buffer.write_fmt(format_args!("ADD C\n")).expect("Failed to write to file"),
        130 => write_buffer.write_fmt(format_args!("ADD D\n")).expect("Failed to write to file"),
        131 => write_buffer.write_fmt(format_args!("ADD E\n")).expect("Failed to write to file"),
        132 => write_buffer.write_fmt(format_args!("ADD H\n")).expect("Failed to write to file"),
        133 => write_buffer.write_fmt(format_args!("ADD L\n")).expect("Failed to write to file"),
        134 => write_buffer.write_fmt(format_args!("ADD M\n")).expect("Failed to write to file"),
        135 => write_buffer.write_fmt(format_args!("ADD A\n")).expect("Failed to write to file"),
        136 => write_buffer.write_fmt(format_args!("ADC B\n")).expect("Failed to write to file"),
        137 => write_buffer.write_fmt(format_args!("ADC C\n")).expect("Failed to write to file"),
        138 => write_buffer.write_fmt(format_args!("ADC D\n")).expect("Failed to write to file"),
        139 => write_buffer.write_fmt(format_args!("ADC E\n")).expect("Failed to write to file"),
        140 => write_buffer.write_fmt(format_args!("ADC H\n")).expect("Failed to write to file"),
        141 => write_buffer.write_fmt(format_args!("ADC L\n")).expect("Failed to write to file"),
        142 => write_buffer.write_fmt(format_args!("ADC M\n")).expect("Failed to write to file"),
        143 => write_buffer.write_fmt(format_args!("ADC A\n")).expect("Failed to write to file"),
        144 => write_buffer.write_fmt(format_args!("SUB B\n")).expect("Failed to write to file"),
        145 => write_buffer.write_fmt(format_args!("SUB C\n")).expect("Failed to write to file"),
        146 => write_buffer.write_fmt(format_args!("SUB D\n")).expect("Failed to write to file"),
        147 => write_buffer.write_fmt(format_args!("SUB E\n")).expect("Failed to write to file"),
        148 => write_buffer.write_fmt(format_args!("SUB H\n")).expect("Failed to write to file"),
        149 => write_buffer.write_fmt(format_args!("SUB L\n")).expect("Failed to write to file"),
        150 => write_buffer.write_fmt(format_args!("SUB M\n")).expect("Failed to write to file"),
        151 => write_buffer.write_fmt(format_args!("SUB A\n")).expect("Failed to write to file"),
        152 => write_buffer.write_fmt(format_args!("SBB B\n")).expect("Failed to write to file"),
        153 => write_buffer.write_fmt(format_args!("SBB C\n")).expect("Failed to write to file"),
        154 => write_buffer.write_fmt(format_args!("SBB D\n")).expect("Failed to write to file"),
        155 => write_buffer.write_fmt(format_args!("SBB E\n")).expect("Failed to write to file"),
        156 => write_buffer.write_fmt(format_args!("SBB H\n")).expect("Failed to write to file"),
        157 => write_buffer.write_fmt(format_args!("SBB L\n")).expect("Failed to write to file"),
        158 => write_buffer.write_fmt(format_args!("SBB M\n")).expect("Failed to write to file"),
        159 => write_buffer.write_fmt(format_args!("SBB A\n")).expect("Failed to write to file"),
        160 => write_buffer.write_fmt(format_args!("ANA B\n")).expect("Failed to write to file"),
        161 => write_buffer.write_fmt(format_args!("ANA C\n")).expect("Failed to write to file"),
        162 => write_buffer.write_fmt(format_args!("ANA D\n")).expect("Failed to write to file"),
        163 => write_buffer.write_fmt(format_args!("ANA E\n")).expect("Failed to write to file"),
        164 => write_buffer.write_fmt(format_args!("ANA H\n")).expect("Failed to write to file"),
        165 => write_buffer.write_fmt(format_args!("ANA L\n")).expect("Failed to write to file"),
        166 => write_buffer.write_fmt(format_args!("ANA M\n")).expect("Failed to write to file"),
        167 => write_buffer.write_fmt(format_args!("ANA A\n")).expect("Failed to write to file"),
        168 => write_buffer.write_fmt(format_args!("XRA B\n")).expect("Failed to write to file"),
        169 => write_buffer.write_fmt(format_args!("XRA C\n")).expect("Failed to write to file"),
        170 => write_buffer.write_fmt(format_args!("XRA D\n")).expect("Failed to write to file"),
        171 => write_buffer.write_fmt(format_args!("XRA E\n")).expect("Failed to write to file"),
        172 => write_buffer.write_fmt(format_args!("XRA H\n")).expect("Failed to write to file"),
        173 => write_buffer.write_fmt(format_args!("XRA L\n")).expect("Failed to write to file"),
        174 => write_buffer.write_fmt(format_args!("XRA M\n")).expect("Failed to write to file"),
        175 => write_buffer.write_fmt(format_args!("XRA A\n")).expect("Failed to write to file"),
        176 => write_buffer.write_fmt(format_args!("ORA B\n")).expect("Failed to write to file"),
        177 => write_buffer.write_fmt(format_args!("ORA C\n")).expect("Failed to write to file"),
        178 => write_buffer.write_fmt(format_args!("ORA D\n")).expect("Failed to write to file"),
        179 => write_buffer.write_fmt(format_args!("ORA E\n")).expect("Failed to write to file"),
        180 => write_buffer.write_fmt(format_args!("ORA H\n")).expect("Failed to write to file"),
        181 => write_buffer.write_fmt(format_args!("ORA L\n")).expect("Failed to write to file"),
        182 => write_buffer.write_fmt(format_args!("ORA M\n")).expect("Failed to write to file"),
        183 => write_buffer.write_fmt(format_args!("ORA A\n")).expect("Failed to write to file"),
        184 => write_buffer.write_fmt(format_args!("CMP B\n")).expect("Failed to write to file"),
        185 => write_buffer.write_fmt(format_args!("CMP C\n")).expect("Failed to write to file"),
        186 => write_buffer.write_fmt(format_args!("CMP D\n")).expect("Failed to write to file"),
        187 => write_buffer.write_fmt(format_args!("CMP E\n")).expect("Failed to write to file"),
        188 => write_buffer.write_fmt(format_args!("CMP H\n")).expect("Failed to write to file"),
        189 => write_buffer.write_fmt(format_args!("CMP L\n")).expect("Failed to write to file"),
        190 => write_buffer.write_fmt(format_args!("CMP M\n")).expect("Failed to write to file"),
        191 => write_buffer.write_fmt(format_args!("CMP A\n")).expect("Failed to write to file"),
        192 => write_buffer.write_fmt(format_args!("RNZ\n")).expect("Failed to write to file"),
        193 => write_buffer.write_fmt(format_args!("POP B\n")).expect("Failed to write to file"),
        194 => {
            write_buffer.write_fmt(format_args!("JNZ {:02x} {:02x}\n", buffer[program_counter + 1], buffer[program_counter + 2])).expect("Failed to write to file");
            read_bytes = 3;
        },
        195 => {
            write_buffer.write_fmt(format_args!("JMP {:02x} {:02x}\n", buffer[program_counter + 1], buffer[program_counter + 2])).expect("Failed to write to file");
            read_bytes = 3;
        },
        196 => {
            write_buffer.write_fmt(format_args!("CNZ {:02x} {:02x}\n", buffer[program_counter + 1], buffer[program_counter + 2])).expect("Failed to write to file");
            read_bytes = 3;
        },
        197 => write_buffer.write_fmt(format_args!("PUSH B\n")).expect("Failed to write to file"),
        198 => {
            write_buffer.write_fmt(format_args!("ADI {:02x}\n", buffer[program_counter + 1])).expect("Failed to write to file");
            read_bytes = 2;
        },
        199 => write_buffer.write_fmt(format_args!("RST 0\n")).expect("Failed to write to file"),
        200 => write_buffer.write_fmt(format_args!("RZ\n")).expect("Failed to write to file"),
        201 => write_buffer.write_fmt(format_args!("RET\n")).expect("Failed to write to file"),
        202 => {
            write_buffer.write_fmt(format_args!("JZ {:02x} {:02x}\n", buffer[program_counter + 1], buffer[program_counter + 2])).expect("Failed to write to file");
            read_bytes = 3;
        },
        204 => {
            write_buffer.write_fmt(format_args!("CZ {:02x} {:02x}\n", buffer[program_counter + 1], buffer[program_counter + 2])).expect("Failed to write to file");
            read_bytes = 3;
        },
        205 => {
            write_buffer.write_fmt(format_args!("CALL {:02x} {:02x}\n", buffer[program_counter + 1], buffer[program_counter + 2])).expect("Failed to write to file");
            read_bytes = 3;
        },
        206 => {
            write_buffer.write_fmt(format_args!("ACI {:02x}\n", buffer[program_counter + 1])).expect("Failed to write to file");
            read_bytes = 2;
        },
        207 => write_buffer.write_fmt(format_args!("RST 1\n")).expect("Failed to write to file"),
        208 => write_buffer.write_fmt(format_args!("RNC\n")).expect("Failed to write to file"),
        209 => write_buffer.write_fmt(format_args!("POP D\n")).expect("Failed to write to file"),
        210 => {
            write_buffer.write_fmt(format_args!("JNC {:02x} {:02x}\n", buffer[program_counter + 1], buffer[program_counter + 2])).expect("Failed to write to file");
            read_bytes = 3;
        },
        211 => {
            write_buffer.write_fmt(format_args!("OUT {:02x}\n", buffer[program_counter + 1])).expect("Failed to write to file");
            read_bytes = 2;
        },
        212 => {
            write_buffer.write_fmt(format_args!("CNC {:02x} {:02x}\n", buffer[program_counter + 1], buffer[program_counter + 2])).expect("Failed to write to file");
            read_bytes = 3;
        },
        213 => write_buffer.write_fmt(format_args!("PUSH D\n")).expect("Failed to write to file"),
        214 => {
            write_buffer.write_fmt(format_args!("SUI {:02x}\n", buffer[program_counter + 1])).expect("Failed to write to file");
            read_bytes = 2;
        },
        215 => write_buffer.write_fmt(format_args!("RST 2\n")).expect("Failed to write to file"),
        216 => write_buffer.write_fmt(format_args!("RC\n")).expect("Failed to write to file"),
        218 => {
            write_buffer.write_fmt(format_args!("JC {:02x} {:02x}\n", buffer[program_counter + 1], buffer[program_counter + 2])).expect("Failed to write to file");
            read_bytes = 3;
        },
        219 => {
            write_buffer.write_fmt(format_args!("IN {:02x}\n", buffer[program_counter + 1])).expect("Failed to write to file");
            read_bytes = 2;
        },
        220 => {
            write_buffer.write_fmt(format_args!("CC {:02x} {:02x}\n", buffer[program_counter + 1], buffer[program_counter + 2])).expect("Failed to write to file");
            read_bytes = 3;
        },
        222 => {
            write_buffer.write_fmt(format_args!("SBI {:02x}\n", buffer[program_counter + 1])).expect("Failed to write to file");
            read_bytes = 2;
        },
        223 => write_buffer.write_fmt(format_args!("RST 3\n")).expect("Failed to write to file"),
        224 => write_buffer.write_fmt(format_args!("RPO\n")).expect("Failed to write to file"),
        225 => write_buffer.write_fmt(format_args!("POP H\n")).expect("Failed to write to file"),
        226 => {
            write_buffer.write_fmt(format_args!("JPO {:02x} {:02x}\n", buffer[program_counter + 1], buffer[program_counter + 2])).expect("Failed to write to file");
            read_bytes = 3;
        },
        227 => write_buffer.write_fmt(format_args!("XTHL\n")).expect("Failed to write to file"),
        228 => {
            write_buffer.write_fmt(format_args!("CPO {:02x} {:02x}\n", buffer[program_counter + 1], buffer[program_counter + 2])).expect("Failed to write to file");
            read_bytes = 3;
        },
        229 => write_buffer.write_fmt(format_args!("PUSH H\n")).expect("Failed to write to file"),
        230 => {
            write_buffer.write_fmt(format_args!("ANI {:02x}\n", buffer[program_counter + 1])).expect("Failed to write to file");
            read_bytes = 2;
        },
        231 => write_buffer.write_fmt(format_args!("RST 4\n")).expect("Failed to write to file"),
        232 => write_buffer.write_fmt(format_args!("RPE\n")).expect("Failed to write to file"),
        233 => write_buffer.write_fmt(format_args!("PCHL\n")).expect("Failed to write to file"),
        234 => {
            write_buffer.write_fmt(format_args!("JPE {:02x} {:02x}\n", buffer[program_counter + 1], buffer[program_counter + 2])).expect("Failed to write to file");
            read_bytes = 3;
        },
        235 => write_buffer.write_fmt(format_args!("XCHG\n")).expect("Failed to write to file"),
        236 => {
            write_buffer.write_fmt(format_args!("CPE {:02x} {:02x}\n", buffer[program_counter + 1], buffer[program_counter + 2])).expect("Failed to write to file");
            read_bytes = 3;
        },
        238 => {
            write_buffer.write_fmt(format_args!("XRI {:02x}\n", buffer[program_counter + 1])).expect("Failed to write to file");
            read_bytes = 2;
        },
        239 => write_buffer.write_fmt(format_args!("RST 5\n")).expect("Failed to write to file"),
        240 => write_buffer.write_fmt(format_args!("RP\n")).expect("Failed to write to file"),
        241 => write_buffer.write_fmt(format_args!("POP PSW\n")).expect("Failed to write to file"),
        242 => {
            write_buffer.write_fmt(format_args!("JP {:02x} {:02x}\n", buffer[program_counter + 1], buffer[program_counter + 2])).expect("Failed to write to file");
            read_bytes = 3;
        },
        243 => write_buffer.write_fmt(format_args!("DI\n")).expect("Failed to write to file"),
        244 => {
            write_buffer.write_fmt(format_args!("CP {:02x} {:02x}\n", buffer[program_counter + 1], buffer[program_counter + 2])).expect("Failed to write to file");
            read_bytes = 3;
        },
        245 => write_buffer.write_fmt(format_args!("PUSH PSW\n")).expect("Failed to write to file"),
        246 => {
            write_buffer.write_fmt(format_args!("ORI {:02x}\n", buffer[program_counter + 1])).expect("Failed to write to file");
            read_bytes = 2;
        },
        247 => write_buffer.write_fmt(format_args!("RST 6\n")).expect("Failed to write to file"),
        248 => write_buffer.write_fmt(format_args!("RM\n")).expect("Failed to write to file"),
        249 => write_buffer.write_fmt(format_args!("SPHL\n")).expect("Failed to write to file"),
        250 => {
            write_buffer.write_fmt(format_args!("JM {:02x} {:02x}\n", buffer[program_counter + 1], buffer[program_counter + 2])).expect("Failed to write to file");
            read_bytes = 3;
        },
        251 => write_buffer.write_fmt(format_args!("EI\n")).expect("Failed to write to file"),
        252 => {
            write_buffer.write_fmt(format_args!("CM {:02x} {:02x}\n", buffer[program_counter + 1], buffer[program_counter + 2])).expect("Failed to write to file");
            read_bytes = 3;
        },
        254 => {
            write_buffer.write_fmt(format_args!("CPI {:02x}\n", buffer[program_counter + 1])).expect("Failed to write to file");
            read_bytes = 2;
        },
        255 => write_buffer.write_fmt(format_args!("RST 7\n")).expect("Failed to write to file"),        
        
		_   => println!("NOP"),
    }

    read_bytes

}