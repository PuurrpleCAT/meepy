use std::collections::HashMap;
use crate::execute::instructions::*;
use crate::Registers;
use crate::exit;

// op rs rt rd shamt funct
// op rs rd imm
pub fn run(registers: &mut Registers, memory: &mut HashMap<u32, u32>, u: u32) {
    let op = op(u);
    match op {
        9 => addiu(registers, r1(u), r2(u), im(u)),
        _ => exit("God's false words"),
    }
}

pub fn r1(u: u32) -> u8 {
    (u << 6 >> 27) as u8
}

pub fn r2(u: u32) -> u8 {
    (u << 11 >> 27) as u8
}

pub fn r3(u: u32) -> u8 {
    (u << 16 >> 27) as u8
}

pub fn sh(u: u32) -> u8 {
    (u << 21 >> 27) as u8
}

// woah
// lets say last 8 bits are 12345678, decimal digits for clarity
// << 3 makes it 34567800
// >> 3 makes it 00345678 -> the 6 bits we want for func, we want the first two bits to be
// zeroed so we can equate it easier
pub fn fu(u: u32) -> u8 {
    (u as u8) << 2 >> 2
}

pub fn im(u: u32) -> u16 {
    u as u16
}

pub fn op(u: u32) -> u8 {
    (u>>26) as u8
}
