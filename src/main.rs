use std::fs::File;
use std::io::Read;
use std::io::{BufWriter, Write};

fn main() {

    // Add option to provide file name by either specifying a directory or a file via command line

    let mut buffer = vec![];

    // load_file("SpaceInvaders/SpaceInvaders.h".to_string(), &mut buffer, 0x800, 0x0);
    // load_file("SpaceInvaders/SpaceInvaders.g".to_string(), &mut buffer, 0x800, 0x800);
    // load_file("SpaceInvaders/SpaceInvaders.f".to_string(), &mut buffer, 0x800, 0x1000);
    // load_file("SpaceInvaders/SpaceInvaders.e".to_string(), &mut buffer, 0x800, 0x1800);

    load_file("cpudiag.bin".to_string(), &mut buffer, 1453, 0x100);

    let mut program_counter = 0;

    let mut file_writer = BufWriter::new(File::create("disassembled.txt").expect("Unable to create file"));

    file_writer.write(b"Format:\n\tProgram Counter: Opcode (in hex) Opcode (in assembly) operands...\n\n").expect("Unable to write to file");
    
    while program_counter < buffer.len(){

        let bytes_read = check_opcode_8080(program_counter, &buffer, &mut file_writer);

        program_counter = program_counter + bytes_read;

    }

}

fn load_file(file_name: String, buffer: &mut Vec<u8>, file_size: usize, offset: usize){

    while buffer.len() < offset{
        buffer.push(0);
    }

    let mut temp_buffer = vec![0u8; file_size];

    let mut file = File::open(file_name).expect("File not found");

    file.read_exact(&mut temp_buffer).expect("Failed to read file");

    buffer.append(&mut temp_buffer);

}

