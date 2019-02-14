use std::fs::File;
use std::io::prelude::*;

fn addr(reg: [usize;4], input: [usize;4], output: [usize;4]) -> (usize, [usize;4]) {
    let a_dex = input[1];
    let b_dex = input[2];
    let c_dex = input[3];
    if a_dex > 3 || b_dex > 3 || c_dex > 3 {
        return (0, reg);
    }
    let mut ret = reg;
    ret[c_dex] = ret[a_dex] + ret[b_dex];
    if ret == output {
        return (1, ret);
    }
    (0, ret)
}

fn addi(reg: [usize;4], input: [usize;4], output: [usize;4]) -> (usize, [usize;4]) {
    let a_dex = input[1];
    let b_val = input[2];
    let c_dex = input[3];
    if a_dex > 3 || c_dex > 3 {
        return (0, reg);
    }
    let mut ret = reg;
    ret[c_dex] = ret[a_dex] + b_val;
    if ret == output {
        return (1, ret);
    }
    (0, ret)
}

fn mulr(reg: [usize;4], input: [usize;4], output: [usize;4]) -> (usize, [usize;4]) {
    let a_dex = input[1];
    let b_dex = input[2];
    let c_dex = input[3];
    if a_dex > 3 || b_dex > 3 || c_dex > 3 {
        return (0, reg);
    }
    let mut ret = reg;
    ret[c_dex] = ret[a_dex] * ret[b_dex];
    if ret == output {
        return (1, ret);
    }
    (0, ret)
}

fn muli(reg: [usize;4], input: [usize;4], output: [usize;4]) -> (usize, [usize;4]) {
    let a_dex = input[1];
    let b_val = input[2];
    let c_dex = input[3];
    if a_dex > 3 || c_dex > 3 {
        return (0, reg);
    }
    let mut ret = reg;
    ret[c_dex] = ret[a_dex] * b_val;
    if ret == output {
        return (1, ret);
    }
    (0, ret)
}

fn banr(reg: [usize;4], input: [usize;4], output: [usize;4]) -> (usize, [usize;4]) {
    let a_dex = input[1];
    let b_dex = input[2];
    let c_dex = input[3];
    if a_dex > 3 || b_dex > 3 || c_dex > 3 {
        return (0, reg);
    }
    let mut ret = reg;
    ret[c_dex] = ret[a_dex] & ret[b_dex];
    if ret == output {
        return (1, ret);
    }
    (0, ret)
}

fn bani(reg: [usize;4], input: [usize;4], output: [usize;4]) -> (usize, [usize;4]) {
    let a_dex = input[1];
    let b_val = input[2];
    let c_dex = input[3];
    if a_dex > 3 || c_dex > 3 {
        return (0, reg);
    }
    let mut ret = reg;
    ret[c_dex] = ret[a_dex] & b_val;
    if ret == output {
        return (1, ret);
    }
    (0, ret)
}

fn borr(reg: [usize;4], input: [usize;4], output: [usize;4]) -> (usize, [usize;4]) {
    let a_dex = input[1];
    let b_dex = input[2];
    let c_dex = input[3];
    if a_dex > 3 || b_dex > 3 || c_dex > 3 {
        return (0, reg);
    }
    let mut ret = reg;
    ret[c_dex] = ret[a_dex] | ret[b_dex];
    if ret == output {
        return (1, ret);
    }
    (0, ret)
}

fn bori(reg: [usize;4], input: [usize;4], output: [usize;4]) -> (usize, [usize;4]) {
    let a_dex = input[1];
    let b_val = input[2];
    let c_dex = input[3];
    if a_dex > 3 || c_dex > 3 {
        return (0, reg);
    }
    let mut ret = reg;
    ret[c_dex] = ret[a_dex] | b_val;
    if ret == output {
        return (1, ret);
    }
    (0, ret)
}

