use std::io::prelude::*;
use std::fs::File;

fn c_area(v_pos: &Vec<[i32;2]>) -> i32 {
    let mut max_x: i32 = std::i32::MIN;
    let mut min_x: i32 = std::i32::MAX;
    let mut max_y: i32 = std::i32::MIN;
    let mut min_y: i32 = std::i32::MAX;

    for [x, y] in v_pos {
        if *x > max_x {
            max_x = *x;
        }
        if *x < min_x {
            min_x = *x;
        }
        if *y > max_y {
            max_y = *y;
        }
        if *y < min_y {
            min_y = *y;
        }
    }
    let cal: i32 = max_x - min_x + max_y - min_y;
    cal
}

fn main() {
    let mut f = File::open("day10.txt").expect("Error");
    let mut content = String::new();
    f.read_to_string(&mut content).expect("Error in read");

    let mut v_pos: Vec<[i32;2]> = Vec::new();
    let mut v_vol: Vec<[i32;2]> = Vec::new();
    for line in content.split("\n") {
        let word: Vec<&str> = line.split_whitespace().collect();
        if word.len() < 3 {
            break;
        }
        let mut arr = [0i32;2];
        arr[0] = word[1].trim_end_matches(",").parse::<i32>().unwrap();
        arr[1] = word[2].parse::<i32>().unwrap();
        v_pos.push(arr);
        arr[0] = word[5].trim_end_matches(",").parse::<i32>().unwrap();
        arr[1] = word[6].parse::<i32>().unwrap();
        v_vol.push(arr);
    }
    let mut area: i32 = c_area(&v_pos);
    let mut change: i32 = -1;
    let mut count: usize = 0;
    while change < 0 {
        let prev = area;
        for i in 0..v_pos.len() {
            v_pos[i][0] += v_vol[i][0];
            v_pos[i][1] += v_vol[i][1];
        }
        area = c_area(&v_pos);
        change = area - prev;
        count += 1;
    }
    for i in 0..v_pos.len() {
        v_pos[i][0] -= v_vol[i][0];
        v_pos[i][1] -= v_vol[i][1];
    }
    let mut grid = [[0i32; 300];300];
    for i in 0..v_pos.len() {
        grid[v_pos[i][1] as usize][v_pos[i][0] as usize] = 1;
    }
    for y in 0..300 {
        for x in 0..300 {
            if grid[y][x] == 1 {
                print!("X");
            }
            else {
                print!(".");
            }
        }
        println!("");
    }
    println!("Count {}", count);
}
