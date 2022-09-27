use std::collections::HashMap;
use std::io::{ stdout, Stdout };
use crate::{ Registers, exit };
use crossterm::{execute, cursor::MoveTo, terminal::{Clear, ClearType}, Result};
mod binary_ops;
mod instructions;

// with the address, we divide 32 to find its corresponding instruction in the vec
pub fn x(m: &mut HashMap<u32, u32>, instructions: Vec<String>, registers: &mut Registers) {
    let mut stdout = stdout();
    execute!(stdout, Clear(ClearType::All)).unwrap(); 
    for _ in 0..3 {
        let addr: u32 = registers.get(32u8);
        let b: u32 = *m.get(&addr).unwrap();
        binary_ops::run(registers, m, b);
        print_everything(registers, &mut stdout, b, &instructions, addr).unwrap();
        if registers.contains(101) {
            exit("gracefully shut down"); 
        }
        let mut buf = String::new();
        std::io::stdin().read_line(&mut buf).unwrap();
    }
}
// move commands to print registers and current executed instruction
pub fn print_everything(r: &mut Registers, s: &mut Stdout, b: u32, i: &Vec<String>, addr: u32) -> Result<()> {
    execute!(s, MoveTo(0, 0), Clear(ClearType::Purge))?;
    r.print(); 
    execute!(s, MoveTo(30, 0))?;
    print(s, b, i, addr)?;
    execute!(s, MoveTo(30, 2))
}
// prints the instrution and its binary format
fn print(s: &mut Stdout, b: u32, i: &Vec<String>, addr: u32) -> Result<()> {
    let fmt = u32_to_bit_fmt(b);
    println!("{}", i[(addr/32) as usize]); // this is a smart thing as the instructions in this vec are in the same order as the file, and so are the addresses in that same order, so we divide the address by 32 (as the instructions address are 0, 32, 64 ... ) to get the instruction index in the vec
    execute!(s, MoveTo(30, 1))?;
    println!("{:08x}   {}", addr, fmt); Ok(())
}
// put it into the 6 5 5 5 5 6 bit format
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

