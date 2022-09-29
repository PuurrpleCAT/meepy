use std::collections::HashMap;
mod to_bin;
mod op_codes;
mod data;
// global variables to keep state even though convert_and_store is called multiple times
static mut IS_DATA: u8 = 0;
static mut INSTRUCTION_ADDRESS: u32 = 0;
// we convert each line into its 32bit machine code, the instruction_address is incremente by
// 32, as each instruction is held in 32 bits;
pub fn convert_and_store(s: String, memory: &mut HashMap<u32, u32>, label_addr: &mut HashMap<String, u32>) {
    if s == ".data" {
        unsafe {IS_DATA = 1};
        return
    }
    if unsafe {IS_DATA == 0} {
        if s.chars().last().unwrap() == ':' {
            return
        }
        let b = to_bin::convert(s, label_addr);   
        if let Some(_) = memory.insert(unsafe{INSTRUCTION_ADDRESS}, b) { // note .insert returns Some(T) if .insert replaced a value
            panic!("Thou hast tried to defenestrate instruction");
        }
        unsafe {INSTRUCTION_ADDRESS += 32};
    } else {
        println!("{}", s);
        let shift = data::insert(s, memory, unsafe{INSTRUCTION_ADDRESS});
        unsafe {INSTRUCTION_ADDRESS += shift};
    }
}
