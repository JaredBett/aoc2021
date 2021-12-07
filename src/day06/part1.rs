const DAYS: u32 = 80;

pub fn main() {
  let mut timers: Vec<u8> = include_str!("input.txt")
    .lines()
    .next()
    .unwrap()
    .split(",")
    .map(|x| x.parse().unwrap())
    .collect();

  for day in 0..DAYS {
    // println!("After {:2} days: {:?}", day, timers);
    let timers_len = timers.len();
    for i in 0..timers_len {
      if timers[i] == 0 {
        timers[i] = 6;
        timers.push(8);
      } else {
        timers[i] -= 1;
      }
    }
  }

  println!("{}", timers.len());
}
