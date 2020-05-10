use std::fs::File;
use std::io::Read;

fn main() {

    let mut file = File::open("invaders.h").expect("File not found");

    let mut buffer = vec![];

    for _ in 0..file.metadata().expect("Failed to read file").len(){
        buffer.push(0);
    }

    file.read_exact(&mut buffer).expect("Failed to read file");

    let mut program_counter = 0usize;

    // for _ in 0..10{
    //     let bytes_read = check_opcode_8080(&program_counter, &buffer);
    //     program_counter = program_counter + bytes_read;
    // }
    
    while program_counter < buffer.len(){
        let bytes_read = check_opcode_8080(&program_counter, &buffer);
        program_counter = program_counter + bytes_read;
    }

}

fn check_opcode_8080(pc: &usize, buffer: &Vec<u8>) -> usize {
    
    let mut read_bytes = 1;

    match buffer[*pc] {
        0 => println!("NOP"),
        1 => {
            println!("LXI B {:02x}{:02x}", buffer[pc + 2], buffer[pc + 1]);
            read_bytes = 3;
        },
        2 => println!("STAX B"),
        3 => println!("INX B"),
        4 => println!("INR B"),
        5 => println!("DCR B"),
        6 => {
            println!("MVI B  {:02x}", buffer[pc + 1]);
            read_bytes = 2;
        },
        7 => println!("RLC"),
        9 => println!("DAD B"),
        10 => println!("LDAX B"),
        11 => println!("DCX B"),
        12 => println!("INR C"),
        13 => println!("DCR C"),
        14 => {
            println!("MVI C {:02x}", buffer[pc + 1]);
            read_bytes = 2;
        },
        15 => println!("RRC"),
        17 => {
            println!("LXI D {:02x}{:02x}", buffer[pc + 2], buffer[pc + 1]);
            read_bytes = 3;
        },
        18 => println!("STAX D"),
        19 => println!("INX D"),
        20 => println!("INR D"),
        21 => println!("DCR D"),
        22 => {
            println!("MVI D  {:02x}", buffer[pc + 1]);
            read_bytes = 2;
        },
        23 => println!("RAL"),
        25 => println!("DAD D"),
        26 => println!("LDAX D"),
        27 => println!("DCX D"),
        28 => println!("INR E"),
        29 => println!("DCR E"),
        30 => {
            println!("MVI E {:02x}", buffer[pc + 1]);
            read_bytes = 2;
        },
        31 => println!("RAR"),
        33 => {
            println!("LXI H {:02x}{:02x}", buffer[pc + 2], buffer[pc + 1]);
            read_bytes = 3;
        },
        34 => println!("SHLD adr"),
        35 => println!("INX H"),
        36 => println!("INR H"),
        37 => println!("DCR H"),
        38 => {
            println!("MVI H {:02x}", buffer[pc + 1]);
            read_bytes = 2;
        },
        39 => println!("DAA"),
        41 => println!("DAD H"),
        42 => println!("LHLD adr"),
        43 => println!("DCX H"),
        44 => println!("INR L"),
        45 => println!("DCR L"),
        46 => {
            println!("MVI L  {:02x}", buffer[pc + 1]);
            read_bytes = 2;
        },
        47 => println!("CMA"),
        49 => {
            println!("LXI SP  {:02x}{:02x}", buffer[pc + 2], buffer[pc + 1]);
            read_bytes = 3;
        },
        50 => println!("STA adr"),
        51 => println!("INX SP"),
        52 => println!("INR M"),
        53 => println!("DCR M"),
        54 => {
            println!("MVI M {:02x}", buffer[pc + 1]);
            read_bytes = 2;
        },
        55 => println!("STC"),
        57 => println!("DAD SP"),
        58 => println!("LDA adr"),
        59 => println!("DCX SP"),
        60 => println!("INR A"),
        61 => println!("DCR A"),
        62 => {
            println!("MVI A {:02x}", buffer[pc + 1]);
            read_bytes = 2;
        },
        63 => println!("CMC"),
        64 => println!("MOV B B"),
        65 => println!("MOV B C"),
        66 => println!("MOV B D"),
        67 => println!("MOV B E"),
        68 => println!("MOV B H"),
        69 => println!("MOV B L"),
        70 => println!("MOV B M"),
        71 => println!("MOV B A"),
        72 => println!("MOV C B"),
        73 => println!("MOV C C"),
        74 => println!("MOV C D"),
        75 => println!("MOV C E"),
        76 => println!("MOV C H"),
        77 => println!("MOV C L"),
        78 => println!("MOV C M"),
        79 => println!("MOV C A"),
        80 => println!("MOV D B"),
        81 => println!("MOV D C"),
        82 => println!("MOV D D"),
        83 => println!("MOV D E"),
        84 => println!("MOV D H"),
        85 => println!("MOV D L"),
        86 => println!("MOV D M"),
        87 => println!("MOV D A"),
        88 => println!("MOV E B"),
        89 => println!("MOV E C"),
        90 => println!("MOV E D"),
        91 => println!("MOV E E"),
        92 => println!("MOV E H"),
        93 => println!("MOV E L"),
        94 => println!("MOV E M"),
        95 => println!("MOV E A"),
        96 => println!("MOV H B"),
        97 => println!("MOV H C"),
        98 => println!("MOV H D"),
        99 => println!("MOV H E"),
        100 => println!("MOV H H"),
        101 => println!("MOV H L"),
        102 => println!("MOV H M"),
        103 => println!("MOV H A"),
        104 => println!("MOV L B"),
        105 => println!("MOV L C"),
        106 => println!("MOV L D"),
        107 => println!("MOV L E"),
        108 => println!("MOV L H"),
        109 => println!("MOV L L"),
        110 => println!("MOV L M"),
        111 => println!("MOV L A"),
        112 => println!("MOV M B"),
        113 => println!("MOV M C"),
        114 => println!("MOV M D"),
        115 => println!("MOV M E"),
        116 => println!("MOV M H"),
        117 => println!("MOV M L"),
        118 => println!("HLT"),
        119 => println!("MOV M A"),
        120 => println!("MOV A B"),
        121 => println!("MOV A C"),
        122 => println!("MOV A D"),
        123 => println!("MOV A E"),
        124 => println!("MOV A H"),
        125 => println!("MOV A L"),
        126 => println!("MOV A M"),
        127 => println!("MOV A A"),
        128 => println!("ADD B"),
        129 => println!("ADD C"),
        130 => println!("ADD D"),
        131 => println!("ADD E"),
        132 => println!("ADD H"),
        133 => println!("ADD L"),
        134 => println!("ADD M"),
        135 => println!("ADD A"),
        136 => println!("ADC B"),
        137 => println!("ADC C"),
        138 => println!("ADC D"),
        139 => println!("ADC E"),
        140 => println!("ADC H"),
        141 => println!("ADC L"),
        142 => println!("ADC M"),
        143 => println!("ADC A"),
        144 => println!("SUB B"),
        145 => println!("SUB C"),
        146 => println!("SUB D"),
        147 => println!("SUB E"),
        148 => println!("SUB H"),
        149 => println!("SUB L"),
        150 => println!("SUB M"),
        151 => println!("SUB A"),
        152 => println!("SBB B"),
        153 => println!("SBB C"),
        154 => println!("SBB D"),
        155 => println!("SBB E"),
        156 => println!("SBB H"),
        157 => println!("SBB L"),
        158 => println!("SBB M"),
        159 => println!("SBB A"),
        160 => println!("ANA B"),
        161 => println!("ANA C"),
        162 => println!("ANA D"),
        163 => println!("ANA E"),
        164 => println!("ANA H"),
        165 => println!("ANA L"),
        166 => println!("ANA M"),
        167 => println!("ANA A"),
        168 => println!("XRA B"),
        169 => println!("XRA C"),
        170 => println!("XRA D"),
        171 => println!("XRA E"),
        172 => println!("XRA H"),
        173 => println!("XRA L"),
        174 => println!("XRA M"),
        175 => println!("XRA A"),
        176 => println!("ORA B"),
        177 => println!("ORA C"),
        178 => println!("ORA D"),
        179 => println!("ORA E"),
        180 => println!("ORA H"),
        181 => println!("ORA L"),
        182 => println!("ORA M"),
        183 => println!("ORA A"),
        184 => println!("CMP B"),
        185 => println!("CMP C"),
        186 => println!("CMP D"),
        187 => println!("CMP E"),
        188 => println!("CMP H"),
        189 => println!("CMP L"),
        190 => println!("CMP M"),
        191 => println!("CMP A"),
        192 => println!("RNZ"),
        193 => println!("POP B"),
        194 => println!("JNZ adr"),
        195 => println!("JMP adr"),
        196 => println!("CNZ adr"),
        197 => println!("PUSH B"),
        198 => {
            println!("ADI {:02x}", buffer[pc + 1]);
            read_bytes = 2;
        },
        199 => println!("RST 0"),
        200 => println!("RZ"),
        201 => println!("RET"),
        202 => println!("JZ adr"),
        204 => println!("CZ adr"),
        205 => println!("CALL adr"),
        206 => {
            println!("ACI {:02x}", buffer[pc + 1]);
            read_bytes = 2;
        },
        207 => {
            println!("RST {:02x}", buffer[pc + 1]);
            read_bytes = 2;
        },
        208 => println!("RNC"),
        209 => println!("POP D"),
        210 => println!("JNC adr"),
        211 => {
            println!("OUT {:02x}", buffer[pc + 1]);
            read_bytes = 2;
        },
        212 => println!("CNC adr"),
        213 => println!("PUSH D"),
        214 => {
            println!("SUI {:02x}", buffer[pc + 1]);
            read_bytes = 2;
        },
        215 => {
            println!("RST {:02x}", buffer[pc + 1]);
            read_bytes = 2;
        },
        216 => println!("RC"),
        218 => println!("JC adr"),
        219 => {
            println!("IN {:02x}", buffer[pc + 1]);
            read_bytes = 2;
        },
        220 => println!("CC adr"),
        222 => {
            println!("SBI {:02x}", buffer[pc + 1]);
            read_bytes = 2;
        },
        223 => {
            println!("RST {:02x}", buffer[pc + 1]);
            read_bytes = 2;
        },
        224 => println!("RPO"),
        225 => println!("POP H"),
        226 => println!("JPO adr"),
        227 => println!("XTHL"),
        228 => println!("CPO adr"),
        229 => println!("PUSH H"),
        230 => {
            println!("ANI {:02x}", buffer[pc + 1]);
            read_bytes = 2;
        },
        231 => {
            println!("RST {:02x}", buffer[pc + 1]);
            read_bytes = 2;
        },
        232 => println!("RPE"),
        233 => println!("PCHL"),
        234 => println!("JPE adr"),
        235 => println!("XCHG"),
        236 => println!("CPE adr"),
        238 => {
            println!("XRI {:02x}", buffer[pc + 1]);
            read_bytes = 2;
        },
        239 => {
            println!("RST {:02x}", buffer[pc + 1]);
            read_bytes = 2;
        },
        240 => println!("RP"),
        241 => println!("POP PSW"),
        242 => println!("JP adr"),
        243 => println!("DI"),
        244 => println!("CP adr"),
        245 => println!("PUSH PSW"),
        246 => {
            println!("ORI {:02x}", buffer[pc + 1]);
            read_bytes = 2;
        },
        247 => {
            println!("RST {:02x}", buffer[pc + 1]);
            read_bytes = 2;
        },
        248 => println!("RM"),
        249 => println!("SPHL"),
        250 => println!("JM adr"),
        251 => println!("EI"),
        252 => println!("CM adr"),
        254 => {
            println!("CPI {:02x}", buffer[pc + 1]);
            read_bytes = 2;
        },
        255 => {
            println!("RST {:02x}", buffer[pc + 1]);
            read_bytes = 2;
        },
		_   => println!("NOP"),
    }

    read_bytes

}

// fn check_args(args: Vec<String>) -> bool {

//     if args.len() == 3{

//         if ["-8080"].contains(&&args[1][..]){

//             return true;

//         }

//     }

//     false

// }