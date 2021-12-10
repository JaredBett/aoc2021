use std::collections::HashMap;

#[derive(Debug)]
struct Entry {
  patterns: Vec<Vec<char>>,
  outputs: Vec<Vec<char>>,
  outputs_str: String,
}

pub fn main() {
  let lines: Vec<&str> = include_str!("input.txt").lines().collect();
  let entries = parse(&lines);
  // dbg!(entries);

  let mut outputs_sum = 0;

  for (entry_index, entry) in entries.iter().enumerate() {
    // dbg!(entry);
    let possibilities: HashMap<char, Vec<char>> = make_possibilities(&entry.patterns);
    // dbg!(&possibilities);

    let mapping = find_valid_mapping(&entry, &possibilities).expect("no valid mapping found!");
    // print_mapping(&mapping);

    let output = unscramble_and_decode_output(&entry, &mapping);
    println!("{}: {}", entry.outputs_str, output);
    outputs_sum += output;
  }

  println!("{}", outputs_sum);
}

fn find_valid_mapping(
  entry: &Entry,
  possibilities: &HashMap<char, Vec<char>>,
) -> Option<HashMap<char, char>> {
  let mut have_complete_mapping = true;
  let mut mapping: HashMap<char, char> = HashMap::new();
  for (src, target_possibilities) in possibilities {
    if target_possibilities.len() == 1 {
      mapping.insert(*src, target_possibilities[0]);
    } else {
      have_complete_mapping = false;
      break;
    }
  }
  if have_complete_mapping {
    if test_mapping(entry, &mapping) {
      return Some(mapping);
    }
  }

  for (src, target_possibilities) in possibilities {
    if target_possibilities.len() > 1 {
      for target in target_possibilities {
        let mut new_possibilities: HashMap<char, Vec<char>> = possibilities.clone();
        new_possibilities.insert(*src, vec![*target]);
        let mapping_opt = find_valid_mapping(entry, &new_possibilities);
        if mapping_opt.is_some() {
          return mapping_opt;
        }
      }
    }
  }

  return None;
}

fn test_mapping(entry: &Entry, mapping: &HashMap<char, char>) -> bool {
  let mut valid = true;
  for pattern in &entry.patterns {
    let segments = unscamble(&pattern, &mapping);
    let digit = decode(&segments);
    valid &= digit.is_some();
    if !valid {
      break;
    }
    // println!("{:?} => {:?} => {}", pattern, segments, digit.unwrap());
  }
  return valid;
}

fn print_mapping(mapping: &HashMap<char, char>) {
  let mut display = String::new();
  display.push(*mapping.get(&'a').unwrap());
  display.push(*mapping.get(&'b').unwrap());
  display.push(*mapping.get(&'c').unwrap());
  display.push(*mapping.get(&'d').unwrap());
  display.push(*mapping.get(&'e').unwrap());
  display.push(*mapping.get(&'f').unwrap());
  display.push(*mapping.get(&'g').unwrap());
  println!("mapping: {}", display);
}

fn make_possibilities(patterns: &Vec<Vec<char>>) -> HashMap<char, Vec<char>> {
  let mut possibilities: HashMap<char, Vec<char>> = HashMap::from([
    ('a', Vec::new()),
    ('b', Vec::new()),
    ('c', Vec::new()),
    ('d', Vec::new()),
    ('e', Vec::new()),
    ('f', Vec::new()),
    ('g', Vec::new()),
  ]);

  // find pattern for digit 1 (cf) and set possibilities
  let mut pattern_digit_1: &Vec<char> = &Vec::new();
  for pattern in patterns {
    if pattern.len() == 2 {
      pattern_digit_1 = pattern;
      for c in pattern {
        possibilities.insert(*c, vec!['c', 'f']);
      }
    }
  }

  // find pattern for digit 7 (acf) and set single possibility
  for pattern in patterns {
    if pattern.len() == 3 {
      for c in pattern {
        if !pattern_digit_1.contains(c) {
          possibilities.insert(*c, vec!['a']);
        }
      }
    }
  }

  // find pattern for digit 4 (bcdf) and set possibilities
  for pattern in patterns {
    if pattern.len() == 4 {
      for c in pattern {
        if possibilities.get(c).unwrap().len() == 0 {
          possibilities.insert(*c, vec!['b', 'd']);
        }
      }
    }
  }

  // find 2 remaining patterns that aren't set yet
  for pattern in patterns {
    for c in pattern {
      if possibilities.get(c).unwrap().len() == 0 {
        possibilities.insert(*c, vec!['e', 'g']);
      }
    }
  }

  return possibilities;
}

fn unscramble_and_decode_output(entry: &Entry, mapping: &HashMap<char, char>) -> u32 {
  let mut output = String::new();
  for output_digit in &entry.outputs {
    let segments = unscamble(&output_digit, mapping);
    let digit = decode(&segments);
    // println!("{:?} => {:?} => {}", output_digit, segments, digit.unwrap());
    output.push_str(&digit.unwrap().to_string())
  }
  let output: u32 = output.parse().unwrap();
  return output;
}

fn unscamble(scrambled: &Vec<char>, mapping: &HashMap<char, char>) -> Vec<char> {
  let mut unscrambled: Vec<char> = scrambled.iter().map(|c| *mapping.get(c).unwrap()).collect();
  unscrambled.sort();
  return unscrambled;
}

fn decode(segments: &Vec<char>) -> Option<u8> {
  let segments_to_digits: HashMap<&str, u8> = HashMap::from([
    ("abcefg", 0),
    ("cf", 1),
    ("acdeg", 2),
    ("acdfg", 3),
    ("bcdf", 4),
    ("abdfg", 5),
    ("abdefg", 6),
    ("acf", 7),
    ("abcdefg", 8),
    ("abcdfg", 9),
  ]);

  let segments = String::from_iter(segments);
  return segments_to_digits.get(&*segments).map(|d| *d);
}

fn parse(lines: &Vec<&str>) -> Vec<Entry> {
  let mut entries: Vec<Entry> = Vec::new();
  for line in lines {
    let parts: Vec<&str> = line.split(" | ").collect();

    let entry = Entry {
      patterns: parse_patterns(parts[0]),
      outputs: parse_patterns(parts[1]),
      outputs_str: String::from(parts[1]),
    };
    entries.push(entry);
  }

  return entries;
}

fn parse_patterns(patterns: &str) -> Vec<Vec<char>> {
  return patterns
    .split(' ')
    .map(|pattern| {
      let mut chars: Vec<char> = pattern.chars().collect();
      chars.sort();
      return chars;
    })
    .collect();
}
