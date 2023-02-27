use std::fs::*;
use std::io::{self, BufRead};
use std::path::Path;

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

pub fn argonsysinfo_getcputemp() -> i32 {
    let contents: i32 = read_to_string("/sys/class/thermal/thermal_zone0/temp")
        .unwrap()
        .trim() // remove \n
        .parse() // convert to int
        .unwrap();
    contents / 1000
}

pub fn argonsysinfo_getrootdev() -> String {
    if let Ok(lines) = read_lines("/proc/mounts") {
        for line in lines {
            let mpoint: Vec<&str>;
            if let Ok(points) = line {
                mpoint = points.split_whitespace().collect();
                if mpoint[1] == "/" {
                    return mpoint[0].to_string();
                }
            }
        }
    }
    return "".to_string();
}

// pub fn argonsysinfo_listhddusage() {
//     println!();
// }

// pub fn argonsysinfo_listraid() {
//     if let Ok(lines) = read_lines("/tmp/mdstat") {
//         for line in lines {
//             if let Ok(data) = line {
//                 println!("{data}");
//             }
//         }
//     }
// }

pub fn argonsysinfo_getram() {
    if let Ok(lines) = read_lines("/proc/meminfo") {
        let mut totalram = 0;
        let mut totalfree = 0;
        for line in lines {
            let splitted: Vec<&str>;
            if let Ok(data) = line {
                splitted = data.split_whitespace().collect();
                match splitted[0] {
                    "MemTotal:" => totalram = splitted[1].parse().unwrap(),
                    "MemFree:" => {
                        let memfree: i32 = splitted[1].parse().unwrap();
                        totalfree = totalfree + memfree;
                    }
                    "Buffers:" => {
                        let memfree: i32 = splitted[1].parse().unwrap();
                        totalfree = totalfree + memfree;
                    }
                    "Cached:" => {
                        let memfree: i32 = splitted[1].parse().unwrap();
                        totalfree = totalfree + memfree;
                    }
                    _ => continue,
                }
            }
        }
        let percent: f64 = (totalfree as f64 / totalram as f64) * 100.0;
        println!("{:.2}", percent);
    }
}
