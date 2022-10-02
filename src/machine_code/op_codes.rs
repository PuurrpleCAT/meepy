// 0     => R form
// 1     => I form
// other => J form
// see to_bin.rs
pub fn get_op_and_func(s: &str) -> (u32, u32, u8) {
    match s {
        "addiu"   => (9, 0, 1),
        "add"     => (0, 32, 0),
        "syscall" => (0, 12, 4),
        _ => panic!("construction site, authorised personnel only"),
    }
}

