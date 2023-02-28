mod argonsysinfo;
use argonsysinfo::*;

fn main() {
    println!("CPU Temp: {}", argonsysinfo_getcputemp());
    println!("Mount: {}", argonsysinfo_getrootdev());
    let memory = serde_json::to_string_pretty(&argonsysinfo_getram()).unwrap();
    println!("Memory: {}", memory);
}
