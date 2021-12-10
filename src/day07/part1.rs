use std::cmp;

pub fn main() {
  let positions: Vec<u32> = include_str!("input.txt")
    .lines()
    .next()
    .unwrap()
    .split(",")
    .map(|num| num.parse().unwrap())
    .collect();

  let max = max(&positions);
  let min = min(&positions);
  println!("max:{}, min:{}", max, min);

  let mut min_total_cost: i32 = -1;
  let mut min_total_cost_align_target: i32 = -1;
  for align_target in min..=max {
    let mut total_cost = 0;
    for position in &positions {
      total_cost += (align_target as i32 - *position as i32).abs();
    }

    if total_cost < min_total_cost || min_total_cost == -1 {
      min_total_cost = total_cost;
      min_total_cost_align_target = align_target as i32;
    }
  }

  println!(
    "total fuel: {}, align target: {}",
    min_total_cost, min_total_cost_align_target
  );
}

fn max(nums: &Vec<u32>) -> u32 {
  let mut max = 0;
  for num in nums {
    max = cmp::max(*num, max);
  }
  return max;
}

fn min(nums: &Vec<u32>) -> u32 {
  let mut min = 0;
  for num in nums {
    min = cmp::min(*num, min);
  }
  return min;
}
