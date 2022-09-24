use crate::machine_code::op_codes;
use std::collections::HashMap;


// get the op func and mode from op_codes dictionary (see op_codes.rs), bitshift add to create
// the 32bit machine code
pub fn convert(s: String, label_addr: &mut HashMap<String, u32>) -> u32 {
    let separate: Vec<&str> = s.split(' ').collect();
    let instruction: &str = separate[0];
    let (op, func, mode): (u32, Option<u32>, u8) = op_codes::get_op_and_func(instruction);
    let mut binary: u32 = 0;
    binary += op << 26;
        if mode == 0 {        // R form   op rs rt rd shamt func
            let rs = reg(&separate[2][..separate[2].len()-1]);
            let rt = reg(separate[3]);
            let rd = reg(&separate[1][..separate[1].len()-1]);
            todo!()
        } else if mode == 1 { // I form   op rs rd imm
            let rs = reg(&separate[2][..separate[2].len()-1]);
            let rd = reg(&separate[1][..separate[1].len()-1]);
            let imm = separate[3].parse::<u32>().unwrap();
    binary |= rs << 21;
    binary |= rd << 16;
    binary |= imm;
        } else if mode == 2 { // J form op 26 bit address eg b

        } else if mode == 3 { // idk form op rs rt? 16-bit address eg bgez or beq

        } else if mode == 4 { // idk form 0 0 (for 20 bits) 12 or 0 for syscall or nop

        }
   binary 
}
fn reg(s: &str) -> u32 {
    match s {
        "$zero" => 0,
        "$v0"   => 2,
        "$v1"   => 3,
        "$a0"   => 4,
        "$a1"   => 5,
        "$a2"   => 6,
        "$a3"   => 7,
        "$t0"   => 8,
        "$t1"   => 9,
        "$t2"   => 10,
        "$t3"   => 11,
        "$t4"   => 12,
        "$t5"   => 13,
        "$t6"   => 14,
        "$t7"   => 15,
        "$s0"   => 16,
        "$s1"   => 17,
        "$s2"   => 18,
        "$s3"   => 19,
        "$s4"   => 20,
        "$s5"   => 21,
        "$s6"   => 22,
        "$s7"   => 23,
        "$t8"   => 24,
        "$t9"   => 25,
        "$gp"   => 28,
        "$sp"   => 29,
        "$fp"   => 30,
        "$ra"   => 31,
        _       => panic!("You have used a demon register"),
    }
}
