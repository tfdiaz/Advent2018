use std::io::prelude::*;
use std::fs::File;

fn adder(iter: &mut std::iter::Iterator<Item=usize>) -> (usize) {
    let mut v: Vec<usize> = Vec::new();
    let nodes = iter.next().unwrap();
    let mut metadata: usize = iter.next().unwrap();
    let mut ret: usize = 0;
    for _ in 0..nodes {
        v.push(adder(iter));
    }
    while metadata > 0 {
        let index = iter.next().unwrap();
        if index <= nodes && index > 0 {
            ret += v[index - 1];
        }
        else if nodes == 0 {
            ret += index;
        }
        metadata -= 1;
    }
    (ret)
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