use ansi_term::Colour;
use std::collections::HashMap;
use std::collections::HashSet;

const MSIZE: usize = 100;
const SIZE: usize = 500;

pub fn main() {
  let mut grid = [[0 as u8; SIZE]; SIZE];

  let mut y = 0;
  for line in include_str!("input.txt").lines() {
    for (x, c) in line.chars().enumerate() {
      grid[x][y] = c.to_digit(10).unwrap() as u8;
    }
    y += 1;
  }
  for repeat_x in 0..5 {
    for repeat_y in 0..5 {
      for x in 0..MSIZE {
        for y in 0..MSIZE {
          grid[x + repeat_x * MSIZE][y + repeat_y * MSIZE] =
            ((grid[x][y] as usize + repeat_x + repeat_y - 1) % 9 + 1) as u8;
        }
      }
    }
  }
  // for x in 0..SIZE {
  //   for y in 0..SIZE {
  //     println!("{}", grid[x][y]);
  //   }
  // }
  // dump(&grid, &vec![]);

  let mut unvisited: HashSet<(u16, u16)> = HashSet::new();
  for x in 0..SIZE {
    for y in 0..SIZE {
      unvisited.insert((x as u16, y as u16));
    }
  }
  let mut best: HashMap<(u16, u16), u32> = HashMap::new();
  for x in 0..SIZE {
    for y in 0..SIZE {
      best.insert((x as u16, y as u16), std::u32::MAX);
    }
  }
  best.insert((0, 0), 0);
  let mut prev: HashMap<(u16, u16), (u16, u16)> = HashMap::new();

  let mut i = 0;
  while !unvisited.is_empty() {
    if i % 100 == 0 {
      println!("{}", unvisited.len());
    }
    i += 1;
    let mut current: (u16, u16) = *unvisited.iter().next().unwrap();
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

    if current == (SIZE as u16 - 1, SIZE as u16 - 1) {
      // let total_cost = cost + grid[SIZE - 1][SIZE - 1] as u32;
      println!("SUCCESS: {}", cost);

      let mut path: Vec<(u16, u16)> = Vec::new();
      let mut node = current;
      while node != (0, 0) {
        path.push(node);
        node = *prev.get(&node).unwrap();
      }
      path.push((0, 0));
      path.reverse();

      // dump(&grid, &path);
      break;
    }
  }
}

fn get_neighbors(x: u16, y: u16) -> Vec<(u16, u16)> {
  let mut neighbors: Vec<(u16, u16)> = Vec::new();

  if y > 0 {
    neighbors.push((x, y - 1));
  }
  if x < SIZE as u16 - 1 {
    neighbors.push((x + 1, y));
  }
  if y < SIZE as u16 - 1 {
    neighbors.push((x, y + 1));
  }
  if x > 0 {
    neighbors.push((x - 1, y));
  }

  return neighbors;
}

fn dump(grid: &[[u8; SIZE]; SIZE], path: &Vec<(u16, u16)>) {
  for y in 0..SIZE {
    for x in 0..SIZE {
      // if path.contains(&(x as u8, y as u8)) {
      // print!("{}", Colour::Red.paint(format!("{}", grid[x][y])));
      // } else {
      print!("{}", grid[x][y]);
      // }
    }
    println!();
  }
  println!();
}
