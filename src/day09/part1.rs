// const GRID_SIZE: usize = 10;
const GRID_SIZE: usize = 100;

pub fn main() {
  let mut grid = [[9 as u8; GRID_SIZE]; GRID_SIZE];

  let mut y = 0;
  for line in include_str!("input.txt").lines() {
    for (x, c) in line.chars().enumerate() {
      grid[x][y] = c.to_digit(10).unwrap() as u8;
    }
    y += 1;
  }

  // dump(grid);

  let mut risk_levels_sum: i32 = 0;
  for x in 0..GRID_SIZE {
    for y in 0..GRID_SIZE {
      let mut up = 9;
      let mut down = 9;
      let mut left = 9;
      let mut right = 9;

      if y > 0 {
        up = grid[x][y - 1];
      }
      if y < GRID_SIZE - 1 {
        down = grid[x][y + 1];
      }
      if x > 0 {
        left = grid[x - 1][y];
      }
      if x < GRID_SIZE - 1 {
        right = grid[x + 1][y];
      }

      if grid[x][y] < up && grid[x][y] < down && grid[x][y] < left && grid[x][y] < right {
        let risk_level = grid[x][y] + 1;
        risk_levels_sum += risk_level as i32;
      }
    }
  }
  println!("{}", risk_levels_sum);
}

fn dump(grid: [[u8; GRID_SIZE]; GRID_SIZE]) {
  for y in 0..GRID_SIZE {
    for x in 0..GRID_SIZE {
      print!("{}", grid[x][y]);
    }
    println!();
  }
}
