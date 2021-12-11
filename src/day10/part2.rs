use std::collections::HashMap;

pub fn main() {
  let mut scores: Vec<u64> = Vec::new();
  for line in include_str!("input.txt").lines() {
    let score = autocomplete_score(line);
    if score > 0 {
      scores.push(score);
    }
  }
  scores.sort();
  // dbg!(&scores);
  println!("{}", scores[scores.len() / 2]);
}

fn autocomplete_score(line: &str) -> u64 {
  let expecting_map = HashMap::from([('(', ')'), ('[', ']'), ('{', '}'), ('<', '>')]);
  let mut stack: Vec<char> = Vec::new();
  for c in line.chars() {
    let expecting: char = if stack.len() > 0 {
      *expecting_map.get(&stack[stack.len() - 1]).unwrap()
    } else {
      ' '
    };
    if c == '{' || c == '(' || c == '[' || c == '<' {
      stack.push(c);
    } else if c == expecting {
      stack.remove(stack.len() - 1);
    } else {
      // println!("{}", line);
      println!("{} exp {} fnd {}", line, stack[stack.len() - 1], c);
      return 0;
    }
  }

  let mut score: u64 = 0;

  // valid line
  if stack.len() == 0 {
    return 0;
  }

  let mut autocompleted = String::new();
  stack.reverse();
  let scores = HashMap::from([(')', 1), (']', 2), ('}', 3), ('>', 4)]);
  for c in stack {
    let missing_char = *expecting_map.get(&c).unwrap();
    autocompleted.push(missing_char);
    score *= 5;
    score += scores.get(&missing_char).unwrap();
  }
  println!("{} {}", autocompleted, score);

  return score;
}