fn setr(reg: [usize;4], input: [usize;4], output: [usize;4]) -> (usize, [usize;4]) {
    let a_dex = input[1];
    let c_dex = input[3];
    if a_dex > 3 || c_dex > 3 {
        return (0, reg);
    }
    let mut ret = reg;
    ret[c_dex] = ret[a_dex];
    if ret == output {
        return (1, ret);
    }
    (0, ret)
}

fn seti(reg: [usize;4], input: [usize;4], output: [usize;4]) -> (usize, [usize;4]) {
    let a_val = input[1];
    let c_dex = input[3];
    if c_dex > 3 {
        return (0, reg);
    }
    let mut ret = reg;
    ret[c_dex] = a_val;
    if ret == output {
        return (1, ret);
    }
    (0, ret)
}

fn gtir(reg: [usize;4], input: [usize;4], output: [usize;4]) -> (usize, [usize;4]) {
    let a_val = input[1];
    let b_dex = input[2];
    let c_dex = input[3];
    if b_dex > 3 || c_dex > 3 {
        return (0, reg);
    }
    let mut ret = reg;
    if a_val > reg[b_dex] {
        ret[c_dex] = 1;
    }
    else {
        ret[c_dex] = 0;
    }
    if ret == output {
        return (1, ret);
    }
    (0, ret)
}

fn gtrr(reg: [usize;4], input: [usize;4], output: [usize;4]) -> (usize, [usize;4]) {
    let a_dex = input[1];
    let b_dex = input[2];
    let c_dex = input[3];
    if a_dex > 3 || b_dex > 3 || c_dex > 3 {
        return (0, reg);
    }
    let mut ret = reg;
    if reg[a_dex] > reg[b_dex] {
        ret[c_dex] = 1;
    }
    else {
        ret[c_dex] = 0;
    }
    if ret == output {
        return (1, ret);
    }
    (0, ret)
}

fn gtri(reg: [usize;4], input: [usize;4], output: [usize;4]) -> (usize, [usize;4]) {
    let a_dex = input[1];
    let b_val = input[2];
    let c_dex = input[3];
    if a_dex > 3 || c_dex > 3 {
        return (0, reg);
    }
    let mut ret = reg;
    if reg[a_dex] > b_val {
        ret[c_dex] = 1;
    }
    else {
        ret[c_dex] = 0;
    }
    if ret == output {
        return (1, ret);
    }
    (0, ret)
}

fn eqir(reg: [usize;4], input: [usize;4], output: [usize;4]) -> (usize, [usize;4]) {
    let a_val = input[1];
    let b_dex = input[2];
    let c_dex = input[3];
    if b_dex > 3 || c_dex > 3 {
        return (0, reg);
    }
    let mut ret = reg;
    if a_val == reg[b_dex] {
        ret[c_dex] = 1;
    }
    else {
        ret[c_dex] = 0;
    }
    if ret == output {
        return (1, ret);
    }
    (0, ret)
}

fn eqri(reg: [usize;4], input: [usize;4], output: [usize;4]) -> (usize, [usize;4]) {
    let a_dex = input[1];
    let b_val = input[2];
    let c_dex = input[3];
    if a_dex > 3 || c_dex > 3 {
        return (0, reg);
    }
    let mut ret = reg;
    if reg[a_dex] == b_val {
        ret[c_dex] = 1;
    }
    else {
        ret[c_dex] = 0;
    }
    if ret == output {
        return (1, ret);
    }
    (0, ret)
}

fn eqrr(reg: [usize;4], input: [usize;4], output: [usize;4]) -> (usize, [usize;4]) {
    let a_dex = input[1];
    let b_dex = input[2];
    let c_dex = input[3];
    if a_dex > 3 || b_dex > 3 || c_dex > 3 {
        return (0, reg);
    }
    let mut ret = reg;
    if reg[a_dex] == reg[b_dex] {
        ret[c_dex] = 1;
    }
    else {
        ret[c_dex] = 0;
    }
    if ret == output {
        return (1, ret);
    }
    (0, ret)
}

