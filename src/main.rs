mod argonsysinfo;
use argonsysinfo::*;

fn main() {
    println!("CPU Temp: {}", argonsysinfo_getcputemp());
    println!("Mount: {}", argonsysinfo_getrootdev());
    argonsysinfo_getram();
}
