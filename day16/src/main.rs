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
    ret[c_dex] = ret[a_dex] * ret[b_dex];
    if ret == output {
        return 1;
    }
    0
}

fn muli(reg: [usize;4], input: [usize;4], output: [usize;4]) -> usize {
    let a_dex = input[1];
    let b_val = input[2];
    let c_dex = input[3];
    if a_dex > 3 || c_dex > 3 {
        return 0;
    }
    let mut ret = reg;
    ret[c_dex] = ret[a_dex] * b_val;
    if ret == output {
        return 1;
    }
    0
}

fn banr(reg: [usize;4], input: [usize;4], output: [usize;4]) -> usize {
    let a_dex = input[1];
    let b_dex = input[2];
    let c_dex = input[3];
    if a_dex > 3 || b_dex > 3 || c_dex > 3 {
        return 0;
    }
    let mut ret = reg;
    ret[c_dex] = ret[a_dex] & ret[b_dex];
    if ret == output {
        return 1;
    }
    0
}

fn bani(reg: [usize;4], input: [usize;4], output: [usize;4]) -> usize {
    let a_dex = input[1];
    let b_val = input[2];
    let c_dex = input[3];
    if a_dex > 3 || c_dex > 3 {
        return 0;
    }
    let mut ret = reg;
    ret[c_dex] = ret[a_dex] & b_val;
    if ret == output {
        return 1;
    }
    0
}

fn borr(reg: [usize;4], input: [usize;4], output: [usize;4]) -> usize {
    let a_dex = input[1];
    let b_dex = input[2];
    let c_dex = input[3];
    if a_dex > 3 || b_dex > 3 || c_dex > 3 {
        return 0;
    }
    let mut ret = reg;
    ret[c_dex] = ret[a_dex] | ret[b_dex];
    if ret == output {
        return 1;
    }
    0
}

fn bori(reg: [usize;4], input: [usize;4], output: [usize;4]) -> usize {
    let a_dex = input[1];
    let b_val = input[2];
    let c_dex = input[3];
    if a_dex > 3 || c_dex > 3 {
        return 0;
    }
    let mut ret = reg;
    ret[c_dex] = ret[a_dex] | b_val;
    if ret == output {
        return 1;
    }
    0
}

fn setr(reg: [usize;4], input: [usize;4], output: [usize;4]) -> usize {
    let a_dex = input[1];
    let c_dex = input[3];
    if a_dex > 3 || c_dex > 3 {
        return 0;
    }
    let mut ret = reg;
    ret[c_dex] = ret[a_dex];
    if ret == output {
        return 1;
    }
    0
}

fn seti(reg: [usize;4], input: [usize;4], output: [usize;4]) -> usize {
    let a_val = input[1];
    let c_dex = input[3];
    if c_dex > 3 {
        return 0;
    }
    let mut ret = reg;
    ret[c_dex] = a_val;
    if ret == output {
        return 1;
    }
    0
}

fn gtir(reg: [usize;4], input: [usize;4], output: [usize;4]) -> usize {
    let a_val = input[1];
    let b_dex = input[2];
    let c_dex = input[3];
    if b_dex > 3 || c_dex > 3 {
        return 0;
    }
    let mut ret = reg;
    if a_val > reg[b_dex] {
        ret[c_dex] = 1;
    }
    else {
        ret[c_dex] = 0;
    }
    if ret == output {
        return 1;
    }
    0
}

fn gtrr(reg: [usize;4], input: [usize;4], output: [usize;4]) -> usize {
    let a_dex = input[1];
    let b_dex = input[2];
    let c_dex = input[3];
    if a_dex > 3 || b_dex > 3 || c_dex > 3 {
        return 0;
    }
    let mut ret = reg;
    if reg[a_dex] > reg[b_dex] {
        ret[c_dex] = 1;
    }
    else {
        ret[c_dex] = 0;
    }
    if ret == output {
        return 1;
    }
    0
}

fn gtri(reg: [usize;4], input: [usize;4], output: [usize;4]) -> usize {
    let a_dex = input[1];
    let b_val = input[2];
    let c_dex = input[3];
    if a_dex > 3 || c_dex > 3 {
        return 0;
    }
    let mut ret = reg;
    if reg[a_dex] > b_val {
        ret[c_dex] = 1;
    }
    else {
        ret[c_dex] = 0;
    }
    if ret == output {
        return 1;
    }
    0
}

