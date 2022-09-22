use std::sync::mpsc::Sender;
use std::fs::File;
use std::io::{BufReader, BufRead};


// reads the file line by line, separating .data from .text, and sending data down the channel
// after all .text, as .text comes first in the program memory layout. We store the instructions
// in a vec to return
pub fn read(s: Sender<String>) -> Vec<String> {
    let f: File = File::open("src/asm.txt").unwrap(); 
    let f = BufReader::new(f);
    let mut data_vec: Vec<String> = Vec::new();
    let mut instructions: Vec<String> = Vec::new();
    let mut is_data: u8 = 0;
    for line in f.lines() {
        let l = line.unwrap();
        let l = l.trim();
        if l == ".data" {
            is_data = 1;
            continue
        } else if l == ".text" {
            is_data = 0;
            continue
        }
        if is_data == 1 {
            data_vec.push(l.to_string());
        } else {
            if l.chars().last().unwrap() != ':' {
                instructions.push(l.to_string());
            }
            s.send(l.to_string()).unwrap();
        }
    }
    if data_vec.len() > 0 {
        s.send(".data".to_string()).unwrap();
        for line in data_vec {
            s.send(line).unwrap();
        }
    }
    instructions
}
