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

  let mut basin_sizes: Vec<usize> = Vec::new();
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
        println!("low point found at ({},{}) of height {}", x, y, grid[x][y]);
        // find out how big the basin is

        let mut radius: i32 = 1;
        let mut basin: Vec<(usize, usize)> = vec![(x, y)];
        loop {
          let begin_basin_size = basin.len();

          let mut basin_candidates: Vec<(usize, usize)> = Vec::new();

          // top
          let ny: i32 = y as i32 - radius;
          for nx in (x as i32 - radius)..=(x as i32 + radius) {
            add_to_basin_candidates((nx, ny), grid[x][y], &mut basin_candidates, &grid);
          }

          // bottom
          let ny: i32 = y as i32 + radius;
          for nx in (x as i32 - radius)..=(x as i32 + radius) {
            add_to_basin_candidates((nx, ny), grid[x][y], &mut basin_candidates, &grid);
          }

          // left
          let nx: i32 = x as i32 - radius;
          for ny in (y as i32 - radius + 1)..=(y as i32 + radius - 1) {
            add_to_basin_candidates((nx, ny), grid[x][y], &mut basin_candidates, &grid);
          }

          // right
          let nx: i32 = x as i32 + radius;
          for ny in (y as i32 - radius + 1)..=(y as i32 + radius - 1) {
            add_to_basin_candidates((nx, ny), grid[x][y], &mut basin_candidates, &grid);
          }

          // filter basin candidates to those adjacent to existing basin points
          loop {
            let mut basin_changed = false;
            let mut candidate_indexes_to_remove: Vec<usize> = Vec::new();
            for (i, (nx, ny)) in basin_candidates.iter().enumerate() {
              if adj_to_basin((*nx as i32, *ny as i32), &basin) {
                basin.push((*nx, *ny));
                basin_changed = true;
                candidate_indexes_to_remove.push(i);
              }
            }
            candidate_indexes_to_remove.sort();
            candidate_indexes_to_remove.reverse();
            for i in candidate_indexes_to_remove {
              basin_candidates.remove(i);
            }
            if !basin_changed {
              break;
            }
          }

          if basin.len() == begin_basin_size {
            break;
          }
          radius += 1;
        }

        println!("basin size: {}", basin.len());
        basin_sizes.push(basin.len());
      }
    }
  }

  basin_sizes.sort();
  basin_sizes.reverse();

  println!("{}", basin_sizes[0] * basin_sizes[1] * basin_sizes[2]);
}

fn add_to_basin_candidates(
  (x, y): (i32, i32),
  low_height: u8,
  basin_candidates: &mut Vec<(usize, usize)>,
  grid: &[[u8; GRID_SIZE]; GRID_SIZE],
) {
  let height = grid_cell_or(*grid, x as i32, y as i32, 9);
  if height > low_height && height < 9 {
    basin_candidates.push((x as usize, y as usize));
  }
}

fn adj_to_basin((nx, ny): (i32, i32), basin: &Vec<(usize, usize)>) -> bool {
  for (x, y) in basin {
    let x = *x as i32;
    let y = *y as i32;
    // top
    if nx == x && ny == y - 1 {
      return true;
    }
    // bottom
    if nx == x && ny == y + 1 {
      return true;
    }
    // left
    if ny == y && nx == x - 1 {
      return true;
    }
    // right
    if ny == y && nx == x + 1 {
      return true;
    }
  }

  return false;
}

fn grid_cell_or(grid: [[u8; GRID_SIZE]; GRID_SIZE], x: i32, y: i32, default: u8) -> u8 {
  if x < 0 || y < 0 || x > GRID_SIZE as i32 - 1 || y > GRID_SIZE as i32 - 1 {
    return default;
  }
  return grid[x as usize][y as usize];
}

fn dump(grid: [[u8; GRID_SIZE]; GRID_SIZE]) {
  for y in 0..GRID_SIZE {
    for x in 0..GRID_SIZE {
      print!("{}", grid[x][y]);
    }
    println!();
  }
}
