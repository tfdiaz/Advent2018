fn cal(num: usize, val: usize) -> usize {
    if val < 100000 {
        val * 10 + num
    }
    else {
        (val % 100000) * 10 + num
    }
}

fn main() {
    let mut elOne: usize = 0;
    let mut elTwo: usize = 1;
    let mut scores: Vec<usize> = Vec::new();
    let input = 652601;

    scores.push(3);
    scores.push(7);
    let mut tot = 37;
    let mut ct: usize = 2;
    while input != tot {
        let val = scores[elOne] + scores[elTwo];
        if val > 9 {
            scores.push(val / 10);
            tot = cal(val / 10, tot);
            if tot == input {
                break;
            }
            scores.push(val % 10);
            tot = cal(val % 10, tot);
            ct += 2;
        }
        else {
            scores.push(val);
            tot = cal(val, tot);
            ct += 1;
        }
        elOne = (1 + scores[elOne] + elOne) % scores.len();
        elTwo = (1 + scores[elTwo] + elTwo) % scores.len();
    }
    println!("Left Num: {} ", ct - 5);
}
