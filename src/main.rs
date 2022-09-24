mod load_file;
mod program;
mod machine_code;
mod execute;

// virtual memory and registers
use std::collections::HashMap;


// program::init also returns us the individual instructions in order as written in file
// this makes it so we can print the line as we execute
fn main() {
    let mut memory:     HashMap<u32, u32>    = HashMap::new();
    let mut label_addr: HashMap<String, u32> = HashMap::new();
    let mut registers:  Registers            = Registers::new();
    let instructions:   Vec<String>          = program::init(&mut memory, &mut label_addr);
    execute::x(&mut memory, *label_addr.get(&"main:".to_string()).unwrap(), instructions, &mut registers); 
}
pub struct Registers {
    r: HashMap<u8, u32>,
}
impl Registers {
    fn new() -> Self {
        Self {
            r: HashMap::<u8, u32>::new(),
        } 
    }
    pub fn get(&self, u: u8) -> u32 {
        *self.r.get(&u).unwrap()
    }
    pub fn update(&mut self, u: u8, x: u32) {
        self.r.insert(u, x);
    }
}
