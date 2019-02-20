use std::io::prelude::*;
use std::fs::File;
use std::collections::VecDeque;

#[derive(Copy, Clone, PartialEq)]
enum State {
    sand,
    clay,
    water,
}

fn leftbound(board: &[[State;1000];2000], x: usize, y: usize) -> bool {
    for i in (x +1)..0 {
        if board[y][i] == State::clay && (board[y + 1][i] == State::clay || board[y + 1][i] == State::water) {
            return true;
        }
        if board[y][i] == State::sand && board[y + 1][i] == State::clay {
            return true;
        }
        if board[y][i] == State::sand && board[y + 1][i] == State::sand {
            return false;
        }
    }
    false
}

fn rightbound(board: &[[State;1000];2000], x: usize, y: usize) -> bool {
    for i in x..1000 {
        if board[y][i] == State::clay && (board[y + 1][i] == State::clay || board[y + 1][i] == State::water) {
            return true;
        }
        if board[y][i] == State::sand && board[y + 1][i] == State::clay {
            return true;
        }
        if board[y][i] == State::sand && board[y + 1][i] == State::sand {
            return false;
        }
    }
    false
}

fn main() {
    let mut content = String::new();
    let mut f = File::open("day17.txt").unwrap();
    f.read_to_string(&mut content);
    let mut board: [[State;1000];2000] = [[State::sand;1000];2000];
    let mut ymax: usize = 0;
    for line in content.split("\n") {
        if line.len() < 1 {
            break;
        }
        let mut args: Vec<&str> = line.split_whitespace().collect();
        let mut fargs: Vec<&str> = args[0].split("=").collect();
        let mut sargs: Vec<&str> = args[1].split("=").collect();
        if fargs[0] == "x" {
            let x: usize = fargs[1].trim_end_matches(",").parse::<usize>().unwrap();
            let bound: Vec<&str> = sargs[1].split("..").collect();
            for y in bound[0].parse::<usize>().unwrap()..(bound[1].parse::<usize>().unwrap() + 1) {
                board[y][x] = State::clay;
                if y > ymax {
                    ymax = y;
                }
            }
        }
        else {
            let y: usize = fargs[1].trim_end_matches(",").parse::<usize>().unwrap();
            if y > ymax {
                ymax = y;
            }
            let bound: Vec<&str> = sargs[1].split("..").collect();
            for x in bound[0].parse::<usize>().unwrap()..(bound[1].parse::<usize>().unwrap() + 1) {
                board[y][x] = State::clay;
            }
        }
    }
    let mut ct: usize = 0;
    let mut cord: [usize;2] = [500, 1];
    let mut streams: VecDeque<[usize;2]> = VecDeque::new();
    streams.push_front(cord);
    while streams.len() != 0 {
        let mut cord = streams.pop_front().unwrap();
        let mut x = cord[0];
        let mut y = cord[1];
        while y <= ymax {
            let mut leftfall = false;
            if board[y][x] == State::sand {
                board[y][x] = State::water;
                ct += 1;
            }
            if board[y + 1][x] == State::sand {
                y += 1;
            }
            else if board[y + 1][x] == State::water {
                let mut tmp_y = y;
                let mut flag = false;
                loop {
                    if board[tmp_y][x - 1] == State::sand {
                        let mut tmp_x = x - 1;
                        if leftbound(&board, tmp_x, tmp_y) {
                            while board[tmp_y][tmp_x] != State::clay {
                                board[tmp_y][tmp_x] = State::water;
                                if board[tmp_y + 1][tmp_x] == State::sand {
                                    streams.push_back([tmp_x, tmp_y + 1]);
                                    leftfall = true;
                                    break;
                                }
                                tmp_x -= 1;
                            }
                        }
                    }
                    if rightbound(&board, x, tmp_y) {
                        let mut tmp_x = x + 1;
                        while board[tmp_y][tmp_x] != State::clay {
                            board[tmp_y][tmp_x] = State::water;
                            if board[tmp_y + 1][tmp_x] == State::sand {
                                leftfall = false;
                                flag = true;
                                x = tmp_x;
                                y = tmp_y + 1;
                                break;
                            }
                            tmp_x += 1;
                        }
                    }
                    else {
                        println!("Jere");
                        break;
                    }
                    if flag || leftfall {
                        break;
                    }
                    tmp_y -= 1;
            for j in 0..100 {
            for k in 400..600 {
                if j != y + 1 || k != x {
                match board[j][k] {
                    State::sand => print!("."),
                    State::clay => print!("#"),
                    State::water => print!("~"),
                }
                }
                else {
                    print!("X");
                }
            }
            println!("");
            }
                }
                break;
            }
            else if board[y + 1][x] == State::clay {
                let mut flag = true;
                while flag {
                    //println!("Innerrr");
                    let mut tmp_x = x - 1;
                    while board[y][tmp_x] != State::clay {
                        board[y][tmp_x] = State::water;
                        ct += 1;
                        if board[y + 1][tmp_x] == State::sand {
                            streams.push_back([tmp_x, y + 1]);
                            leftfall = true;
                            break;
                        }
                        tmp_x -= 1;
                    }
                    tmp_x = x + 1;
                    while board[y][tmp_x] != State::clay {
                        board[y][tmp_x] = State::water;
                        ct += 1;
                        if board[y + 1][tmp_x] == State::sand {
                            flag = false;
                            leftfall = false;
                            y += 1;
                            x = tmp_x;
                            break;
                        }
                        tmp_x += 1;
                    }
                    if leftfall {
                        break;
                    }
                    if flag {
                        y -= 1;
                    }
                }
            }
            if leftfall {
                break;
            }
    }
    }
    for y in 0..100 {
        for x in 400..600 {
            match board[y][x] {
                State::sand => print!("."),
                State::clay => print!("#"),
                State::water => print!("~"),
            }
        }
        println!("");
    }
    println!("{}",ymax);
}
