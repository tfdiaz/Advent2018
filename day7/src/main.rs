use std::io::prelude::*;
use std::fs::File;
use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;

#[derive(Default)]
struct Factory<'a> {
    buds: HashMap<&'a str, u8>,
    totalTime: usize
}

impl<'a> Factory<'a> { 
    fn workit(& mut self, buf: & mut VecDeque<&'a str>) -> () {
    let mut flag: bool = true;
    while flag {
        if self.buds.len() > 0 {
            for (key, val) in self.buds.iter_mut() {
                *val -= 1;
                print!("{}", key);
            }
            println!("");
            self.totalTime += 1;
            let zeros: Vec<_> = self.buds.iter().filter(|&(_, &v)| v == 0).map(|(k, _)| k.clone()).collect();
            for zero in zeros {
                self.buds.remove(zero);
                buf.push_back(zero);
                flag = false;
                println!("Do I get here");
            }
        }
    }
}
    fn setit(& mut self, s: &'a str) -> bool {
    if  self.buds.len() < 5 {
            self.buds.insert(s, s.as_bytes()[0] - &"A".as_bytes()[0] + 61);
            return true;
        }
        false
    }
}

fn main() {
    let mut f = File::open("day7input.txt").expect("Error in opening");
    let mut content = String::new();
    let mut fact: Factory = Default::default();
    f.read_to_string(&mut content).expect("Error in read");
    let mut incoming: HashMap<&str, HashSet<&str>> = HashMap::new();
    let mut outgoing: HashMap<&str, HashSet<&str>> = HashMap::new();
    for line in content.split("\n") {
        let mut v: Vec<&str> = line.split(" ").collect();
        if v.len() < 5 {
            break;
        }
        outgoing.entry(v[1]).or_insert(HashSet::new()).insert(v[7]);
        incoming.entry(v[7]).or_insert(HashSet::new()).insert(v[1]);
        incoming.entry(v[1]).or_insert(HashSet::new());
        outgoing.entry(v[7]).or_insert(HashSet::new());
    }
    let mut ret:Vec<&str> = Vec::new();
    let mut buf:VecDeque<&str> = VecDeque::new();
    let mut alpha: &str = &"";
    while incoming.len() > 0 {
        let mut keys: Vec<&str> = Vec::new();
        for (key, edges) in incoming.iter() {
            if edges.len() == 0 {
                if !fact.setit(*key) {
                    break;
                }
                keys.push(key);
            }
        }
        for key in keys.iter() {
            incoming.remove(key);
        }
        keys.clear();
        fact.workit(&mut buf);
        while buf.len() > 0 {
            let alpha = buf.pop_front().unwrap();
            ret.push(alpha);
            let edges = outgoing.get(alpha).unwrap();
            for node in edges.iter() {
                let sides = incoming.get_mut(node).unwrap();
                sides.remove(alpha);
            }
        }
    }
    for elm in ret.iter() {
        print!("{}", elm);
    }
    println!("");
    println!("Total Time: {}", fact.totalTime);
}
