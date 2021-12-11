use std::collections::HashMap;

pub fn main() {
  let mut total = 0;
  for line in include_str!("input.txt").lines() {
    total += syntax_error_score(line);
  }
  println!("{}", total);
}

fn syntax_error_score(line: &str) -> u32 {
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
      println!("{} Expected {}, found {}", line, stack[stack.len() - 1], c);
      let scores = HashMap::from([(')', 3), (']', 57), ('}', 1197), ('>', 25137)]);
      return *scores.get(&c).unwrap();
    }
  }

  return 0;
}
