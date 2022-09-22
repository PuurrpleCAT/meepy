use std::collections::HashMap;
mod to_bin;
mod op_codes;
static mut IS_DATA: u8 = 0;
static mut PROGRAM_ADDRESS: u32 = 0;
pub fn convert_and_store(s: String, memory: &mut HashMap<u32, u32>, labels: &mut HashMap<String, u32>) {
    if s == ".data" {
        unsafe {IS_DATA = 1};
    }
    if unsafe {IS_DATA == 0} {
        if s.chars().last().unwrap() == ':' {
            if let Some(_) = labels.insert(s, unsafe{PROGRAM_ADDRESS}) {
                panic!("This label already exists, choose a different one you dunce");
            }
            return
        }
        let b = to_bin::convert(s);   
        unsafe {PROGRAM_ADDRESS += 32};
    } else {
        todo!() // <-- the .data section of program
    }
}
