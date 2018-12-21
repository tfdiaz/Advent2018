use std::io::prelude::*;
use std::fs::File;
use std::collections::HashSet;

fn main() {
    let mut f = File::open("day3input.txt").expect("Error");
    let mut contents = String::new();
    let mut grid = [[0i32; 1000]; 1000];
    let mut symb: i32 = 0;
    let mut xdex: usize = 0;
    let mut ydex: usize = 0;
    let mut xsize: usize = 0;
    let mut ysize: usize = 0;
    let mut count: usize = 0;
    let mut mh: HashSet<i32> = HashSet::new();

    f.read_to_string(&mut contents).expect("OO");
    for line in contents.split('\n') {
        let v: Vec<&str> = line.split(' ').collect();
        if v.len() != 4 {
            break;
        }
        for i in 0..v.len() {
            if i == 0 {
                symb = v[i].trim_start_matches('#').parse::<i32>().expect("Symb");
            }
            if i == 2 {
                let j: Vec<&str> = v[i].split(',').collect();
                for i in 0..j.len() {
                    if i == 0 {
                        xdex = j[i].parse::<usize>().expect("xdex");
                    }
                    if i == 1 {
                        ydex = j[i].trim_end_matches(':').parse::<usize>().expect("ydex");
                    }
                }
            }
            if i == 3 {
                let j: Vec<&str> = v[i].split('x').collect();
                for i in 0..j.len() {
                    if i == 0 {
                        xsize = j[i].parse::<usize>().expect("xsize");
                    }
                    if i == 1 {
                        ysize = j[i].parse::<usize>().expect("ysize");
                    }
                }
            }
        }
        mh.insert(symb);
        for i in 0..ysize {
            for x in 0..xsize {
                if grid[ydex + i][xdex + x] == 0 {
                    grid[ydex + i][xdex + x] = symb;
                }
                else if grid[ydex + i][xdex + x] != -1 {
                    mh.remove(&grid[ydex + i][xdex + x]);
                    mh.remove(&symb);
                    grid[ydex + i][xdex + x] = -1;
                    count += 1;
                }
                else {
                    mh.remove(&symb);
                }
            }
        }
    }
    for i in &mh {
        println!("Nums: {}", i);
    }
}