pub fn check_opcode_8080(program_counter: usize, buffer: &Vec<u8>, write_buffer: &mut BufWriter<File>) -> usize {
    
    let mut read_bytes = 1;
    
    match buffer[program_counter] {
        0 => write_buffer.write_fmt(format_args!("0x{:04x}: 0x{:02x} NOP\n", program_counter, buffer[program_counter])).expect("Failed to write to file"),
        1 => {
            write_buffer.write_fmt(format_args!("0x{:04x}: 0x{:02x} LXI B {:02x}{:02x}\n", program_counter, buffer[program_counter], buffer[program_counter + 1], buffer[program_counter + 2])).expect("Failed to write to file");
            read_bytes = 3;
        },
        2 => write_buffer.write_fmt(format_args!("0x{:04x}: 0x{:02x} STAX B\n", program_counter, buffer[program_counter])).expect("Failed to write to file"),
        3 => write_buffer.write_fmt(format_args!("0x{:04x}: 0x{:02x} INX B\n", program_counter, buffer[program_counter])).expect("Failed to write to file"),
        4 => write_buffer.write_fmt(format_args!("0x{:04x}: 0x{:02x} INR B\n", program_counter, buffer[program_counter])).expect("Failed to write to file"),
        5 => write_buffer.write_fmt(format_args!("0x{:04x}: 0x{:02x} DCR B\n", program_counter, buffer[program_counter])).expect("Failed to write to file"),
        6 => {
            write_buffer.write_fmt(format_args!("0x{:04x}: 0x{:02x} MVI B  {:02x}\n", program_counter, buffer[program_counter], buffer[program_counter + 1])).expect("Failed to write to file");
            read_bytes = 2;
        },
        7 => write_buffer.write_fmt(format_args!("0x{:04x}: 0x{:02x} RLC\n", program_counter, buffer[program_counter])).expect("Failed to write to file"),
        9 => write_buffer.write_fmt(format_args!("0x{:04x}: 0x{:02x} DAD B\n", program_counter, buffer[program_counter])).expect("Failed to write to file"),
        10 => write_buffer.write_fmt(format_args!("0x{:04x}: 0x{:02x} LDAX B\n", program_counter, buffer[program_counter])).expect("Failed to write to file"),
        11 => write_buffer.write_fmt(format_args!("0x{:04x}: 0x{:02x} DCX B\n", program_counter, buffer[program_counter])).expect("Failed to write to file"),
        12 => write_buffer.write_fmt(format_args!("0x{:04x}: 0x{:02x} INR C\n", program_counter, buffer[program_counter])).expect("Failed to write to file"),
        13 => write_buffer.write_fmt(format_args!("0x{:04x}: 0x{:02x} DCR C\n", program_counter, buffer[program_counter])).expect("Failed to write to file"),
        14 => {
            write_buffer.write_fmt(format_args!("0x{:04x}: 0x{:02x} MVI C {:02x}\n", program_counter, buffer[program_counter], buffer[program_counter + 1])).expect("Failed to write to file");
            read_bytes = 2;
        },
        15 => write_buffer.write_fmt(format_args!("0x{:04x}: 0x{:02x} RRC\n", program_counter, buffer[program_counter])).expect("Failed to write to file"),
        17 => {
            write_buffer.write_fmt(format_args!("0x{:04x}: 0x{:02x} LXI D {:02x}{:02x}\n", program_counter, buffer[program_counter], buffer[program_counter + 1], buffer[program_counter + 2])).expect("Failed to write to file");
            read_bytes = 3;
        },
        18 => write_buffer.write_fmt(format_args!("0x{:04x}: 0x{:02x} STAX D\n", program_counter, buffer[program_counter])).expect("Failed to write to file"),
        19 => write_buffer.write_fmt(format_args!("0x{:04x}: 0x{:02x} INX D\n", program_counter, buffer[program_counter])).expect("Failed to write to file"),
        20 => write_buffer.write_fmt(format_args!("0x{:04x}: 0x{:02x} INR D\n", program_counter, buffer[program_counter])).expect("Failed to write to file"),
        21 => write_buffer.write_fmt(format_args!("0x{:04x}: 0x{:02x} DCR D\n", program_counter, buffer[program_counter])).expect("Failed to write to file"),
        22 => {
            write_buffer.write_fmt(format_args!("0x{:04x}: 0x{:02x} MVI D  {:02x}\n", program_counter, buffer[program_counter], buffer[program_counter + 1])).expect("Failed to write to file");
            read_bytes = 2;
        },
        23 => write_buffer.write_fmt(format_args!("0x{:04x}: 0x{:02x} RAL\n", program_counter, buffer[program_counter])).expect("Failed to write to file"),
        25 => write_buffer.write_fmt(format_args!("0x{:04x}: 0x{:02x} DAD D\n", program_counter, buffer[program_counter])).expect("Failed to write to file"),
        26 => write_buffer.write_fmt(format_args!("0x{:04x}: 0x{:02x} LDAX D\n", program_counter, buffer[program_counter])).expect("Failed to write to file"),
        27 => write_buffer.write_fmt(format_args!("0x{:04x}: 0x{:02x} DCX D\n", program_counter, buffer[program_counter])).expect("Failed to write to file"),
        28 => write_buffer.write_fmt(format_args!("0x{:04x}: 0x{:02x} INR E\n", program_counter, buffer[program_counter])).expect("Failed to write to file"),
        29 => write_buffer.write_fmt(format_args!("0x{:04x}: 0x{:02x} DCR E\n", program_counter, buffer[program_counter])).expect("Failed to write to file"),
        30 => {
            write_buffer.write_fmt(format_args!("0x{:04x}: 0x{:02x} MVI E {:02x}\n", program_counter, buffer[program_counter], buffer[program_counter + 1])).expect("Failed to write to file");
            read_bytes = 2;
        },
        31 => write_buffer.write_fmt(format_args!("0x{:04x}: 0x{:02x} RAR\n", program_counter, buffer[program_counter])).expect("Failed to write to file"),
        33 => {
            write_buffer.write_fmt(format_args!("0x{:04x}: 0x{:02x} LXI H {:02x}{:02x}\n", program_counter, buffer[program_counter], buffer[program_counter + 1], buffer[program_counter + 2])).expect("Failed to write to file");
            read_bytes = 3;
        },
        34 => {
            write_buffer.write_fmt(format_args!("0x{:04x}: 0x{:02x} SHLD {:02x}{:02x}\n", program_counter, buffer[program_counter], buffer[program_counter + 1], buffer[program_counter + 2])).expect("Failed to write to file");
            read_bytes = 3;
        },
        35 => write_buffer.write_fmt(format_args!("0x{:04x}: 0x{:02x} INX H\n", program_counter, buffer[program_counter])).expect("Failed to write to file"),
        36 => write_buffer.write_fmt(format_args!("0x{:04x}: 0x{:02x} INR H\n", program_counter, buffer[program_counter])).expect("Failed to write to file"),
        37 => write_buffer.write_fmt(format_args!("0x{:04x}: 0x{:02x} DCR H\n", program_counter, buffer[program_counter])).expect("Failed to write to file"),
        38 => {
            write_buffer.write_fmt(format_args!("0x{:04x}: 0x{:02x} MVI H {:02x}\n", program_counter, buffer[program_counter], buffer[program_counter + 1])).expect("Failed to write to file");
            read_bytes = 2;
        },
        39 => write_buffer.write_fmt(format_args!("0x{:04x}: 0x{:02x} DAA\n", program_counter, buffer[program_counter])).expect("Failed to write to file"),
        41 => write_buffer.write_fmt(format_args!("0x{:04x}: 0x{:02x} DAD H\n", program_counter, buffer[program_counter])).expect("Failed to write to file"),
        42 => {
            write_buffer.write_fmt(format_args!("0x{:04x}: 0x{:02x} LHLD {:02x}{:02x}\n", program_counter, buffer[program_counter], buffer[program_counter + 1], buffer[program_counter + 2])).expect("Failed to write to file");
            read_bytes = 3;
        },
        43 => write_buffer.write_fmt(format_args!("0x{:04x}: 0x{:02x} DCX H\n", program_counter, buffer[program_counter])).expect("Failed to write to file"),
        44 => write_buffer.write_fmt(format_args!("0x{:04x}: 0x{:02x} INR L\n", program_counter, buffer[program_counter])).expect("Failed to write to file"),
        45 => write_buffer.write_fmt(format_args!("0x{:04x}: 0x{:02x} DCR L\n", program_counter, buffer[program_counter])).expect("Failed to write to file"),
        46 => {
            write_buffer.write_fmt(format_args!("0x{:04x}: 0x{:02x} MVI L  {:02x}\n", program_counter, buffer[program_counter], buffer[program_counter + 1])).expect("Failed to write to file");
            read_bytes = 2;
        },
        47 => write_buffer.write_fmt(format_args!("0x{:04x}: 0x{:02x} CMA\n", program_counter, buffer[program_counter])).expect("Failed to write to file"),
        49 => {
            write_buffer.write_fmt(format_args!("0x{:04x}: 0x{:02x} LXI SP  {:02x}{:02x}\n", program_counter, buffer[program_counter], buffer[program_counter + 1], buffer[program_counter + 2])).expect("Failed to write to file");
            read_bytes = 3;
        },
        50 => {
            write_buffer.write_fmt(format_args!("0x{:04x}: 0x{:02x} STA {:02x}{:02x}\n", program_counter, buffer[program_counter], buffer[program_counter + 1], buffer[program_counter + 2])).expect("Failed to write to file");
            read_bytes = 3;
        },
        51 => write_buffer.write_fmt(format_args!("0x{:04x}: 0x{:02x} INX SP\n", program_counter, buffer[program_counter])).expect("Failed to write to file"),
        52 => write_buffer.write_fmt(format_args!("0x{:04x}: 0x{:02x} INR M\n", program_counter, buffer[program_counter])).expect("Failed to write to file"),
        53 => write_buffer.write_fmt(format_args!("0x{:04x}: 0x{:02x} DCR M\n", program_counter, buffer[program_counter])).expect("Failed to write to file"),
        54 => {
            write_buffer.write_fmt(format_args!("0x{:04x}: 0x{:02x} MVI M {:02x}\n", program_counter, buffer[program_counter], buffer[program_counter + 1])).expect("Failed to write to file");
            read_bytes = 2;
        },
        55 => write_buffer.write_fmt(format_args!("0x{:04x}: 0x{:02x} STC\n", program_counter, buffer[program_counter])).expect("Failed to write to file"),
        57 => write_buffer.write_fmt(format_args!("0x{:04x}: 0x{:02x} DAD SP\n", program_counter, buffer[program_counter])).expect("Failed to write to file"),
        58 => {
            write_buffer.write_fmt(format_args!("0x{:04x}: 0x{:02x} LDA {:02x}{:02x}\n", program_counter, buffer[program_counter], buffer[program_counter + 1], buffer[program_counter + 2])).expect("Failed to write to file");
            read_bytes = 3;
        },
        59 => write_buffer.write_fmt(format_args!("0x{:04x}: 0x{:02x} DCX SP\n", program_counter, buffer[program_counter])).expect("Failed to write to file"),
        60 => write_buffer.write_fmt(format_args!("0x{:04x}: 0x{:02x} INR A\n", program_counter, buffer[program_counter])).expect("Failed to write to file"),
        61 => write_buffer.write_fmt(format_args!("0x{:04x}: 0x{:02x} DCR A\n", program_counter, buffer[program_counter])).expect("Failed to write to file"),
        62 => {
            write_buffer.write_fmt(format_args!("0x{:04x}: 0x{:02x} MVI A {:02x}\n", program_counter, buffer[program_counter], buffer[program_counter + 1])).expect("Failed to write to file");
            read_bytes = 2;
        },
        63 => write_buffer.write_fmt(format_args!("0x{:04x}: 0x{:02x} CMC\n", program_counter, buffer[program_counter])).expect("Failed to write to file"),
        64 => write_buffer.write_fmt(format_args!("0x{:04x}: 0x{:02x} MOV B B\n", program_counter, buffer[program_counter])).expect("Failed to write to file"),
        65 => write_buffer.write_fmt(format_args!("0x{:04x}: 0x{:02x} MOV B C\n", program_counter, buffer[program_counter])).expect("Failed to write to file"),
        66 => write_buffer.write_fmt(format_args!("0x{:04x}: 0x{:02x} MOV B D\n", program_counter, buffer[program_counter])).expect("Failed to write to file"),
        67 => write_buffer.write_fmt(format_args!("0x{:04x}: 0x{:02x} MOV B E\n", program_counter, buffer[program_counter])).expect("Failed to write to file"),
        68 => write_buffer.write_fmt(format_args!("0x{:04x}: 0x{:02x} MOV B H\n", program_counter, buffer[program_counter])).expect("Failed to write to file"),
        69 => write_buffer.write_fmt(format_args!("0x{:04x}: 0x{:02x} MOV B L\n", program_counter, buffer[program_counter])).expect("Failed to write to file"),
        70 => write_buffer.write_fmt(format_args!("0x{:04x}: 0x{:02x} MOV B M\n", program_counter, buffer[program_counter])).expect("Failed to write to file"),
        71 => write_buffer.write_fmt(format_args!("0x{:04x}: 0x{:02x} MOV B A\n", program_counter, buffer[program_counter])).expect("Failed to write to file"),
        72 => write_buffer.write_fmt(format_args!("0x{:04x}: 0x{:02x} MOV C B\n", program_counter, buffer[program_counter])).expect("Failed to write to file"),
        73 => write_buffer.write_fmt(format_args!("0x{:04x}: 0x{:02x} MOV C C\n", program_counter, buffer[program_counter])).expect("Failed to write to file"),
        74 => write_buffer.write_fmt(format_args!("0x{:04x}: 0x{:02x} MOV C D\n", program_counter, buffer[program_counter])).expect("Failed to write to file"),
        75 => write_buffer.write_fmt(format_args!("0x{:04x}: 0x{:02x} MOV C E\n", program_counter, buffer[program_counter])).expect("Failed to write to file"),
        76 => write_buffer.write_fmt(format_args!("0x{:04x}: 0x{:02x} MOV C H\n", program_counter, buffer[program_counter])).expect("Failed to write to file"),
        77 => write_buffer.write_fmt(format_args!("0x{:04x}: 0x{:02x} MOV C L\n", program_counter, buffer[program_counter])).expect("Failed to write to file"),
        78 => write_buffer.write_fmt(format_args!("0x{:04x}: 0x{:02x} MOV C M\n", program_counter, buffer[program_counter])).expect("Failed to write to file"),
        79 => write_buffer.write_fmt(format_args!("0x{:04x}: 0x{:02x} MOV C A\n", program_counter, buffer[program_counter])).expect("Failed to write to file"),
        80 => write_buffer.write_fmt(format_args!("0x{:04x}: 0x{:02x} MOV D B\n", program_counter, buffer[program_counter])).expect("Failed to write to file"),
        81 => write_buffer.write_fmt(format_args!("0x{:04x}: 0x{:02x} MOV D C\n", program_counter, buffer[program_counter])).expect("Failed to write to file"),
        82 => write_buffer.write_fmt(format_args!("0x{:04x}: 0x{:02x} MOV D D\n", program_counter, buffer[program_counter])).expect("Failed to write to file"),
        83 => write_buffer.write_fmt(format_args!("0x{:04x}: 0x{:02x} MOV D E\n", program_counter, buffer[program_counter])).expect("Failed to write to file"),
        84 => write_buffer.write_fmt(format_args!("0x{:04x}: 0x{:02x} MOV D H\n", program_counter, buffer[program_counter])).expect("Failed to write to file"),
        85 => write_buffer.write_fmt(format_args!("0x{:04x}: 0x{:02x} MOV D L\n", program_counter, buffer[program_counter])).expect("Failed to write to file"),
        86 => write_buffer.write_fmt(format_args!("0x{:04x}: 0x{:02x} MOV D M\n", program_counter, buffer[program_counter])).expect("Failed to write to file"),
        87 => write_buffer.write_fmt(format_args!("0x{:04x}: 0x{:02x} MOV D A\n", program_counter, buffer[program_counter])).expect("Failed to write to file"),
        88 => write_buffer.write_fmt(format_args!("0x{:04x}: 0x{:02x} MOV E B\n", program_counter, buffer[program_counter])).expect("Failed to write to file"),
        89 => write_buffer.write_fmt(format_args!("0x{:04x}: 0x{:02x} MOV E C\n", program_counter, buffer[program_counter])).expect("Failed to write to file"),
        90 => write_buffer.write_fmt(format_args!("0x{:04x}: 0x{:02x} MOV E D\n", program_counter, buffer[program_counter])).expect("Failed to write to file"),
        91 => write_buffer.write_fmt(format_args!("0x{:04x}: 0x{:02x} MOV E E\n", program_counter, buffer[program_counter])).expect("Failed to write to file"),
        92 => write_buffer.write_fmt(format_args!("0x{:04x}: 0x{:02x} MOV E H\n", program_counter, buffer[program_counter])).expect("Failed to write to file"),
        93 => write_buffer.write_fmt(format_args!("0x{:04x}: 0x{:02x} MOV E L\n", program_counter, buffer[program_counter])).expect("Failed to write to file"),
        94 => write_buffer.write_fmt(format_args!("0x{:04x}: 0x{:02x} MOV E M\n", program_counter, buffer[program_counter])).expect("Failed to write to file"),
        95 => write_buffer.write_fmt(format_args!("0x{:04x}: 0x{:02x} MOV E A\n", program_counter, buffer[program_counter])).expect("Failed to write to file"),
        96 => write_buffer.write_fmt(format_args!("0x{:04x}: 0x{:02x} MOV H B\n", program_counter, buffer[program_counter])).expect("Failed to write to file"),
        97 => write_buffer.write_fmt(format_args!("0x{:04x}: 0x{:02x} MOV H C\n", program_counter, buffer[program_counter])).expect("Failed to write to file"),
        98 => write_buffer.write_fmt(format_args!("0x{:04x}: 0x{:02x} MOV H D\n", program_counter, buffer[program_counter])).expect("Failed to write to file"),
        99 => write_buffer.write_fmt(format_args!("0x{:04x}: 0x{:02x} MOV H E\n", program_counter, buffer[program_counter])).expect("Failed to write to file"),
        100 => write_buffer.write_fmt(format_args!("0x{:04x}: 0x{:02x} MOV H H\n", program_counter, buffer[program_counter])).expect("Failed to write to file"),
        101 => write_buffer.write_fmt(format_args!("0x{:04x}: 0x{:02x} MOV H L\n", program_counter, buffer[program_counter])).expect("Failed to write to file"),
        102 => write_buffer.write_fmt(format_args!("0x{:04x}: 0x{:02x} MOV H M\n", program_counter, buffer[program_counter])).expect("Failed to write to file"),
        103 => write_buffer.write_fmt(format_args!("0x{:04x}: 0x{:02x} MOV H A\n", program_counter, buffer[program_counter])).expect("Failed to write to file"),
        104 => write_buffer.write_fmt(format_args!("0x{:04x}: 0x{:02x} MOV L B\n", program_counter, buffer[program_counter])).expect("Failed to write to file"),
        105 => write_buffer.write_fmt(format_args!("0x{:04x}: 0x{:02x} MOV L C\n", program_counter, buffer[program_counter])).expect("Failed to write to file"),
        106 => write_buffer.write_fmt(format_args!("0x{:04x}: 0x{:02x} MOV L D\n", program_counter, buffer[program_counter])).expect("Failed to write to file"),
        107 => write_buffer.write_fmt(format_args!("0x{:04x}: 0x{:02x} MOV L E\n", program_counter, buffer[program_counter])).expect("Failed to write to file"),
        108 => write_buffer.write_fmt(format_args!("0x{:04x}: 0x{:02x} MOV L H\n", program_counter, buffer[program_counter])).expect("Failed to write to file"),
        109 => write_buffer.write_fmt(format_args!("0x{:04x}: 0x{:02x} MOV L L\n", program_counter, buffer[program_counter])).expect("Failed to write to file"),
        110 => write_buffer.write_fmt(format_args!("0x{:04x}: 0x{:02x} MOV L M\n", program_counter, buffer[program_counter])).expect("Failed to write to file"),
        111 => write_buffer.write_fmt(format_args!("0x{:04x}: 0x{:02x} MOV L A\n", program_counter, buffer[program_counter])).expect("Failed to write to file"),
        112 => write_buffer.write_fmt(format_args!("0x{:04x}: 0x{:02x} MOV M B\n", program_counter, buffer[program_counter])).expect("Failed to write to file"),
        113 => write_buffer.write_fmt(format_args!("0x{:04x}: 0x{:02x} MOV M C\n", program_counter, buffer[program_counter])).expect("Failed to write to file"),
        114 => write_buffer.write_fmt(format_args!("0x{:04x}: 0x{:02x} MOV M D\n", program_counter, buffer[program_counter])).expect("Failed to write to file"),
        115 => write_buffer.write_fmt(format_args!("0x{:04x}: 0x{:02x} MOV M E\n", program_counter, buffer[program_counter])).expect("Failed to write to file"),
        116 => write_buffer.write_fmt(format_args!("0x{:04x}: 0x{:02x} MOV M H\n", program_counter, buffer[program_counter])).expect("Failed to write to file"),
        117 => write_buffer.write_fmt(format_args!("0x{:04x}: 0x{:02x} MOV M L\n", program_counter, buffer[program_counter])).expect("Failed to write to file"),
        118 => write_buffer.write_fmt(format_args!("0x{:04x}: 0x{:02x} HLT\n", program_counter, buffer[program_counter])).expect("Failed to write to file"),
        119 => write_buffer.write_fmt(format_args!("0x{:04x}: 0x{:02x} MOV M A\n", program_counter, buffer[program_counter])).expect("Failed to write to file"),
        120 => write_buffer.write_fmt(format_args!("0x{:04x}: 0x{:02x} MOV A B\n", program_counter, buffer[program_counter])).expect("Failed to write to file"),
        121 => write_buffer.write_fmt(format_args!("0x{:04x}: 0x{:02x} MOV A C\n", program_counter, buffer[program_counter])).expect("Failed to write to file"),
        122 => write_buffer.write_fmt(format_args!("0x{:04x}: 0x{:02x} MOV A D\n", program_counter, buffer[program_counter])).expect("Failed to write to file"),
        123 => write_buffer.write_fmt(format_args!("0x{:04x}: 0x{:02x} MOV A E\n", program_counter, buffer[program_counter])).expect("Failed to write to file"),
        124 => write_buffer.write_fmt(format_args!("0x{:04x}: 0x{:02x} MOV A H\n", program_counter, buffer[program_counter])).expect("Failed to write to file"),
        125 => write_buffer.write_fmt(format_args!("0x{:04x}: 0x{:02x} MOV A L\n", program_counter, buffer[program_counter])).expect("Failed to write to file"),
        126 => write_buffer.write_fmt(format_args!("0x{:04x}: 0x{:02x} MOV A M\n", program_counter, buffer[program_counter])).expect("Failed to write to file"),
        127 => write_buffer.write_fmt(format_args!("0x{:04x}: 0x{:02x} MOV A A\n", program_counter, buffer[program_counter])).expect("Failed to write to file"),
        128 => write_buffer.write_fmt(format_args!("0x{:04x}: 0x{:02x} ADD B\n", program_counter, buffer[program_counter])).expect("Failed to write to file"),
        129 => write_buffer.write_fmt(format_args!("0x{:04x}: 0x{:02x} ADD C\n", program_counter, buffer[program_counter])).expect("Failed to write to file"),
        130 => write_buffer.write_fmt(format_args!("0x{:04x}: 0x{:02x} ADD D\n", program_counter, buffer[program_counter])).expect("Failed to write to file"),
        131 => write_buffer.write_fmt(format_args!("0x{:04x}: 0x{:02x} ADD E\n", program_counter, buffer[program_counter])).expect("Failed to write to file"),
        132 => write_buffer.write_fmt(format_args!("0x{:04x}: 0x{:02x} ADD H\n", program_counter, buffer[program_counter])).expect("Failed to write to file"),
        133 => write_buffer.write_fmt(format_args!("0x{:04x}: 0x{:02x} ADD L\n", program_counter, buffer[program_counter])).expect("Failed to write to file"),
        134 => write_buffer.write_fmt(format_args!("0x{:04x}: 0x{:02x} ADD M\n", program_counter, buffer[program_counter])).expect("Failed to write to file"),
        135 => write_buffer.write_fmt(format_args!("0x{:04x}: 0x{:02x} ADD A\n", program_counter, buffer[program_counter])).expect("Failed to write to file"),
        136 => write_buffer.write_fmt(format_args!("0x{:04x}: 0x{:02x} ADC B\n", program_counter, buffer[program_counter])).expect("Failed to write to file"),
        137 => write_buffer.write_fmt(format_args!("0x{:04x}: 0x{:02x} ADC C\n", program_counter, buffer[program_counter])).expect("Failed to write to file"),
        138 => write_buffer.write_fmt(format_args!("0x{:04x}: 0x{:02x} ADC D\n", program_counter, buffer[program_counter])).expect("Failed to write to file"),
        139 => write_buffer.write_fmt(format_args!("0x{:04x}: 0x{:02x} ADC E\n", program_counter, buffer[program_counter])).expect("Failed to write to file"),
        140 => write_buffer.write_fmt(format_args!("0x{:04x}: 0x{:02x} ADC H\n", program_counter, buffer[program_counter])).expect("Failed to write to file"),
        141 => write_buffer.write_fmt(format_args!("0x{:04x}: 0x{:02x} ADC L\n", program_counter, buffer[program_counter])).expect("Failed to write to file"),
        142 => write_buffer.write_fmt(format_args!("0x{:04x}: 0x{:02x} ADC M\n", program_counter, buffer[program_counter])).expect("Failed to write to file"),
        143 => write_buffer.write_fmt(format_args!("0x{:04x}: 0x{:02x} ADC A\n", program_counter, buffer[program_counter])).expect("Failed to write to file"),
        144 => write_buffer.write_fmt(format_args!("0x{:04x}: 0x{:02x} SUB B\n", program_counter, buffer[program_counter])).expect("Failed to write to file"),
        145 => write_buffer.write_fmt(format_args!("0x{:04x}: 0x{:02x} SUB C\n", program_counter, buffer[program_counter])).expect("Failed to write to file"),
        146 => write_buffer.write_fmt(format_args!("0x{:04x}: 0x{:02x} SUB D\n", program_counter, buffer[program_counter])).expect("Failed to write to file"),
        147 => write_buffer.write_fmt(format_args!("0x{:04x}: 0x{:02x} SUB E\n", program_counter, buffer[program_counter])).expect("Failed to write to file"),
        148 => write_buffer.write_fmt(format_args!("0x{:04x}: 0x{:02x} SUB H\n", program_counter, buffer[program_counter])).expect("Failed to write to file"),
        149 => write_buffer.write_fmt(format_args!("0x{:04x}: 0x{:02x} SUB L\n", program_counter, buffer[program_counter])).expect("Failed to write to file"),
        150 => write_buffer.write_fmt(format_args!("0x{:04x}: 0x{:02x} SUB M\n", program_counter, buffer[program_counter])).expect("Failed to write to file"),
        151 => write_buffer.write_fmt(format_args!("0x{:04x}: 0x{:02x} SUB A\n", program_counter, buffer[program_counter])).expect("Failed to write to file"),
        152 => write_buffer.write_fmt(format_args!("0x{:04x}: 0x{:02x} SBB B\n", program_counter, buffer[program_counter])).expect("Failed to write to file"),
        153 => write_buffer.write_fmt(format_args!("0x{:04x}: 0x{:02x} SBB C\n", program_counter, buffer[program_counter])).expect("Failed to write to file"),
        154 => write_buffer.write_fmt(format_args!("0x{:04x}: 0x{:02x} SBB D\n", program_counter, buffer[program_counter])).expect("Failed to write to file"),
        155 => write_buffer.write_fmt(format_args!("0x{:04x}: 0x{:02x} SBB E\n", program_counter, buffer[program_counter])).expect("Failed to write to file"),
        156 => write_buffer.write_fmt(format_args!("0x{:04x}: 0x{:02x} SBB H\n", program_counter, buffer[program_counter])).expect("Failed to write to file"),
        157 => write_buffer.write_fmt(format_args!("0x{:04x}: 0x{:02x} SBB L\n", program_counter, buffer[program_counter])).expect("Failed to write to file"),
        158 => write_buffer.write_fmt(format_args!("0x{:04x}: 0x{:02x} SBB M\n", program_counter, buffer[program_counter])).expect("Failed to write to file"),
        159 => write_buffer.write_fmt(format_args!("0x{:04x}: 0x{:02x} SBB A\n", program_counter, buffer[program_counter])).expect("Failed to write to file"),
        160 => write_buffer.write_fmt(format_args!("0x{:04x}: 0x{:02x} ANA B\n", program_counter, buffer[program_counter])).expect("Failed to write to file"),
        161 => write_buffer.write_fmt(format_args!("0x{:04x}: 0x{:02x} ANA C\n", program_counter, buffer[program_counter])).expect("Failed to write to file"),
        162 => write_buffer.write_fmt(format_args!("0x{:04x}: 0x{:02x} ANA D\n", program_counter, buffer[program_counter])).expect("Failed to write to file"),
        163 => write_buffer.write_fmt(format_args!("0x{:04x}: 0x{:02x} ANA E\n", program_counter, buffer[program_counter])).expect("Failed to write to file"),
        164 => write_buffer.write_fmt(format_args!("0x{:04x}: 0x{:02x} ANA H\n", program_counter, buffer[program_counter])).expect("Failed to write to file"),
        165 => write_buffer.write_fmt(format_args!("0x{:04x}: 0x{:02x} ANA L\n", program_counter, buffer[program_counter])).expect("Failed to write to file"),
        166 => write_buffer.write_fmt(format_args!("0x{:04x}: 0x{:02x} ANA M\n", program_counter, buffer[program_counter])).expect("Failed to write to file"),
        167 => write_buffer.write_fmt(format_args!("0x{:04x}: 0x{:02x} ANA A\n", program_counter, buffer[program_counter])).expect("Failed to write to file"),
        168 => write_buffer.write_fmt(format_args!("0x{:04x}: 0x{:02x} XRA B\n", program_counter, buffer[program_counter])).expect("Failed to write to file"),
        169 => write_buffer.write_fmt(format_args!("0x{:04x}: 0x{:02x} XRA C\n", program_counter, buffer[program_counter])).expect("Failed to write to file"),
        170 => write_buffer.write_fmt(format_args!("0x{:04x}: 0x{:02x} XRA D\n", program_counter, buffer[program_counter])).expect("Failed to write to file"),
        171 => write_buffer.write_fmt(format_args!("0x{:04x}: 0x{:02x} XRA E\n", program_counter, buffer[program_counter])).expect("Failed to write to file"),
        172 => write_buffer.write_fmt(format_args!("0x{:04x}: 0x{:02x} XRA H\n", program_counter, buffer[program_counter])).expect("Failed to write to file"),
        173 => write_buffer.write_fmt(format_args!("0x{:04x}: 0x{:02x} XRA L\n", program_counter, buffer[program_counter])).expect("Failed to write to file"),
        174 => write_buffer.write_fmt(format_args!("0x{:04x}: 0x{:02x} XRA M\n", program_counter, buffer[program_counter])).expect("Failed to write to file"),
        175 => write_buffer.write_fmt(format_args!("0x{:04x}: 0x{:02x} XRA A\n", program_counter, buffer[program_counter])).expect("Failed to write to file"),
        176 => write_buffer.write_fmt(format_args!("0x{:04x}: 0x{:02x} ORA B\n", program_counter, buffer[program_counter])).expect("Failed to write to file"),
        177 => write_buffer.write_fmt(format_args!("0x{:04x}: 0x{:02x} ORA C\n", program_counter, buffer[program_counter])).expect("Failed to write to file"),
        178 => write_buffer.write_fmt(format_args!("0x{:04x}: 0x{:02x} ORA D\n", program_counter, buffer[program_counter])).expect("Failed to write to file"),
        179 => write_buffer.write_fmt(format_args!("0x{:04x}: 0x{:02x} ORA E\n", program_counter, buffer[program_counter])).expect("Failed to write to file"),
        180 => write_buffer.write_fmt(format_args!("0x{:04x}: 0x{:02x} ORA H\n", program_counter, buffer[program_counter])).expect("Failed to write to file"),
        181 => write_buffer.write_fmt(format_args!("0x{:04x}: 0x{:02x} ORA L\n", program_counter, buffer[program_counter])).expect("Failed to write to file"),
        182 => write_buffer.write_fmt(format_args!("0x{:04x}: 0x{:02x} ORA M\n", program_counter, buffer[program_counter])).expect("Failed to write to file"),
        183 => write_buffer.write_fmt(format_args!("0x{:04x}: 0x{:02x} ORA A\n", program_counter, buffer[program_counter])).expect("Failed to write to file"),
        184 => write_buffer.write_fmt(format_args!("0x{:04x}: 0x{:02x} CMP B\n", program_counter, buffer[program_counter])).expect("Failed to write to file"),
        185 => write_buffer.write_fmt(format_args!("0x{:04x}: 0x{:02x} CMP C\n", program_counter, buffer[program_counter])).expect("Failed to write to file"),
        186 => write_buffer.write_fmt(format_args!("0x{:04x}: 0x{:02x} CMP D\n", program_counter, buffer[program_counter])).expect("Failed to write to file"),
        187 => write_buffer.write_fmt(format_args!("0x{:04x}: 0x{:02x} CMP E\n", program_counter, buffer[program_counter])).expect("Failed to write to file"),
        188 => write_buffer.write_fmt(format_args!("0x{:04x}: 0x{:02x} CMP H\n", program_counter, buffer[program_counter])).expect("Failed to write to file"),
        189 => write_buffer.write_fmt(format_args!("0x{:04x}: 0x{:02x} CMP L\n", program_counter, buffer[program_counter])).expect("Failed to write to file"),
        190 => write_buffer.write_fmt(format_args!("0x{:04x}: 0x{:02x} CMP M\n", program_counter, buffer[program_counter])).expect("Failed to write to file"),
        191 => write_buffer.write_fmt(format_args!("0x{:04x}: 0x{:02x} CMP A\n", program_counter, buffer[program_counter])).expect("Failed to write to file"),
        192 => write_buffer.write_fmt(format_args!("0x{:04x}: 0x{:02x} RNZ\n", program_counter, buffer[program_counter])).expect("Failed to write to file"),
        193 => write_buffer.write_fmt(format_args!("0x{:04x}: 0x{:02x} POP B\n", program_counter, buffer[program_counter])).expect("Failed to write to file"),
        194 => {
            write_buffer.write_fmt(format_args!("0x{:04x}: 0x{:02x} JNZ {:02x}{:02x}\n", program_counter, buffer[program_counter], buffer[program_counter + 1], buffer[program_counter + 2])).expect("Failed to write to file");
            read_bytes = 3;
        },
        195 => {
            write_buffer.write_fmt(format_args!("0x{:04x}: 0x{:02x} JMP {:02x}{:02x}\n", program_counter, buffer[program_counter], buffer[program_counter + 1], buffer[program_counter + 2])).expect("Failed to write to file");
            read_bytes = 3;
        },
        196 => {
            write_buffer.write_fmt(format_args!("0x{:04x}: 0x{:02x} CNZ {:02x}{:02x}\n", program_counter, buffer[program_counter], buffer[program_counter + 1], buffer[program_counter + 2])).expect("Failed to write to file");
            read_bytes = 3;
        },
        197 => write_buffer.write_fmt(format_args!("0x{:04x}: 0x{:02x} PUSH B\n", program_counter, buffer[program_counter])).expect("Failed to write to file"),
        198 => {
            write_buffer.write_fmt(format_args!("0x{:04x}: 0x{:02x} ADI {:02x}\n", program_counter, buffer[program_counter], buffer[program_counter + 1])).expect("Failed to write to file");
            read_bytes = 2;
        },
        199 => write_buffer.write_fmt(format_args!("0x{:04x}: 0x{:02x} RST 0\n", program_counter, buffer[program_counter])).expect("Failed to write to file"),
        200 => write_buffer.write_fmt(format_args!("0x{:04x}: 0x{:02x} RZ\n", program_counter, buffer[program_counter])).expect("Failed to write to file"),
        201 => write_buffer.write_fmt(format_args!("0x{:04x}: 0x{:02x} RET\n", program_counter, buffer[program_counter])).expect("Failed to write to file"),
        202 => {
            write_buffer.write_fmt(format_args!("0x{:04x}: 0x{:02x} JZ {:02x}{:02x}\n", program_counter, buffer[program_counter], buffer[program_counter + 1], buffer[program_counter + 2])).expect("Failed to write to file");
            read_bytes = 3;
        },
        204 => {
            write_buffer.write_fmt(format_args!("0x{:04x}: 0x{:02x} CZ {:02x}{:02x}\n", program_counter, buffer[program_counter], buffer[program_counter + 1], buffer[program_counter + 2])).expect("Failed to write to file");
            read_bytes = 3;
        },
        205 => {
            write_buffer.write_fmt(format_args!("0x{:04x}: 0x{:02x} CALL {:02x}{:02x}\n", program_counter, buffer[program_counter], buffer[program_counter + 1], buffer[program_counter + 2])).expect("Failed to write to file");
            read_bytes = 3;
        },
        206 => {
            write_buffer.write_fmt(format_args!("0x{:04x}: 0x{:02x} ACI {:02x}\n", program_counter, buffer[program_counter], buffer[program_counter + 1])).expect("Failed to write to file");
            read_bytes = 2;
        },
        207 => write_buffer.write_fmt(format_args!("0x{:04x}: 0x{:02x} RST 1\n", program_counter, buffer[program_counter])).expect("Failed to write to file"),
        208 => write_buffer.write_fmt(format_args!("0x{:04x}: 0x{:02x} RNC\n", program_counter, buffer[program_counter])).expect("Failed to write to file"),
        209 => write_buffer.write_fmt(format_args!("0x{:04x}: 0x{:02x} POP D\n", program_counter, buffer[program_counter])).expect("Failed to write to file"),
        210 => {
            write_buffer.write_fmt(format_args!("0x{:04x}: 0x{:02x} JNC {:02x}{:02x}\n", program_counter, buffer[program_counter], buffer[program_counter + 1], buffer[program_counter + 2])).expect("Failed to write to file");
            read_bytes = 3;
        },
        211 => {
            write_buffer.write_fmt(format_args!("0x{:04x}: 0x{:02x} OUT {:02x}\n", program_counter, buffer[program_counter], buffer[program_counter + 1])).expect("Failed to write to file");
            read_bytes = 2;
        },
        212 => {
            write_buffer.write_fmt(format_args!("0x{:04x}: 0x{:02x} CNC {:02x}{:02x}\n", program_counter, buffer[program_counter], buffer[program_counter + 1], buffer[program_counter + 2])).expect("Failed to write to file");
            read_bytes = 3;
        },
        213 => write_buffer.write_fmt(format_args!("0x{:04x}: 0x{:02x} PUSH D\n", program_counter, buffer[program_counter])).expect("Failed to write to file"),
        214 => {
            write_buffer.write_fmt(format_args!("0x{:04x}: 0x{:02x} SUI {:02x}\n", program_counter, buffer[program_counter], buffer[program_counter + 1])).expect("Failed to write to file");
            read_bytes = 2;
        },
        215 => write_buffer.write_fmt(format_args!("0x{:04x}: 0x{:02x} RST 2\n", program_counter, buffer[program_counter])).expect("Failed to write to file"),
        216 => write_buffer.write_fmt(format_args!("0x{:04x}: 0x{:02x} RC\n", program_counter, buffer[program_counter])).expect("Failed to write to file"),
        218 => {
            write_buffer.write_fmt(format_args!("0x{:04x}: 0x{:02x} JC {:02x}{:02x}\n", program_counter, buffer[program_counter], buffer[program_counter + 1], buffer[program_counter + 2])).expect("Failed to write to file");
            read_bytes = 3;
        },
        219 => {
            write_buffer.write_fmt(format_args!("0x{:04x}: 0x{:02x} IN {:02x}\n", program_counter, buffer[program_counter], buffer[program_counter + 1])).expect("Failed to write to file");
            read_bytes = 2;
        },
        220 => {
            write_buffer.write_fmt(format_args!("0x{:04x}: 0x{:02x} CC {:02x}{:02x}\n", program_counter, buffer[program_counter], buffer[program_counter + 1], buffer[program_counter + 2])).expect("Failed to write to file");
            read_bytes = 3;
        },
        222 => {
            write_buffer.write_fmt(format_args!("0x{:04x}: 0x{:02x} SBI {:02x}\n", program_counter, buffer[program_counter], buffer[program_counter + 1])).expect("Failed to write to file");
            read_bytes = 2;
        },
        223 => write_buffer.write_fmt(format_args!("0x{:04x}: 0x{:02x} RST 3\n", program_counter, buffer[program_counter])).expect("Failed to write to file"),
        224 => write_buffer.write_fmt(format_args!("0x{:04x}: 0x{:02x} RPO\n", program_counter, buffer[program_counter])).expect("Failed to write to file"),
        225 => write_buffer.write_fmt(format_args!("0x{:04x}: 0x{:02x} POP H\n", program_counter, buffer[program_counter])).expect("Failed to write to file"),
        226 => {
            write_buffer.write_fmt(format_args!("0x{:04x}: 0x{:02x} JPO {:02x}{:02x}\n", program_counter, buffer[program_counter], buffer[program_counter + 1], buffer[program_counter + 2])).expect("Failed to write to file");
            read_bytes = 3;
        },
        227 => write_buffer.write_fmt(format_args!("0x{:04x}: 0x{:02x} XTHL\n", program_counter, buffer[program_counter])).expect("Failed to write to file"),
        228 => {
            write_buffer.write_fmt(format_args!("0x{:04x}: 0x{:02x} CPO {:02x}{:02x}\n", program_counter, buffer[program_counter], buffer[program_counter + 1], buffer[program_counter + 2])).expect("Failed to write to file");
            read_bytes = 3;
        },
        229 => write_buffer.write_fmt(format_args!("0x{:04x}: 0x{:02x} PUSH H\n", program_counter, buffer[program_counter])).expect("Failed to write to file"),
        230 => {
            write_buffer.write_fmt(format_args!("0x{:04x}: 0x{:02x} ANI {:02x}\n", program_counter, buffer[program_counter], buffer[program_counter + 1])).expect("Failed to write to file");
            read_bytes = 2;
        },
        231 => write_buffer.write_fmt(format_args!("0x{:04x}: 0x{:02x} RST 4\n", program_counter, buffer[program_counter])).expect("Failed to write to file"),
        232 => write_buffer.write_fmt(format_args!("0x{:04x}: 0x{:02x} RPE\n", program_counter, buffer[program_counter])).expect("Failed to write to file"),
        233 => write_buffer.write_fmt(format_args!("0x{:04x}: 0x{:02x} PCHL\n", program_counter, buffer[program_counter])).expect("Failed to write to file"),
        234 => {
            write_buffer.write_fmt(format_args!("0x{:04x}: 0x{:02x} JPE {:02x}{:02x}\n", program_counter, buffer[program_counter], buffer[program_counter + 1], buffer[program_counter + 2])).expect("Failed to write to file");
            read_bytes = 3;
        },
        235 => write_buffer.write_fmt(format_args!("0x{:04x}: 0x{:02x} XCHG\n", program_counter, buffer[program_counter])).expect("Failed to write to file"),
        236 => {
            write_buffer.write_fmt(format_args!("0x{:04x}: 0x{:02x} CPE {:02x}{:02x}\n", program_counter, buffer[program_counter], buffer[program_counter + 1], buffer[program_counter + 2])).expect("Failed to write to file");
            read_bytes = 3;
        },
        238 => {
            write_buffer.write_fmt(format_args!("0x{:04x}: 0x{:02x} XRI {:02x}\n", program_counter, buffer[program_counter], buffer[program_counter + 1])).expect("Failed to write to file");
            read_bytes = 2;
        },
        239 => write_buffer.write_fmt(format_args!("0x{:04x}: 0x{:02x} RST 5\n", program_counter, buffer[program_counter])).expect("Failed to write to file"),
        240 => write_buffer.write_fmt(format_args!("0x{:04x}: 0x{:02x} RP\n", program_counter, buffer[program_counter])).expect("Failed to write to file"),
        241 => write_buffer.write_fmt(format_args!("0x{:04x}: 0x{:02x} POP PSW\n", program_counter, buffer[program_counter])).expect("Failed to write to file"),
        242 => {
            write_buffer.write_fmt(format_args!("0x{:04x}: 0x{:02x} JP {:02x}{:02x}\n", program_counter, buffer[program_counter], buffer[program_counter + 1], buffer[program_counter + 2])).expect("Failed to write to file");
            read_bytes = 3;
        },
        243 => write_buffer.write_fmt(format_args!("0x{:04x}: 0x{:02x} DI\n", program_counter, buffer[program_counter])).expect("Failed to write to file"),
        244 => {
            write_buffer.write_fmt(format_args!("0x{:04x}: 0x{:02x} CP {:02x}{:02x}\n", program_counter, buffer[program_counter], buffer[program_counter + 1], buffer[program_counter + 2])).expect("Failed to write to file");
            read_bytes = 3;
        },
        245 => write_buffer.write_fmt(format_args!("0x{:04x}: 0x{:02x} PUSH PSW\n", program_counter, buffer[program_counter])).expect("Failed to write to file"),
        246 => {
            write_buffer.write_fmt(format_args!("0x{:04x}: 0x{:02x} ORI {:02x}\n", program_counter, buffer[program_counter], buffer[program_counter + 1])).expect("Failed to write to file");
            read_bytes = 2;
        },
        247 => write_buffer.write_fmt(format_args!("0x{:04x}: 0x{:02x} RST 6\n", program_counter, buffer[program_counter])).expect("Failed to write to file"),
        248 => write_buffer.write_fmt(format_args!("0x{:04x}: 0x{:02x} RM\n", program_counter, buffer[program_counter])).expect("Failed to write to file"),
        249 => write_buffer.write_fmt(format_args!("0x{:04x}: 0x{:02x} SPHL\n", program_counter, buffer[program_counter])).expect("Failed to write to file"),
        250 => {
            write_buffer.write_fmt(format_args!("0x{:04x}: 0x{:02x} JM {:02x}{:02x}\n", program_counter, buffer[program_counter], buffer[program_counter + 1], buffer[program_counter + 2])).expect("Failed to write to file");
            read_bytes = 3;
        },
        251 => write_buffer.write_fmt(format_args!("0x{:04x}: 0x{:02x} EI\n", program_counter, buffer[program_counter])).expect("Failed to write to file"),
        252 => {
            write_buffer.write_fmt(format_args!("0x{:04x}: 0x{:02x} CM {:02x}{:02x}\n", program_counter, buffer[program_counter], buffer[program_counter + 1], buffer[program_counter + 2])).expect("Failed to write to file");
            read_bytes = 3;
        },
        254 => {
            write_buffer.write_fmt(format_args!("0x{:04x}: 0x{:02x} CPI {:02x}\n", program_counter, buffer[program_counter], buffer[program_counter + 1])).expect("Failed to write to file");
            read_bytes = 2;
        },
        255 => write_buffer.write_fmt(format_args!("0x{:04x}: 0x{:02x} RST 7\n", program_counter, buffer[program_counter])).expect("Failed to write to file"),
        
		_   => {},
    }

    read_bytes

}