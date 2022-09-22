use std::sync::mpsc::Sender;
use std::fs::File;
use std::io::{BufReader, BufRead};
pub fn read(s: Sender<String>) {
    let f: File = File::open("src/asm.txt").unwrap(); 
    let f = BufReader::new(f);
    for line in f.lines() {
        let l = line.unwrap();
        let l = l.trim();
        s.send(l.to_string());
    }
}
