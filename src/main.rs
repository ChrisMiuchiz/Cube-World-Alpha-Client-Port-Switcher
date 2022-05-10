use clap::Parser;
use std::fs::{read, write};
use byteorder::{LittleEndian, WriteBytesExt};

const PUSH_12345: &'static [u8] = &[0x68, 0x39, 0x30, 0x00, 0x00];

#[derive(Parser, Debug)]
struct Arguments {
    /// The Cube World Alpha client to change
    input: String,

    /// Path to the file to be generated
    output: String,

    /// The new port
    port: u16
}

fn find_seq(buf: &Vec<u8>, target: &Vec<u8>) -> Option<usize> {
    let mut match_start: Option<usize> = None;
    if buf.len() > 0 && target.len() > 0 {
        for (i, e) in buf.iter().enumerate() {
            // Figure out where to compare in the target
            let target_i: usize = if let Some(start) = match_start {
                    i - start
                }
                else {
                    0
                };

            // Check for match
            if buf[i] == target[target_i] {
                if let None = match_start {
                    match_start = Some(i);
                }
            }
            else {
                match_start = None;
            }

            // Break if we finished matching
            if target_i == target.len() - 1 {
                break;
            }
        }

        if let Some(start) = match_start {
            if start > buf.len() - target.len() {
                match_start = None;
            }
        }
    }

    match_start
}

fn main() {
    let arguments = Arguments::parse();

    let mut contents: Vec<u8> = read(&arguments.input)
        .expect("Unable to read file.");

    let push_port_index = find_seq(&contents, &PUSH_12345.to_vec())
        .expect("Could not find \"PUSH 12345\" in executable.");

    let mut new_instruction: Vec<u8> =  vec![];
    new_instruction.write_u8(0x68).unwrap(); // Push
    new_instruction.write_u16::<LittleEndian>(arguments.port).unwrap();

    for i in 0..new_instruction.len() {
        contents[push_port_index + i] = new_instruction[i];
    }

    write(&arguments.output, contents)
        .expect("Failed to write file.");
}