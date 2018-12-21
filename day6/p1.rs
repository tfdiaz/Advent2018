use std::io::prelude::*;
use std::fs::File;
use std::collections::HashSet;
use std::collections::HashMap;

fn main() {
    let mut f = File::open("day6.txt").expect("Error");
    let mut content = String::new();
    f.read_to_string(&mut content).expect("Error in read");
    let mut grid = [[0usize; 400]; 400];
    let mut points: Vec<[usize; 2]> = Vec::new();
    let mut bounds = HashSet::new();
    let mut total = [0usize; 50];
    for line in content.split('\n') {
        let v: Vec<&str> = line.split(", ").collect();
        if v.len() < 2 {
            break;
        }
        let mut tmp = [0usize; 2];
        tmp[0] = v[0].parse::<usize>().expect("V0 parse error");
        tmp[1] = v[1].parse::<usize>().expect("V1 parse error");
        points.push(tmp);
    }
    for i in 0..points.len() {
        grid[points[i][1]][points[i][0]] = i;
    }
    for y in 0..400 {
        for x in 0..400 {
            let mut mindest: usize = 800;
            let mut min = [0usize; 2];
            let mut flag: usize = 0;
            for point in points.iter() {
                let mut ds_x: usize = 0;
                let mut ds_y: usize = 0;
                if point[0] > x {
                    ds_x = point[0] - x;
                }
                else {
                    ds_x = x - point[0];
                }
                if point[1] > y {
                    ds_y = point[1] - y;
                }
                else {
                    ds_y = y - point[1];
                }
                let dest = ds_x + ds_y;
                if dest < mindest {
                    min = *point;
                    mindest = dest;
                    flag = 0;
                }
                else if (dest == mindest) {
                    flag += 1;
                }
            }
            if (flag > 0) {
                grid[y][x] = 100;
            }
            else {
                grid[y][x] = grid[min[1]][min[0]];
                total[grid[y][x]] += 1;
            }
            if (y == 0 || x == 0 || x == 399 || y == 399) && flag == 0 {
                bounds.insert(grid[y][x]);
            }
        }
    }
    let mut max:usize = 0;
    for i in 0..total.len() {
        if !bounds.contains(&i) && total[i] > max {
            max = total[i];
        }
    }
    println!("Total inner max {} ", max);
}
