const SIZE: usize = 10;

pub fn main() {
  let mut grid = [[0 as i32; SIZE]; SIZE];

  let mut y = 0;
  for line in include_str!("input.txt").lines() {
    for (x, c) in line.chars().enumerate() {
      grid[x][y] = c.to_digit(10).unwrap() as i32;
    }
    y += 1;
  }

  println!("Before any steps:");
  dump(&grid);
  let mut total_flashes = 0;
  for step in 1..=100 {
    total_flashes += simulate(&mut grid);
    // println!("After step {}", step);
    // dump(&grid);
  }
  println!("{}", total_flashes);
}

fn simulate(grid: &mut [[i32; SIZE]; SIZE]) -> u32 {
  let mut flashes = 0;

  for x in 0..SIZE {
    for y in 0..SIZE {
      grid[x][y] += 1;
    }
  }
  // dump(grid);

  let mut flashed = true;
  while flashed {
    flashed = false;
    for x in 0..SIZE {
      for y in 0..SIZE {
        if grid[x][y] > 9 {
          flashed = true;
          flashes += 1;
          let x = x as i32;
          let y = y as i32;
          grid[x as usize][y as usize] = -1;
          handle_flash(x - 1, y - 1, grid);
          handle_flash(x, y - 1, grid);
          handle_flash(x + 1, y - 1, grid);
          handle_flash(x + 1, y, grid);
          handle_flash(x + 1, y + 1, grid);
          handle_flash(x, y + 1, grid);
          handle_flash(x - 1, y + 1, grid);
          handle_flash(x - 1, y, grid);
          // println!("({},{}) flashed", x, y);
          // dump(grid);
        }
      }
    }
  }

  for x in 0..SIZE {
    for y in 0..SIZE {
      if grid[x][y] == -1 {
        grid[x][y] = 0;
      }
    }
  }
  return flashes;
}

fn handle_flash(x: i32, y: i32, grid: &mut [[i32; SIZE]; SIZE]) -> bool {
  if x >= 0
    && x <= SIZE as i32 - 1
    && y >= 0
    && y <= SIZE as i32 - 1
    && grid[x as usize][y as usize] != -1
  {
    grid[x as usize][y as usize] += 1;
    return true;
  }
  return false;
}

fn dump(grid: &[[i32; SIZE]; SIZE]) {
  for y in 0..SIZE {
    for x in 0..SIZE {
      print!("{}", grid[x][y]);
    }
    println!();
  }
  println!();
}
