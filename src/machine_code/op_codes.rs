// 0     => R form
// 1     => I form
// other => J form
// see to_bin.rs
pub fn get_op_and_func(s: &str) -> (u32, Option<u32>, u8) {
    match s {
        "addiu"   => (9, None, 1),
        "syscall" => (0, None, 4),
        _ => panic!("construction site, authorised personnel only"),
    }
}