fn findops(reg: [usize;4], input: [usize;4], output: [usize;4],
        args: &Vec<&Fn([usize;4], [usize;4], [usize;4]) -> (usize, [usize;4])>, table: &mut[[bool;16];16]) -> usize {
    let mut ct: usize = 0;
    for i in 0..16 {
        let (ret, dummy) = (args[i])(reg, input, output);
        if ret == 1 {
            table[i][input[0]] = true;
        }
        ct += ret;
    }
    if ct >= 3 {
        return 1;
    }
    0
}

fn is_single(table: &[[bool;16];16], dex: usize) -> (bool, usize) {
    let mut opt: usize = 0;
    let mut ct: usize = 0;
    for y in 0..16 {
        if table[y][dex] {
            ct += 1;
            opt = y;
        }
    }
    if ct == 1 {
        return (true, opt);
    }
    (false, 0)
}

fn reduce(table: &mut[[bool;16];16]) -> [usize;16] {
    let mut ret: [usize;16] = [0;16];
    let mut total: usize = 0;
    while total != 120 {
        for x in 0..16 {
            let (found, opt) = is_single(&table, x);
            if found {
                ret[x] = opt;
                total += opt;
                for x in 0..16 {
                    table[opt][x] = false;
                }
            }
        }
    }
    ret
}

fn main() {
    let mut f = File::open("inputday16.txt").expect("Error");
    let mut content = String::new();
    f.read_to_string(&mut content).expect("Error in read");
    let mut switch = 0;
    let mut reg: [usize;4] = [0;4];
    let mut input: [usize;4] = [0;4];
    let mut output: [usize;4] = [0;4];
    let mut args: Vec<&Fn([usize;4], [usize;4], [usize;4]) -> (usize, [usize;4])> = Vec::new();
    let mut tot: usize = 0;
    let mut table: [[bool;16]; 16] = [[false;16];16];
    args = vec![&addr, &addi, &mulr, &muli, &banr, &bani, &borr, &bori, &setr, &seti, &gtir,
                 &gtri, &gtrr, &eqir, &eqri, &eqrr];
    let mut raw: Vec<&str> = Vec::new();
    for line in content.split("\n") {
        tot += line.len() + 1;
        if switch % 4 == 0 {
            let cont: Vec<&str> = line.split(' ').collect();
            if cont.len() != 5 {
                break;  
            }
            reg[0] = cont[1].trim_start_matches('[')
                    .trim_end_matches(',')
                    .parse::<usize>().unwrap();
            reg[1] = cont[2].trim_end_matches(',')
                    .parse::<usize>().unwrap();
            reg[2] = cont[3].trim_end_matches(',')
                    .parse::<usize>().unwrap();
            reg[3] = cont[4].trim_end_matches(']')
                    .parse::<usize>().unwrap();
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
                    .parse::<usize>().unwrap();
            output[1] = cont[2].trim_end_matches(',')
                    .parse::<usize>().unwrap();
            output[2] = cont[3].trim_end_matches(',')
                    .parse::<usize>().unwrap();
            output[3] = cont[4].trim_end_matches(']')
                    .parse::<usize>().unwrap();
            findops(reg, input, output, &args, &mut table);
        }
        switch += 1;
    }
    content.drain(..tot);
    let index: [usize;16] = reduce(&mut table);
    let mut ct = 0;
    reg = [0;4];
    input = [0;4];
    for num in content.split_whitespace() {
        if ct % 4 == 0 {
            input[0] = num.parse::<usize>().unwrap();
        }
        else if ct % 4 == 1 {
            input[1] = num.parse::<usize>().unwrap();
        }
        else if ct % 4 == 2 {
            input[2] = num.parse::<usize>().unwrap();
        }
        else if ct % 4 == 3 {
            input[3] = num.parse::<usize>().unwrap();
            let (dummy, ret) = (args[index[input[0]]])(reg, input, input);
            reg = ret;
        }
        ct += 1;
    }
    println!("Val in all registers: {}, {}, {}, {}", reg[0], reg[1], reg[2], reg[3]);
}
