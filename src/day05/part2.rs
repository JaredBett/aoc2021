use regex::Regex;
use std::cmp;

const GRID_SIZE: usize = 1_000;
// const GRID_SIZE: usize = 10;

pub fn main() {
  println!("day5 part1");
  let mut grid = [[0; GRID_SIZE]; GRID_SIZE];

  let lines = include_str!("input.txt").lines();
  // 0,9 -> 5,9
  let re = Regex::new(r"(\d+),(\d+) -> (\d+),(\d+)").unwrap();
  for line in lines {
    let caps = re.captures(line).expect(&format!("Invalid line: {}", line));
    let x1: usize = caps.get(1).unwrap().as_str().parse().unwrap();
    let y1: usize = caps.get(2).unwrap().as_str().parse().unwrap();
    let x2: usize = caps.get(3).unwrap().as_str().parse().unwrap();
    let y2: usize = caps.get(4).unwrap().as_str().parse().unwrap();
    // println!("drawling line {}", line);
    if x1 == x2 {
      for y in cmp::min(y1, y2)..=cmp::max(y1, y2) {
        grid[y][x1] += 1;
      }
    } else if y1 == y2 {
      for x in cmp::min(x1, x2)..=cmp::max(x1, x2) {
        grid[y1][x] += 1;
      }
    } else {
      let mut x = x1;
      let mut y = y1;
      let xd: isize = if x2 > x1 { 1 } else { -1 };
      let yd: isize = if y2 > y1 { 1 } else { -1 };
      let length = x2 as isize - x1 as isize;
      for i in 0..=length.abs() {
        grid[y][x] += 1;
        x = (x as isize + xd) as usize;
        y = (y as isize + yd) as usize;
      }
    }
    // dump(&grid);
  }

  // dump(&grid);

  println!("{}", num_points_gte_2(&grid));
}

fn dump(grid: &[[usize; GRID_SIZE]; GRID_SIZE]) {
  for y in 0..GRID_SIZE {
    for x in 0..GRID_SIZE {
      if grid[y][x] == 0 {
        print!(".");
      } else {
        print!("{}", grid[y][x]);
      }
    }
    println!();
  }
  println!();
}

fn num_points_gte_2(grid: &[[usize; GRID_SIZE]; GRID_SIZE]) -> u32 {
  let mut sum: u32 = 0;
  for y in 0..GRID_SIZE {
    for x in 0..GRID_SIZE {
      if grid[y][x] >= 2 {
        sum += 1;
      }
    }
  }
  return sum;
}
