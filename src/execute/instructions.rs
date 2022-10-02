// note we need to update the pc pointer after every instruction, this allows branch instructions,
// and others to directly change the return address (register 31) or the pc register (register 32)
use crate::Registers;
use crate::{ exit };
pub fn addiu(r: &mut Registers, rs: u8, rd: u8, imm: u16) {
    r.update(rd, (r.get(rs) as u64 + imm as u64) as u32); // surprisingly as u32 ignores overflow
    r.update(32, r.get(32)+32);
}
pub fn add(r: &mut Registers, rs: u8, rt: u8, rd: u8) {
    r.update(rd, r.get(rs) + r.get(rt));
    r.update(32, r.get(32)+32);
}
pub fn syscall(r: &mut Registers) {
    match r.get(2) {
        10 => r.update(101, 0),
        _  => exit("idk, does the bible say so?"),
    }
}
