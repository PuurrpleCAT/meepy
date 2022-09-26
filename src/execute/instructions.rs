use crate::Registers;
pub fn addiu(r: &mut Registers, rs: u8, rd: u8, imm: u16) {
    r.update(rd, (r.get(rs) as u64 + imm as u64) as u32); // overflow is ignored
    r.update(32, r.get(32)+32);
}
