use std::fs::File;
use std::io::prelude::*;

fn addr(reg: [usize;4], input: [usize;4], output: [usize;4]) -> usize {
    let a_dex = input[1];
    let b_dex = input[2];
    let c_dex = input[3];
    if a_dex > 3 || b_dex > 3 || c_dex > 3 {
        return 0;
    }
    let mut ret = reg;
    ret[c_dex] = ret[a_dex] + ret[b_dex];
    if ret == output {
        return 1;
    }
    0
}

fn addi(reg: [usize;4], input: [usize;4], output: [usize;4]) -> usize {
    let a_dex = input[1];
    let b_val = input[2];
    let c_dex = input[3];
    if a_dex > 3 || c_dex > 3 {
        return 0;
    }
    let mut ret = reg;
    ret[c_dex] = ret[a_dex] + b_val;
    if ret == output {
        return 1;
    }
    0
}

fn mulr(reg: [usize;4], input: [usize;4], output: [usize;4]) -> usize {
    let a_dex = input[1];
    let b_dex = input[2];
    let c_dex = input[3];
    if a_dex > 3 || b_dex > 3 || c_dex > 3 {
        return 0;
    }
    let mut ret = reg;
    ret[c_dex] = ret[a_dex] + ret[b_dex];
    if ret == output {
        return 1;
    }
    0
}


fn findops(reg: [usize;4], input: [usize;4], output: [usize;4],
        &args: Vec<Fn([usize;4], [usize;4], [usize;4]) -> usize>) -> usize {
    let mut ct: usize = 0;
    for i in 0..16 {
        ct += *(args[i])(reg, input, output);
    }
    if ct >= 3 {
        return 1;
    }
    0
}

fn main() {
    let mut f = File::open("inputday16.txt").expect("Error");
    let mut content = String::new();
    f.read_to_string(&mut content).expect("Error in read");
    let mut switch = 0;
    let mut reg: [usize;4];
    let mut input: [usize;4];
    let mut output: [usize;4];
    let mut output: usize = 0;
    let mut args: Vec<Fn([usize;4], [usize;4], [usize;4]) -> usize> = Vec::new();
    args = vec![addr, addi, mulr, muli, banr, bani, borr, bori, setr, seti, gtir, gtri, gtrr
        eqir, eqri, eqrr];

    for line in content.split('\n') {
        if switch % 3 == 0 {
            let cont = line.split(' ').collect();
            if cont.len() != 5 {
                break;
            }
            reg[0] = cont[1].trim_start_matches('['])
                    .trim_end_matches(',')
                    .parse::<usize>().unwrap();
            reg[1] = cont[2].trim_end_matches(',')
                    .parse::<usize>().unwrap());
            reg[2] = cont[3].trim_end_matches(',')
                    .parse::<usize>().unwrap());
            reg[3] = cont[4].trim_end_matches(']')
                    .parse::<usize>().unwrap());
        }
        else if switch % 3 == 1 {
            let cont = line.split(' ').collect();
            for i in 0..cont.len() {
                input[i] = cont[i].parse::<usize>().unwrap();
            }
        }
        else {
            output[0] = cont[1].trim_start_matches('['])
                    .trim_end_matches(',')
                    .parse::<usize>().unwrap();
            output[1] = cont[2].trim_end_matches(',')
                    .parse::<usize>().unwrap();
            output[2] = cont[3].trim_end_matches(',')
                    .parse::<usize>().unwrap();
            output[3] = cont[4].trim_end_matches(']')
                    .parse::<usize>().unwrap();
            tot += findops(reg, input, output);
        }
        switch += 1;
    }
    println!("Tot: {}", tot);
}
