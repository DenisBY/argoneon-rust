mod argonsysinfo;
use argonsysinfo::*;

#[allow(dead_code, unused_imports, unused_mut, unused_variables)]
fn main() {
    println!("CPU Temp: {}", argonsysinfo_getcputemp());
    println!("Mount: {}", argonsysinfo_getrootdev());
    let mut memory = argonsysinfo_getram();
    memory.title = (memory.title).to_lowercase();
    println!("Memory: {}", serde_json::to_string_pretty(&memory).unwrap());
}
