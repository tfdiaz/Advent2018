use std::io::prelude::*;
use std::fs::File;
use std::collections::HashMap;
use std::collections::VecDeque;

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
    let mut buff: usize = 5;
    let mut neg: i32 = 5;
    for _ in 0..buff {
        inital.push_front('.');
        inital.push_back('.');
    }
    for _ in 0..20 {
        for index in 0..inital.len() - buff {
            let mut s = String::new();
            for i in 0..5 {
                s.push(inital[i + index]);
            }
            println!("String: {}", s);
            inital[index + 2] = *rules.get(&s).unwrap(); 
            if index + 2 < neg as usize && inital[index + 2] == '#' {
                neg += 1;
                inital.push_front('.');
            }
            if index + 2 > inital.len() - buff - 5 && inital[index + 2] == '#' {
                buff += 1;
                inital.push_back('.');
            }
        }
    }
    let mut ret: i32 = 0;
    for i in 0..inital.len() {
        if inital[i] == '#' {
            ret += i as i32 - neg;
        }
        print!("{}", inital[i]);
    }
    println!("");
    println!("Total: {}", ret);
}
