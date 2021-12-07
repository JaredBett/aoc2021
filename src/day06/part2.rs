use std::collections::HashMap;

pub fn main() {
  let days: u32 = 256;

  let mut timers: HashMap<u8, u64> = HashMap::new();
  let input_timers = include_str!("input.txt")
    .lines()
    .next()
    .unwrap()
    .split(",");

  for timer in input_timers {
    let timer: u8 = timer.parse().unwrap();
    let count = timers.entry(timer).or_insert(0);
    *count += 1;
  }

  for day in 0..days {
    // println!("After {:2} days: {} {:?}", day, total(&timers), timers);

    let mut new_timers = HashMap::new();
    for (timer, count) in &mut timers {
      if *timer == 0 {
        let count6 = new_timers.entry(6).or_insert(0);
        *count6 += *count;
        let count8 = new_timers.entry(8).or_insert(0);
        *count8 += *count;
      } else {
        let count_lt1 = new_timers.entry(*timer - 1).or_insert(0);
        *count_lt1 += *count;
      }
    }

    timers = new_timers;
  }

  println!("{}", total(&timers));
}

fn total(timers: &HashMap<u8, u64>) -> u64 {
  let mut total = 0;
  for (timer, count) in timers {
    total += count;
  }
  return total;
}
