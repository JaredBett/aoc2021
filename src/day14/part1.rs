use regex::Regex;
use std::collections::HashMap;

pub fn main() {
  let mut template = String::new();
  let mut rules: HashMap<&str, &str> = HashMap::new();

  let mut parsing_template = true;
  let rule_re = Regex::new(r"(..) -> (.)").unwrap();
  for line in include_str!("sample.txt").lines() {
    if line == "" {
      parsing_template = false;
      continue;
    }

    if parsing_template {
      template.push_str(line);
    } else {
      let caps = rule_re
        .captures(line)
        .expect(&format!("Invalid rule: {}", line));
      let pair = caps.get(1).unwrap().as_str();
      let insert = caps.get(2).unwrap().as_str();
      rules.insert(&pair, &insert);
    }
  }

  // dbg!(&template);
  // dbg!(&rules);

  println!("Template: {}", template);

  for step in 1..=4 {
    println!("executing step {}", step);
    let template_chars: Vec<char> = template.chars().collect();
    let mut new_template = String::new();
    for (i, current_char) in template_chars.iter().enumerate() {
      if i < template.len() - 1 {
        let next_char = template_chars[i + 1];
        let mut pair = String::new();
        pair.push(*current_char);
        pair.push(next_char);
        println!("looking at pair {}", pair);

        if rules.contains_key(&*pair) {
          let insert = *rules.get(&*pair).unwrap();
          println!("inserting {}", insert);
          new_template.push(*current_char);
          new_template.push_str(insert);
          if i == template.len() - 2 {
            new_template.push(next_char);
          }
        } else {
          println!("Warning: no rule for pair {}", pair);
        }
        println!("new template is {}", new_template);
      }
    }
    template = new_template;
    println!("After step {}: {}", step, template);
  }

  let mut char_counts: HashMap<char, usize> = HashMap::new();
  for ch in template.chars() {
    let count = char_counts.entry(ch).or_insert(0);
    *count += 1;
  }

  let mut most_common_char: char;
  let mut most_common_char_count: usize = 0;
  let mut least_common_char: char;
  let mut least_common_char_count: usize = 999999999;
  for (ch, count) in char_counts {
    if count > most_common_char_count {
      most_common_char_count = count;
      most_common_char = ch;
    }
    if count < least_common_char_count {
      least_common_char_count = count;
      least_common_char = ch;
    }
  }
  println!("{}", most_common_char_count - least_common_char_count);
}
