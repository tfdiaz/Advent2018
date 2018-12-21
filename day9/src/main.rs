use std::collections::VecDeque;

fn rotate(v: &mut VecDeque<usize>) -> () {
    for _ in 0..2 {
        let tmp = v.pop_front().unwrap();
        v.push_back(tmp);
    }
}

fn revrotate(v: &mut VecDeque<usize>) -> () {
    for _ in 0..7 {
        let tmp = v.pop_back().unwrap();
        v.push_front(tmp);
    }
}

fn main() {
    let mut v: VecDeque<usize> = VecDeque::new();
    v.push_front(0);
    let mut ret = [0usize;459];
    let mut player: usize = 0;
    for i in 1..7132001 {
        if i % 23 == 0 {
            ret[player] += i;
            revrotate(&mut v);
            ret[player] += v.pop_front().unwrap();
        }
        else {
            rotate(&mut v);
            v.push_front(i);
        }
        player = (player + 1) % 459;
    }
    let mut max = [0usize;2];
    for i in 0..ret.len() {
        if ret[i] > max[0] {
            max[0] = ret[i];
            max[1] = i;
        }
    }
    println!("Player: {}, Score: {}", max[1] + 1, max[0]);
}
