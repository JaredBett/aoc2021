use ansi_term::Colour;
use std::collections::HashMap;
use std::collections::HashSet;

const SIZE: usize = 100;

pub fn main() {
  let mut grid = [[0 as u8; SIZE]; SIZE];

  let mut y = 0;
  for line in include_str!("input.txt").lines() {
    for (x, c) in line.chars().enumerate() {
      grid[x][y] = c.to_digit(10).unwrap() as u8;
    }
    y += 1;
  }

  let mut unvisited: HashSet<(u8, u8)> = HashSet::new();
  for x in 0..SIZE {
    for y in 0..SIZE {
      unvisited.insert((x as u8, y as u8));
    }
  }
  let mut best: HashMap<(u8, u8), u32> = HashMap::new();
  for x in 0..SIZE {
    for y in 0..SIZE {
      best.insert((x as u8, y as u8), std::u32::MAX);
    }
  }
  best.insert((0, 0), 0);
  let mut prev: HashMap<(u8, u8), (u8, u8)> = HashMap::new();

  while !unvisited.is_empty() {
    let mut current: (u8, u8) = *unvisited.iter().next().unwrap();
    for node in &unvisited {
      if best.get(&node).unwrap() < best.get(&current).unwrap() {
        current = *node;
      }
    }
    // println!("visiting node: {:?}", current);
    // dump(&grid, &vec![current]);

    let cost = *best.get(&current).unwrap();
    let (x, y) = current;

    for neighbor in get_neighbors(x, y) {
      if !unvisited.contains(&neighbor) {
        continue;
      }

      let (nx, ny) = neighbor;
      let tentative_cost = cost + grid[nx as usize][ny as usize] as u32;
      if tentative_cost < *best.get(&neighbor).unwrap() {
        best.insert(neighbor, tentative_cost);
        prev.insert(neighbor, current);
      }
    }

    unvisited.remove(&current);

    if current == (SIZE as u8 - 1, SIZE as u8 - 1) {
      // let total_cost = cost + grid[SIZE - 1][SIZE - 1] as u32;
      println!("SUCCESS: {}", cost);

      let mut path: Vec<(u8, u8)> = Vec::new();
      let mut node = current;
      while node != (0, 0) {
        path.push(node);
        node = *prev.get(&node).unwrap();
      }
      path.push((0, 0));
      path.reverse();

      dump(&grid, &path);
      break;
    }
  }
}

fn get_neighbors(x: u8, y: u8) -> Vec<(u8, u8)> {
  let mut neighbors: Vec<(u8, u8)> = Vec::new();

  if y > 0 {
    neighbors.push((x, y - 1));
  }
  if x < SIZE as u8 - 1 {
    neighbors.push((x + 1, y));
  }
  if y < SIZE as u8 - 1 {
    neighbors.push((x, y + 1));
  }
  if x > 0 {
    neighbors.push((x - 1, y));
  }

  return neighbors;
}

fn dump(grid: &[[u8; SIZE]; SIZE], path: &Vec<(u8, u8)>) {
  for y in 0..SIZE {
    for x in 0..SIZE {
      if path.contains(&(x as u8, y as u8)) {
        print!("{}", Colour::Red.paint(format!("{}", grid[x][y])));
      } else {
        print!("{}", grid[x][y]);
      }
    }
    println!();
  }
  println!();
}