fn eqir(reg: [usize;4], input: [usize;4], output: [usize;4]) -> usize {
    let a_val = input[1];
    let b_dex = input[2];
    let c_dex = input[3];
    if b_dex > 3 || c_dex > 3 {
        return 0;
    }
    let mut ret = reg;
    if a_val == reg[b_dex] {
        ret[c_dex] = 1;
    }
    else {
        ret[c_dex] = 0;
    }
    if ret == output {
        return 1;
    }
    0
}

fn eqri(reg: [usize;4], input: [usize;4], output: [usize;4]) -> usize {
    let a_dex = input[1];
    let b_val = input[2];
    let c_dex = input[3];
    if a_dex > 3 || c_dex > 3 {
        return 0;
    }
    let mut ret = reg;
    if reg[a_dex] == b_val {
        ret[c_dex] = 1;
    }
    else {
        ret[c_dex] = 0;
    }
    if ret == output {
        return 1;
    }
    0
}

fn eqrr(reg: [usize;4], input: [usize;4], output: [usize;4]) -> usize {
    let a_dex = input[1];
    let b_dex = input[2];
    let c_dex = input[3];
    if a_dex > 3 || b_dex > 3 || c_dex > 3 {
        return 0;
    }
    let mut ret = reg;
    if reg[a_dex] == reg[b_dex] {
        ret[c_dex] = 1;
    }
    else {
        ret[c_dex] = 0;
    }
    if ret == output {
        return 1;
    }
    0
}

fn findops(reg: [usize;4], input: [usize;4], output: [usize;4],
        args: &Vec<&Fn([usize;4], [usize;4], [usize;4]) -> usize>, table: &mut[[bool;16];16]) -> usize {
    let mut ct: usize = 0;
    for i in 0..16 {
        ct += (args[i])(reg, input, output);
    }
    if ct >= 3 {
        return 1;
    }
    0
}

fn reduce(table: [[bool;16]16]) -> [usize;16] {
    
}

fn main() {
    let mut f = File::open("inputday16.txt").expect("Error");
    let mut content = String::new();
    f.read_to_string(&mut content).expect("Error in read");
    let mut switch = 0;
    let mut reg: [usize;4] = [0;4];
    let mut input: [usize;4] = [0;4];
    let mut output: [usize;4] = [0;4];
    let mut args: Vec<&Fn([usize;4], [usize;4], [usize;4]) -> usize> = Vec::new();
    let mut tot: usize = 0;
    let mut table: [[bool;16]; 16] = [[false;16];16];
    args = vec![&addr, &addi, &mulr, &muli, &banr, &bani, &borr, &bori, &setr, &seti, &gtir,
                 &gtri, &gtrr, &eqir, &eqri, &eqrr];

    for line in content.split("\n") {
        if switch % 4 == 0 {
            let cont: Vec<&str> = line.split(' ').collect();
            if cont.len() != 5 {
                break;
            }
            reg[0] = cont[1].trim_start_matches('[')
                    .trim_end_matches(',')
                    .parse::<usize>().expect("1");
            reg[1] = cont[2].trim_end_matches(',')
                    .parse::<usize>().expect("2");
            reg[2] = cont[3].trim_end_matches(',')
                    .parse::<usize>().expect("3");
            reg[3] = cont[4].trim_end_matches(']')
                    .parse::<usize>().expect("4");
        }
        else if switch % 4 == 1 {
            let cont: Vec<&str> = line.split(' ').collect();
            for i in 0..cont.len() {
                input[i] = cont[i].parse::<usize>().expect("5");
            }
        }
        else if switch % 4 == 2 {
            let cont: Vec<&str> = line.split_whitespace().collect();
            output[0] = cont[1].trim_start_matches('[')
                    .trim_end_matches(',')
                    .parse::<usize>().expect("6");
            output[1] = cont[2].trim_end_matches(',')
                    .parse::<usize>().expect("7");
            output[2] = cont[3].trim_end_matches(',')
                    .parse::<usize>().expect("8");
            output[3] = cont[4].trim_end_matches(']')
                    .parse::<usize>().expect("9");
            tot += findops(reg, input, output, &args, &mut table);
        }
        switch += 1;
    }
    let index: [usize:16] = reduce(table);

    println!("Tot: {}", tot);
}
