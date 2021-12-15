use regex::Regex;
use std::cmp;

pub fn main() {
  let mut dots: Vec<(usize, usize)> = Vec::new();
  let mut folds: Vec<(usize, usize)> = Vec::new();

  let mut parsing_dots = true;
  let fold_re = Regex::new(r"fold along (x|y)=(\d+)").unwrap();
  let mut max_x: usize = 0;
  let mut max_y: usize = 0;
  for line in include_str!("input.txt").lines() {
    if line == "" {
      parsing_dots = false;
      continue;
    }

    if parsing_dots {
      let mut parts = line.split(",");
      let x: usize = parts.next().unwrap().parse().unwrap();
      let y: usize = parts.next().unwrap().parse().unwrap();
      max_x = cmp::max(x, max_x);
      max_y = cmp::max(y, max_y);
      dots.push((x, y));
    } else {
      let caps = fold_re
        .captures(line)
        .expect(&format!("Invalid fold: {}", line));
      let axis = caps.get(1).unwrap().as_str();
      let fold_value: usize = caps.get(2).unwrap().as_str().parse().unwrap();
      if axis == "x" {
        folds.push((fold_value, 0));
      } else {
        folds.push((0, fold_value));
      }
    }
  }

  let mut grid = vec![vec![false; max_y + 1]; max_x + 1];
  for (x, y) in dots {
    grid[x][y] = true;
  }

  // dump(&grid);

  // dbg!(&folds);

  for (fold_x, fold_y) in folds {
    println!("Executing fold along: {:?}", (fold_x, fold_y));
    fold(&mut grid, fold_x, fold_y);
    // dump(&grid);
  }

  // dump(&grid);
  println!("{}", visible(&grid));
  // XXXXXXXX
  //
}

fn fold(grid: &mut Vec<Vec<bool>>, fold_x: usize, fold_y: usize) {
  let x_size = grid.len();
  let y_size = grid[0].len();

  if fold_y > 0 {
    for x in 0..x_size {
      if grid[x][fold_y] {
        println!("ERROR: dots along y fold line {}", fold_y);
        return;
      }
    }

    let dy = y_size - fold_y - 1;
    for x in 0..x_size {
      for y in (fold_y - dy)..fold_y {
        grid[x][y] |= grid[x][fold_y + dy - y];
      }
    }
    for x in 0..x_size {
      grid[x].resize(fold_y, false);
    }
  }

  if fold_x > 0 {
    for y in 0..y_size {
      if grid[fold_x][y] {
        println!("ERROR: dots along x fold line {}", fold_x);
        return;
      }
    }

    let dx = x_size - fold_x - 1;
    for y in 0..y_size {
      for x in (fold_x - dx)..fold_x {
        grid[x][y] |= grid[fold_x + dx - x][y];
      }
    }
    grid.resize(fold_x, vec![false; y_size]);
  }
}

fn dump(grid: &Vec<Vec<bool>>) {
  for y in 0..grid[0].len() {
    for x in 0..grid.len() {
      let display = if grid[x][y] { '#' } else { '.' };
      print!("{}", display);
    }
    println!();
  }
  println!();
}

fn visible(grid: &Vec<Vec<bool>>) -> u32 {
  let mut total = 0;
  for col in grid {
    for cell in col {
      if *cell {
        total += 1;
      }
    }
  }
  return total;
}
