#![allow(unused)]

use serde::{Deserialize, Serialize};
use std::fs::*;
use std::io::{self, BufRead};
use std::path::Path;

#[derive(Serialize, Deserialize, Debug)]
pub enum Values {
    String(Vec<String>),
    Num(Vec<usize>),
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Response {
    pub title: String,
    pub headers: Vec<String>,
    pub values: Values,
}

impl Response {
    pub fn lowercase(&mut self) -> &Response {
        self.title = (self.title).to_lowercase();
        for header in &mut self.headers {
            *header = header.to_lowercase();
        }
        self
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

pub fn argonsysinfo_getcputemp() -> u32 {
    let contents: u32 = read_to_string("/sys/class/thermal/thermal_zone0/temp")
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

pub fn argonsysinfo_getram() -> Response {
    let lines = read_lines("/proc/meminfo");
    let mut totalram = 0;
    let mut totalfree = 0;
    let mut percent: f64 = 0.0;
    // let mut result = HashMap::new();
    for line in lines.unwrap() {
        // println!("{}", line.unwrap());
        let splitted: Vec<&str>;
        let data = line.unwrap();
        splitted = data.split_whitespace().collect();
        match splitted[0] {
            "MemTotal:" => totalram = splitted[1].parse().unwrap(),
            "MemFree:" => {
                let memfree: u32 = splitted[1].parse().unwrap();
                totalfree = totalfree + memfree;
            }
            "Buffers:" => {
                let memfree: u32 = splitted[1].parse().unwrap();
                totalfree = totalfree + memfree;
            }
            "Cached:" => {
                let memfree: u32 = splitted[1].parse().unwrap();
                totalfree = totalfree + memfree;
            }
            _ => continue,
        }
        percent = (totalfree as f64 / totalram as f64) * 100.0;
    }
    let headers = [
        "Total".to_string(),
        "Free".to_string(),
        "Percent".to_string(),
    ];
    let values = [
        totalram.to_string(),
        totalfree.to_string(),
        format!("{:.5}", percent.to_string()),
    ];
    Response {
        title: "Memory".to_string(),
        headers: headers.to_vec(),
        values: Values::String(values.to_vec()),
    }
}
