use crate::load_file;
use crate::machine_code;
use std::sync::mpsc::{channel, Sender, Receiver};
use std::thread;
use std::collections::HashMap;
pub fn init(memory: &mut HashMap<u32, u32>) {
    let (sender, receiver): (Sender<String>, Receiver<String>) = channel();
    let handle = thread::spawn(|| {
        load_file::read(sender);
    });
    while let Ok(string) = receiver.recv() {
        machine_code::convert_and_store(string, memory);
    }
    handle.join().expect("hmm someone died");
}
