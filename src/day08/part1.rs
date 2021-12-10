
pub fn main() {
  let lines = include_str!("input.txt").lines();
  let mut total_unique = 0;
  for line in lines {
    let parts: Vec<&str> = line.split(" | ").collect();
    let patterns = parts[0];
    let outputs = parts[1];
    // println!("{} ### {}", patterns, display);
    for output in outputs.split(' ') {
      if output.len() == 2 || output.len() == 3 || output.len() == 4 || output.len() == 7 {
        total_unique += 1;
      }
    }
  }

  println!("{}", total_unique);
}
