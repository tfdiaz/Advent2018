use std::io::prelude::*;
use std::fs::File;

fn adder(iter: &mut std::iter::Iterator<Item=usize>) -> usize {
    let mut ret: usize = 0;
    let mut nodes = iter.next().unwrap();
    let mut metadata: usize = iter.next().unwrap();
    while nodes > 0 {
        ret += adder(iter);
        nodes -= 1;
    }
    while metadata > 0 {
        ret += iter.next().unwrap();
        metadata -= 1;
    }
    ret
}

fn main() {
    let mut f = File::open("inputday8.txt").unwrap();
    let mut contents = String::new();
    f.read_to_string(&mut contents).unwrap();
    let mut iter = contents
        .split_whitespace()
        .map(|x| x.parse::<usize>()
        .unwrap());
    let ret = adder(&mut iter);
    println!("Total Meta: {}", ret);
}
