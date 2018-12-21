
use std::io::prelude::*;
use std::fs::File;
use std::collections::VecDeque;

fn main() {
    let mut f = File::open("inputday5.txt").expect("Error in file");
    let mut content = String::new();
    f.read_to_string(&mut content).expect("Eror");
    let mut buf: VecDeque<char> = content.chars().collect();
    let mut c: char = buf.pop_front().expect("SHould be good");
    let mut past: usize = 0;
    while buf.len() != past {
        past = buf.len();
        for _ in 0..buf.len() {
            let d: char = buf.pop_front().expect("Should to good");
            if c.eq_ignore_ascii_case(&d) && !(c == d) {
                c = buf.pop_front().expect("Null");
            }
            else {
                buf.push_back(c);
                c = d;
            }
        }
    }
    let tmp_str = "abcdefghijklmnopqrstuvwxyz";
    println!("First buff len: {}", buf.len());
    let mut min: usize = buf.len();
    for a in tmp_str.chars() {
        let mut cpy = buf.clone();
        for _ in 0..buf.len() {
            if a.eq_ignore_ascii_case(cpy.front().expect("Get Error")) {
                cpy.pop_front().expect("Error");
            }
            else {
                let d = cpy.pop_front().expect("Error");
                cpy.push_back(d);
            }
        }
        println!("Buff Len: {}", cpy.len());
        past = 0;
        c = cpy.pop_front().expect("C pop in 42");
        while cpy.len() != past {
            past = cpy.len();
            for _ in 0..cpy.len() {
                let d: char = cpy.pop_front().expect("C pop in 44");
                if c.eq_ignore_ascii_case(&d) && !(c == d) {
                    c = cpy.pop_front().expect("Null");
                }
                else {
                    cpy.push_back(c);
                    c = d;
                }
            }
        }
        println!("Letter: {} Size: {}", a, cpy.len());
            if cpy.len() < min {
                min = cpy.len();
            }
    }

    println!("Total: {} ", min);
}
