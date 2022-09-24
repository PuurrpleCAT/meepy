use std::collections::HashMap;
use crate::Registers;
mod binary_ops;
mod instructions;

// with the address, we divide 32 to find its corresponding instruction in the vec
pub fn x(m: &mut HashMap<u32, u32>, start_addr: u32, instructions: Vec<String>, registers: &mut Registers) {
    let mut addr = start_addr;
    for _ in 0..3 {
        let b = *m.get(&addr).unwrap();
        print(b, &instructions, addr);
        binary_ops::run(registers, m, b);
        addr += 32;
    }
}
fn print_all(m: &mut HashMap<u32, u32>) {
    let mut keys: Vec<&u32> = m.keys().collect();
    keys.sort_unstable();
    for i in keys {
        let fmt = u32_to_bit_fmt(*m.get(&i).unwrap());
        println!("{:08x}   {}", i, fmt);
    }
}
fn print(b: u32, i: &Vec<String>, addr: u32) {
    let fmt = u32_to_bit_fmt(b);
    println!("           {}\n{:08x}   {}\n", i[(addr/32) as usize], addr, fmt);
}
fn u32_to_bit_fmt(u: u32) -> String {
    let bits: Vec<char> = format!("{:032b}", u).chars().collect();
    let s1: String = bits[..=5].iter().collect();
    let s2: String = bits[6..=10].iter().collect();
    let s3: String = bits[11..=15].iter().collect();
    let s4: String = bits[16..=20].iter().collect();
    let s5: String = bits[21..=25].iter().collect();
    let s6: String = bits[26..].iter().collect();
    format!("{} {} {} {} {} {}", s1, s2, s3, s4, s5, s6)
}

