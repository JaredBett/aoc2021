use regex::Regex;
use std::collections::HashMap;

pub fn main() {
  let mut template = String::new();
  let mut rules: HashMap<String, char> = HashMap::new();

  let mut parsing_template = true;
  let rule_re = Regex::new(r"(..) -> (.)").unwrap();
  for line in include_str!("input.txt").lines() {
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
      let insert = caps.get(2).unwrap().as_str().chars().next().unwrap();
      rules.insert(String::from(pair), insert);
    }
  }

  // dbg!(&template);
  // dbg!(&rules);

  println!("Template: {}", template);

  let mut pair_counts: HashMap<String, u64> = HashMap::new();
  let template_chars: Vec<char> = template.chars().collect();
  for i in 0..template_chars.len() - 1 {
    let mut pair = String::new();
    pair.push(template_chars[i]);
    pair.push(template_chars[i + 1]);
    let count = pair_counts.entry(pair).or_insert(0);
    *count += 1;
  }

  for step in 1..=40 {
    // println!("\nexecuting step {}", step);

    let mut new_pair_counts: HashMap<String, u64> = HashMap::new();
    for (pair, count) in &pair_counts {
      if rules.contains_key(pair) {
        let insert = *rules.get(pair).unwrap();
        let pair_chars: Vec<char> = pair.chars().collect();

        let mut new_pair1 = String::new();
        new_pair1.push(pair_chars[0]);
        new_pair1.push(insert);

        let mut new_pair2 = String::new();
        new_pair2.push(insert);
        new_pair2.push(pair_chars[1]);

        let count1 = new_pair_counts.entry(new_pair1).or_insert(0);
        *count1 += count;

        let count2 = new_pair_counts.entry(new_pair2).or_insert(0);
        *count2 += count;
      } else {
        println!("Warning: no rule for pair {}", pair);
      }
    }
    pair_counts = new_pair_counts;
    // dbg!(&pair_counts);

    let mut step_len = 1;
    for (pair, pair_count) in &pair_counts {
      step_len += pair_count;
    }
    println!("step {} len {}", step, step_len);
  }

  // dbg!(&pair_counts);

  let mut char_counts: HashMap<char, u64> = HashMap::new();
  for (pair, pair_count) in pair_counts {
    let ch = pair.chars().next().unwrap();
    let ch_count = char_counts.entry(ch).or_insert(0);
    *ch_count += pair_count;
  }
  let last_char = template.chars().last().unwrap();
  let last_char_count = char_counts.entry(last_char).or_insert(0);
  *last_char_count += 1;

  dbg!(&char_counts);

  let mut most_common_char_count: u64 = 0;
  let mut least_common_char_count: u64 = std::u64::MAX;
  for (ch, count) in char_counts {
    if count > most_common_char_count {
      most_common_char_count = count;
    }
    if count < least_common_char_count {
      least_common_char_count = count;
    }
  }
  println!("{}", most_common_char_count - least_common_char_count);
}
