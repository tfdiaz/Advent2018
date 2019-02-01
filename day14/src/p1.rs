fn main() {
    let mut elOne: usize = 0;
    let mut elTwo: usize = 1;
    let mut scores: Vec<usize> = Vec::new();
    let input = 652601;

    scores.push(3);
    scores.push(7);
    let mut ct: usize = 2;
    while ct < (input + 10) {
        let val = scores[elOne] + scores[elTwo];
        // println!("Score: {}, Index {}, Score: {}, Index {}", scores[elOne],
        //     elOne, scores[elTwo], elTwo);
        if val > 9 {
            scores.push(val / 10);
            scores.push(val % 10);
            ct += 2;
        }
        else {
            scores.push(val);
            ct += 1;
        }
        elOne = (1 + scores[elOne] + elOne) % scores.len();
        elTwo = (1 + scores[elTwo] + elTwo) % scores.len();
        // for x in 0..scores.len() {
        //     print!(" {} ", scores[x]);
        // }
        // println!("");
    }
    for x in input..(input + 10) {
        print!("{}", scores[x]);
    }
    println!("");
}
