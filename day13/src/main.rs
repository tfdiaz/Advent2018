use std::io::prelude::*;
use std::fs::File;
use std::cmp::Ordering;

#[derive(PartialEq, Eq)]
enum Move {
    left,
    straight,
    right,
}

#[derive(PartialEq, Eq)]
enum Ori {
    up,
    down,
    left,
    right,
}

#[derive(Eq)]
struct Cart {
    m: Move,
    coord: [usize;2],
    o: Ori,
    prev: char,
    alive: bool,
}

impl Ord for Cart {
    fn cmp(&self, other: &Cart) -> Ordering {
        self.coord.cmp(&other.coord)
    }
}

impl PartialOrd for Cart {
    fn partial_cmp(&self, other: &Cart) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Cart {
    fn eq(&self, other: &Cart) -> bool {
        self.coord[0] == other.coord[0]
    }
}

impl Cart {

    fn new(x: usize, y: usize, place: char) -> Cart {
        let mut or = Ori::up;
        let cord = [y, x];
        let mut prev: char = '|';
        if place == '<' {
            or = Ori::left;
            prev = '-';
        }
        if place == '>' {
            or = Ori::right;
            prev = '-'
        }
        if place == 'v' {
            or = Ori::down;
        }
        Cart {m: Move::left, coord: cord, o: or, prev: prev, alive: true}
    }

    fn shift (&mut self, g: &mut Vec<Vec<char>>) -> bool {
        let mut next: char = '.';
        if !self.alive {
            return true;
        }
        match self.o {
            Ori::up => {
                next = g[self.coord[0] - 1][self.coord[1]];
                g[self.coord[0]][self.coord[1]] = self.prev;
                self.coord[0] -= 1;
            },
            Ori::down => {
                next = g[self.coord[0] + 1][self.coord[1]];
                g[self.coord[0]][self.coord[1]] = self.prev;
                self.coord[0] += 1;
            },
            Ori::right => {
                next = g[self.coord[0]][self.coord[1] + 1];
                g[self.coord[0]][self.coord[1]] = self.prev;
                self.coord[1] += 1;
            },
            Ori::left => {
                next = g[self.coord[0]][self.coord[1] - 1];
                g[self.coord[0]][self.coord[1]] = self.prev;
                self.coord[1] -= 1;
            },
        }

        if next == '+' {
            match self.m {
                Move::left => {
                    match self.o {
                        Ori::up => self.o = Ori::left,
                        Ori::down => self.o = Ori::right,
                        Ori::right => self.o = Ori::up,
                        Ori::left => self.o = Ori::down,
                    }
                    self.m = Move::straight;
                },
                Move::straight => {
                    self.m = Move::right;
                }
                Move::right => {
                    match self.o {
                        Ori::up => self.o = Ori::right,
                        Ori::down => self.o = Ori::left,
                        Ori::right => self.o = Ori::down,
                        Ori::left => self.o = Ori::up,
                    }
                    self.m = Move::left;
                }
            }
        }
        if next == '\\' {
            match self.o {
                Ori::up => self.o = Ori::left,
                Ori::down => self.o = Ori::right,
                Ori::right => self.o = Ori::down,
                Ori::left => self.o = Ori::up,
            }
        }
        if next == '/' {
            match self.o {
                Ori::up => self.o = Ori::right,
                Ori::down => self.o = Ori::left,
                Ori::right => self.o = Ori::up,
                Ori::left => self.o = Ori::down,
            }
        }
        match self.o {
            Ori::up => g[self.coord[0]][self.coord[1]] = '^',
            Ori::down => g[self.coord[0]][self.coord[1]] = 'v',
            Ori::right => g[self.coord[0]][self.coord[1]] = '>',
            Ori::left => g[self.coord[0]][self.coord[1]] = '<',
        }
        self.prev = next;
        if next == '<' || next == '^' || next == 'v' || next == '>' {
            self.alive = false;
            return false;
        }
        true
    }
}

fn main() {
    let mut f = File::open("day13.txt").expect("Error in opening");
    let mut content = String::new();
    f.read_to_string(&mut content).expect("error");
    let mut g: Vec<Vec<char>> = Vec::new();
    for line in content.split('\n') {
        if line.len() < 5 {
            break;
        }
        g.push(line.chars().collect());
    }
    let mut carts: Vec<Cart> = Vec::new();
    for y in 0..g.len() {
        for x in 0..g[y].len() {
            if g[y][x] == '<' || g[y][x] == '^' || g[y][x] == 'v' || g[y][x] == '>' {
                let cart = Cart::new(x, y, g[y][x]);
                carts.push(cart);
            }
        }
    }
    carts.sort();
    loop {
        let mut flag = true;
        for i in 0..carts.len() {
            if !carts[i].shift(&mut g) {
                println!("HIT X: {}, Y: {}", carts[i].coord[1], carts[i].coord[0]);
                for x in 0..carts.len() {
                    if carts[i].coord == carts[x].coord && carts[x].alive {
                        g[carts[i].coord[0]][carts[i].coord[1]] = carts[x].prev;
                        carts[x].alive = false;
                        println!("Match! X: {}, Y: {}", carts[x].coord[1], carts[x].coord[0]);
                    }
                }
            }
        }
        let mut count: usize = 0;
        for cart in carts.iter() {
            if cart.alive {
                count += 1;
            }
        }
        if count == 1 {
            for cart in carts.iter() {
                if cart.alive {
                    println!("Final:: X: {}, Y: {}", cart.coord[1], cart.coord[0]);
                }
            }
            break;
        }
        carts.sort();
    }
}
