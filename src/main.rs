mod load_file;
mod program;
mod machine_code;
mod execute;


// virtual memory and registers
use std::collections::HashMap;


// program::init also returns us the individual instructions in order as written in file
// this makes it so we can print the line as we execute
fn main() {
    use crossterm::terminal::size;
    let size = size().unwrap();
    assert!(size.1 >= 35);
    let mut memory:     HashMap<u32, u32>    = HashMap::new();
    let mut label_addr: HashMap<String, u32> = HashMap::new();
    let mut registers:  Registers            = Registers::new();
    let instructions:   Vec<String>          = program::init(&mut memory, &mut label_addr);
    registers.init_start_addr(&label_addr);
    execute::x(&mut memory, instructions, &mut registers); 
}
pub struct Registers {
    r: HashMap<u8, u32>,
}
impl Registers {
    fn new() -> Self {
        let mut r: HashMap<u8, u32> = HashMap::new();
        Registers::init(&mut r);
        Self {
            r,
        } 
    }
    // creates register hashmap
    fn init(h: &mut HashMap<u8, u32>) {
        for r in 0..32 {
            h.insert(r, 0);
        }
    }
    // initialise the pc counter
    fn init_start_addr(&mut self, label: &HashMap<String, u32>) {
        self.r.insert(32, *label.get(&"main:".to_string()).unwrap());
    }
    // helper get and update functions to keep registers field private
    pub fn get(&self, u: u8) -> u32 {
        *self.r.get(&u).unwrap()
    }
    pub fn update(&mut self, u: u8, x: u32) {
        self.r.insert(u, x);
    }
    pub fn contains(&self, u: u8) -> bool {
        self.r.contains_key(&u)
    }
    // prints registers with proper register names, in 4 groups of hexadigit pairs
    pub fn print(&mut self) {
        let mut keys: Vec<&u8> = self.r.keys().collect(); 
        keys.sort_unstable();
        let mut keys = keys.iter();
        let reg_letter = [("$at", 0, 0), ("$v", 0, 1), ("$a", 0, 3),
                          ("$t", 0, 7), ("$s", 0, 7), ("$t", 8, 9), ("$k", 0, 1), 
                          ("$gp", 0, 0), ("$sp", 0, 0), ("$fp", 0, 0), ("$ra", 0, 0),
                          ("$pc", 0, 0)];
        println!("$zero  00 00 00 00");
        keys.next();
        for r in reg_letter {
            if r.2 == 0 {
                let num = self.r.get(keys.next().unwrap()).unwrap();
                println!("{}    {}", r.0, Registers::bin_to_hex(*num));
            } else {
                for n in r.1..=r.2 {
                    let num = self.r.get(keys.next().unwrap()).unwrap();
                    println!("{}    {}", format!("{}{}", r.0, n), Registers::bin_to_hex(*num));
                }
            }
        }
    }
    // the function that makes the 4 groups of hexadigit pairs
    fn bin_to_hex(u: u32) -> String {
        let hex = format!("{:08x}", u);
        hex.chars()
           .collect::<Vec<char>>()
           .chunks(2)
           .map(|x| {let mut r = x.to_vec().iter().collect::<String>(); r.push_str(" "); r})
           .collect::<String>()
    }
}
// helper exit function to reset the cursor position, as before it would overwrite the
// printed registers (as cursor position was like (32, 2)). Resets it to bottom of the terminal
pub fn exit(s: &str) -> ! {
    use crossterm::{execute, terminal::size, cursor::MoveTo};
    use std::io::stdout;
    let mut stdout = stdout();
    let size = size().unwrap();
    execute!(stdout, MoveTo(0, size.1 -2)).unwrap();
    if s == "gracefully shut down" {
        std::process::exit(0);
    }
    panic!("{}", s)
}
