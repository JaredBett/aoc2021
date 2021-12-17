use ansi_term::Colour;
use rand::Rng;
use std::cmp;

const SIZE: usize = 100;

static mut NUM_STEPS: u64 = 0;

pub fn main() {
  let mut grid = [[0 as u8; SIZE]; SIZE];

  let mut y = 0;
  for line in include_str!("input.txt").lines() {
    for (x, c) in line.chars().enumerate() {
      grid[x][y] = c.to_digit(10).unwrap() as u8;
    }
    y += 1;
  }

  let mut rng = rand::thread_rng();

  println!("start:");
  dump(&grid, &vec![]);

  let path1 = simple_path1(&grid);
  let mut best_score = score_path(&path1);
  println!("starting best score: {}", best_score);
  dump(&grid, &path1);

  let mut attempts: Vec<Vec<(u8, u8, u8)>> = Vec::new();

  // attempts.push(vec![(1, 2, 3)]);
  // println!("{}", attempts.contains(&vec![(4, 2, 1)]));

  // let mut num_full_paths: u64 = 0;
  // let mut num_partial_paths: u64 = 0;
  // let mut num_dup_paths: u64 = 0;
  find_paths(
    vec![(0, 0, grid[0][0])],
    0,
    &grid,
    &mut best_score,
    &mut attempts,
    // &mut num_dup_paths,
    &mut rng,
  );
}

fn find_paths(
  existing_path: Vec<(u8, u8, u8)>,
  existing_score: u16,
  grid: &[[u8; SIZE]; SIZE],
  best_score: &mut u16,
  attempts: &mut Vec<Vec<(u8, u8, u8)>>,
  rnd: &mut rand::prelude::ThreadRng, // num_dup_paths: &mut u64,
) {
  unsafe {
    NUM_STEPS += 1;
    if NUM_STEPS % 10000000 == 0 {
      dump(&grid, &existing_path);
      println!(
        "best: {}, steps: {}, attempts: {}, last len: {}, last score: {}",
        // "best: {}, partial: {}, full: {}, dups: {}, last len: {}, last score: {}",
        *best_score,
        NUM_STEPS,
        attempts.len(),
        // *num_dup_paths,
        existing_path.len(),
        existing_score
      );
    }
  }

  // if attempts.contains(&existing_path) {
  //   // *num_dup_paths += 1;
  //   return;
  // }
  // attempts.push(existing_path.clone());

  if existing_score > *best_score {
    // *num_partial_paths += 1;
    // attempts.push(existing_path);
    return;
  }

  let (last_x, last_y, last_score) = existing_path.iter().last().unwrap();

  let best_case_remaining_score = SIZE as u16 - *last_x as u16 + SIZE as u16 - *last_y as u16;
  if existing_score + best_case_remaining_score > *best_score {
    return;
  }

  // let (last_x, last_y, last_score) = existing_path[existing_path.len() - 1];
  if *last_x == SIZE as u8 - 1 && *last_y == SIZE as u8 - 1 {
    // NUM_FULL_PATHS += 1;
    // let score = score_path(&existing_path, grid);
    if existing_score < *best_score {
      *best_score = existing_score;
      println!("new best path {}", existing_score);
      dump(grid, &existing_path);
    } else {
      println!("new path, but worse score {}", existing_score);
    }
    // attempts.push(existing_path);
    return;
  }

  let next_options = next_path_options(&existing_path, grid, rnd);
  for option in next_options {
    let mut path = existing_path.clone();
    path.push(option);
    let (x, y, score) = option;
    let new_score = existing_score + score as u16;
    find_paths(
      path, new_score, grid, best_score, attempts,
      // num_partial_paths,
      // num_full_paths,
      // num_dup_paths,
      rnd,
    );
  }
}

fn next_path_options(
  path: &Vec<(u8, u8, u8)>,
  grid: &[[u8; SIZE]; SIZE],
  rnd: &mut rand::prelude::ThreadRng,
) -> Vec<(u8, u8, u8)> {
  let (x, y, last_score) = *path.iter().last().unwrap();
  let mut options: Vec<(u8, u8, u8)> = Vec::new();

  // top
  if y > 0 {
    add_valid_option(x, y - 1, &mut options, path, grid);
  }

  // bottom
  if y < SIZE as u8 - 1 {
    add_valid_option(x, y + 1, &mut options, path, grid);
  }

  // left
  if x > 0 {
    add_valid_option(x - 1, y, &mut options, path, grid);
  }

  // right
  if x < SIZE as u8 - 1 {
    add_valid_option(x + 1, y, &mut options, path, grid);
  }

  // sort options by ascending score?
  options.sort_by(|a, b| {
    // return if rnd.gen::<u8>() % 2 == 0 {
    //   cmp::Ordering::Greater
    // } else {
    //   cmp::Ordering::Less
    // };

    let a_dist: usize = SIZE - a.0 as usize + SIZE - a.1 as usize;
    let b_dist: usize = SIZE - b.0 as usize + SIZE - b.1 as usize;
    let result = a_dist.cmp(&b_dist);

    if result == cmp::Ordering::Equal {
      // return (a.0 * a.1).cmp(&(b.0 * b.1));
      return a.2.cmp(&b.2);
    }
    return result;
  });

  return options;
}

fn add_valid_option(
  x: u8,
  y: u8,
  options: &mut Vec<(u8, u8, u8)>,
  path: &Vec<(u8, u8, u8)>,
  grid: &[[u8; SIZE]; SIZE],
) {
  let next = (x, y, grid[x as usize][y as usize]);
  if !path.contains(&next) {
    options.push(next);
  }
}

fn simple_path1(grid: &[[u8; SIZE]; SIZE]) -> Vec<(u8, u8, u8)> {
  let mut path: Vec<(u8, u8, u8)> = vec![];

  let mut x: u8 = 0;
  let mut y: u8 = 0;
  for i in 0..(SIZE - 1) * 2 {
    path.push((x, y, grid[x as usize][y as usize]));
    if path.len() % 2 == 1 {
      x += 1;
    } else {
      y += 1;
    }
  }

  return path;
}

fn score_path(path: &Vec<(u8, u8, u8)>) -> u16 {
  let mut total_score: u16 = 0;
  for (x, y, score) in path {
    total_score += *score as u16;
  }
  return total_score;
}

fn dump(grid: &[[u8; SIZE]; SIZE], path: &Vec<(u8, u8, u8)>) {
  for y in 0..SIZE {
    for x in 0..SIZE {
      if !path.is_empty() && path.iter().last().unwrap() == &(x as u8, y as u8, grid[x][y]) {
        print!("{}", Colour::Yellow.paint(format!("{}", grid[x][y])));
      } else if path.contains(&(x as u8, y as u8, grid[x][y])) {
        print!("{}", Colour::Red.paint(format!("{}", grid[x][y])));
      } else {
        print!("{}", grid[x][y]);
      }
    }
    println!();
  }
  println!();
}
