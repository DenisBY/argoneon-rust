#![allow(unused)]

mod argonsysinfo;
use argonsysinfo::*;

fn main() {
    // println!("CPU Temp: {}", argonsysinfo_getcputemp());
    // println!("Mount: {}", argonsysinfo_getrootdev());
    // let memory = argonsysinfo_getram().lowercase();
    // memory.title = (memory.title).to_lowercase();
    // println!("Memory: {}", serde_json::to_string_pretty(&memory).unwrap());
    println!(
        "{}",
        serde_json::to_string_pretty(argonsysinfo_getram().lowercase()).unwrap()
    );
}
