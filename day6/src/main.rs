use std::io::prelude::*;
use std::fs::File;

fn main() {
    let mut f = File::open("day6.txt").expect("Error");
    let mut content = String::new();
    f.read_to_string(&mut content).expect("Error in read");
    let mut points: Vec<[usize; 2]> = Vec::new();
    let mut total:usize = 0;
    for line in content.split('\n') {
        let v: Vec<&str> = line.split(", ").collect();
        if v.len() < 2 {
            break;
        }
        let mut tmp = [0usize; 2];
        tmp[0] = v[0].parse::<usize>().expect("V0 parse error");
        tmp[1] = v[1].parse::<usize>().expect("V1 parse error");
        points.push(tmp);
    }
    for y in 0..400 {
        for x in 0..400 {
            let mut mindest: usize = 800;
            let mut min = [0usize; 2];
            let mut dest: usize = 0;
            for point in points.iter() {
                let mut ds_x: usize = 0;
                let mut ds_y: usize = 0;
                if point[0] > x {
                    ds_x = point[0] - x;
                }
                else {
                    ds_x = x - point[0];
                }
                if point[1] > y {
                    ds_y = point[1] - y;
                }
                else {
                    ds_y = y - point[1];
                }
                dest += ds_x + ds_y;
            }
            if dest < 10000 {
                total += 1;
            }
        }
    }
    println!("Total inner max {} ", total);
}
