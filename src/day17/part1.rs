use std::cmp;

const SIZE_X: usize = 100;
const SIZE_Y: usize = 100;

// sample
// const T_MIN_X: i32 = 20;
// const T_MAX_X: i32 = 30;
// const T_MIN_Y: i32 = -10;
// const T_MAX_Y: i32 = -5;

// input
const T_MIN_X: i32 = 206;
const T_MAX_X: i32 = 250;
const T_MIN_Y: i32 = -105;
const T_MAX_Y: i32 = -57;

const OFF_X: usize = 0;
const OFF_Y: usize = 25;

pub fn main() {
  // launch(7, 2);
  // launch(6, 3);
  // launch(9, 0);
  // launch(17, -4);
  // launch(6, 9);

  // let mut x:i32 = 5;
  // let mut y:i32 = 5;
  let mut max_y: i32 = 0;

  // let mut min_error_x: i32 = 9999999;
  // let mut min_error_y: i32 = 9999999;
  let mut any_success = false; 
  for x in 0..1000 {
    for y in 0..1000 {
      let (success, probe_max_y, error_x, error_y) = launch(x, y);
      if success {
        println!(
          "{},{} success:{}, max_y:{}, error_x:{}, error_y:{}",
          x, y, success, max_y, error_x, error_y
        );
        any_success = true;
        max_y = cmp::max(max_y, probe_max_y);
      }
      // min_error_x = cmp::min(min_error_x, error_x);
      // min_error_y = cmp::min(min_error_y, error_y);
    }
  }

  println!(
    // "any_success:{}, max_y:{}, min_error_x:{}, min_error_y:{}",
    "any_success:{}, max_y:{}",
    any_success,
    max_y,
    // min_error_x, min_error_y
  );
}

fn launch(init_veloc_x: i32, init_veloc_y: i32) -> (bool, i32, i32, i32) {
  // println!("testing {},{}", init_veloc_x, init_veloc_y);

  // let mut grid = [['.'; SIZE_Y]; SIZE_X];
  // grid[0 + OFF_X][0 + OFF_Y] = 'S';
  // for x in T_MIN_X..=T_MAX_X {
  //   for y in T_MIN_Y..=T_MAX_Y {
  //     grid[(x + OFF_X as i32) as usize][(y + OFF_Y as i32) as usize] = 'T';
  //   }
  // }

  let mut probe_x: i32 = 0;
  let mut probe_y: i32 = 0;
  let mut veloc_x = init_veloc_x;
  let mut veloc_y = init_veloc_y;
  let mut max_probe_y: i32 = 0;
  let mut error_x: i32 = 0;
  let mut error_y: i32 = 0;

  let mut success = false;
  for step in 0..1000 {
    probe_x += veloc_x;
    probe_y += veloc_y;
    max_probe_y = cmp::max(max_probe_y, probe_y);

    // grid[(probe_x + OFF_X as i32) as usize][(probe_y + OFF_Y as i32) as usize] = '#';

    if probe_x >= T_MIN_X && probe_x <= T_MAX_X && probe_y >= T_MIN_Y && probe_y <= T_MAX_Y {
      success = true;
      break;
    }

    if probe_y < T_MIN_Y || probe_x > T_MAX_X {
      break;
    }

    if veloc_x > 0 {
      veloc_x -= 1;
    } else if veloc_x < 0 {
      veloc_x += 1;
    }
    veloc_y -= 1;
  }

  if (success) {
    // println!("SUCCESS! max_y={}", max_probe_y);
  } else {
    // println!("FAIL!");
    if probe_x < T_MIN_X {
      error_x = probe_x - T_MIN_X;
    } else if probe_x > T_MAX_X {
      error_x = probe_x - T_MAX_X;
    } else {
      error_y = probe_y - T_MIN_Y;
    }
  }
  // dump(&grid);

  return (success, max_probe_y, error_x, error_y);
}

fn dump(grid: &[[char; SIZE_Y]; SIZE_X]) {
  for y in 0..SIZE_Y {
    for x in 0..SIZE_X {
      print!("{}", grid[x][SIZE_Y - 1 - y]);
    }
    println!();
  }
  println!();
}
