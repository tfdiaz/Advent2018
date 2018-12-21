use std::io::prelude::*;
use std::fs::File;
use std::collections::HashMap;

fn main() {
    let mut f = File::open("inputDay4.txt").expect("Error in file");
    let mut content = String::new();
    f.read_to_string(&mut content).unwrap();
    let mut sdx: usize = 0;
    let mut edx: usize = 0;
    let mut gid: usize = 0;
    let mut map: HashMap<usize, Vec<[bool;60]> > = HashMap::new();
    let mut bvec = [false;60];
    for line in content.split('\n') {
        let v: Vec<&str> = line.split(' ').collect();
        if v.len() < 4 {
            break;
        }
        if v[3].contains('#') {
            if gid != 0 {
                map.entry(gid).or_insert(Vec::new()).push(bvec);
                bvec = [false;60];
            }
            gid = v[3].trim_start_matches('#').parse::<usize>().expect("Guard parse");
        }
        if v[2].contains("falls") {
            let sv: Vec<&str> = v[1].split(':').collect();
            sdx = sv[1].trim_end_matches(']').parse::<usize>().expect("Error in sdx");
            bvec[sdx] = true;
        }
        if v[2].contains("wakes") {
            let ev: Vec<&str> = v[1].split(':').collect();
            edx = ev[1].trim_end_matches(']').parse::<usize>().expect("Error in edx");
            for i in sdx..edx {
                bvec[i] = true;
            }
        }
    }
    map.entry(gid).or_insert(Vec::new()).push(bvec);
    let mut max = [0usize; 3];
    let mut ct: usize = 0;
    let mut minct = [0usize; 60];
    for (guard, shift) in map.iter() {
        ct = 0;
        minct = [0;60];
        for one in shift.iter() {
            for i in 0..one.len() {
                if one[i] == true {
                    minct[i] += 1;
                }
            }
        }
        if *minct.iter().max().unwrap() > max[2] {
            max[0] = *guard;
            max[1] = (0..minct.len()).max_by_key(|&x| minct[x]).unwrap();
            max[2] = *minct.iter().max().unwrap();
        }
    }
    println!("Total checksum: {} ", max[0] * max[1]);
}
