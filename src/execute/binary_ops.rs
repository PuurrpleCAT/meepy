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
        0 => op0(u, registers),
        _ => exit("God's false words"),
    }
}
fn op0(u: u32, registers: &mut Registers) {
    let func = u << 27 >> 27;
    match func {
        12 => syscall(registers),
        _  => exit("thy a bit noob"),
    }
}
fn r1(u: u32) -> u8 {
    (u << 6 >> 27) as u8
}

fn r2(u: u32) -> u8 {
    (u << 11 >> 27) as u8
}

fn r3(u: u32) -> u8 {
    (u << 16 >> 27) as u8
}

fn sh(u: u32) -> u8 {
    (u << 21 >> 27) as u8
}

// woah
// lets say last 8 bits are 12345678, decimal digits for clarity
// << 3 makes it 34567800
// >> 3 makes it 00345678 -> the 6 bits we want for func, we want the first two bits to be
// zeroed so we can equate it easier
fn fu(u: u32) -> u8 {
    (u as u8) << 2 >> 2
}

fn im(u: u32) -> u16 {
    u as u16
}

fn op(u: u32) -> u8 {
    (u>>26) as u8
}
