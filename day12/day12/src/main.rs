use std::io::prelude::*;
use std::fs::File;
use std::collections::HashMap;
use std::collections::VecDeque;

fn findhash(v: &VecDeque<char>) -> i32 {
    let mut ct: i32 = 0;
    for i in 0..v.len() {
        if v[i] == '#' {
            break;
        }
        ct += 1;
    }
    ct
}

fn findend(v: &VecDeque<char>) -> i32 {
    let mut ct: i32 = 0;
    for i in v.len()..0 {
        if v[i] == '#' {
            break;
        }
        ct += 1;
    }
    ct
}

fn main() {
    let mut f = File::open("day11.txt").expect("Error");
    let mut content = String::new();
    f.read_to_string(&mut content).expect("Err");

    let mut inital: VecDeque<char> = VecDeque::new();
    let mut rules: HashMap<String, char> = HashMap::new(); 
    for line in content.split("\n") {
        let part: Vec<&str> = line.split_whitespace().collect();
        if part.len() == 0 {
            continue;
        }
        if part.len() == 2 {
            inital = part[1].chars().collect();
        }
        if part.len() == 3 {
            rules.insert(part[0].to_string(), part[2].chars().next().expect("parsing error"));
        }
    }
    let mut neg: i32 = 5;
    for _ in 0..5 {
        inital.push_front('.');
        inital.push_back('.');
    }
    for x in 0..20 {
    if x % 1000 == 0 {
        let mut ret: i32 = 0;
        for i in 0..inital.len() {
            if inital[i] == '#' {
                ret += i as i32 - neg;
            }
        }
        println!("Total: {}", ret);
    }
        let mut cpy = inital.clone();
        for index in 0..inital.len() - 5 {
            let mut s = String::new();
            for i in 0..5 {
                s.push(inital[i + index]);
            }
            cpy[index + 2] = *rules.get(&s).unwrap();
        }
        inital = cpy;
        if findhash(&inital) < 5 {
            neg += 1;
            inital.push_front('.');
        }
        if findend(&inital) < 5 {
            inital.push_back('.');
        }
    }
    let mut ret: i32 = 0;
    for i in 0..inital.len() {
        if inital[i] == '#' {
            ret += i as i32 - neg;
        }
    }
    println!("");
    println!("Total: {}", ret);
}
