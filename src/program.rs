use crate::load_file;
use crate::machine_code;
use std::sync::mpsc::{channel, Sender, Receiver};
use std::thread::{self};
use std::fs::File;
use std::io::{BufReader, BufRead};
use std::collections::HashMap;


// fun thing to send file reading into another thread, which sends line by line down a channel
// which is received here and passed into the converter to machine code. The thread returns
// a vec of instructions as a string in order in file (without labels)
pub fn init(memory: &mut HashMap<u32, u32>, label_addr: &mut HashMap<String, u32>) -> Vec<String> {
    get_label_addr(label_addr);
    println!("{:?}", label_addr);
    let (sender, receiver): (Sender<String>, Receiver<String>) = channel();
    let handle = thread::spawn(|| {
        load_file::read(sender)
    });
    while let Ok(string) = receiver.recv() {
        machine_code::convert_and_store(string, memory, label_addr);
    }
    handle.join().expect("God refuses you")
}
// we get the addresses of the labels - because that was a problem I had, not being able to branch
// to a label, because I might have not actually read the label yet
fn get_label_addr(a: &mut HashMap<String, u32>) {
    let f = File::open("src/asm.txt").unwrap();
    let f = BufReader::new(f);
    let mut in_text = 0;
    let mut addr = 0;
    for l in f.lines() {
        let l = l.unwrap();
        let l = l.trim();
        if l == ".text" {
            in_text = 1;
            continue
        }
        if in_text == 1 {
            if l.chars().last().unwrap() == ':' {
                if let Some(_) = a.insert(l.to_string(), addr) {
                    panic!("Label already exists");
                } else {
                    continue
                }
            }
            addr += 32;
        }
    }
}
