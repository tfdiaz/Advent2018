use std::io::prelude::*;
use std::fs::File;

fn main() {
    let mut f = File::open("day2Input.txt").unwrap();
    let mut contents = String::new();
    let mut blocks: Vec<String> = Vec::new();

    f.read_to_string(&mut contents).unwrap();
    for line in contents.split('\n') {
        blocks.push(line.to_string());
    }
    let mut cdiff: usize = 0;
    let mut diff: u32 = 0;
    for l in blocks.clone() {
        for m in blocks.clone() {
            diff = 0;
            let mut liter = l.chars();
            let mut miter = m.chars();
            for i in 0..m.len() {
                if diff > 1 {
                    break;
                }
                if liter.next() != miter.next() {
                    diff += 1;
                    cdiff = i;
                }
            }
            if diff == 1 {
            println!("Line: {}, Line: {}, Remove: {}", l, m, cdiff);
            }
        }
    }
}
