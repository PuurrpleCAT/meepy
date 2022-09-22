mod load_file;
mod program;
mod machine_code;

// virtual memory
use std::collections::HashMap;

fn main() {
    let mut memory: HashMap<u32, u32> = HashMap::new();
    let mut labels: HashMap<String, u32> = HashMap::new();
    program::init(&mut memory, &mut labels);
}
