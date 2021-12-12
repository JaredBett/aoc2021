use std::collections::HashMap;

pub fn main() {
  let mut caves: HashMap<&str, Vec<&str>> = HashMap::new();
  for line in include_str!("input.txt").lines() {
    let mut parts = line.split('-');
    let cave_a = parts.next().unwrap();
    let cave_b = parts.next().unwrap();

    if !caves.contains_key(cave_a) {
      caves.insert(cave_a, Vec::new());
    }
    if !caves.contains_key(cave_b) {
      caves.insert(cave_b, Vec::new());
    }

    caves.get_mut(cave_a).unwrap().push(cave_b);
    caves.get_mut(cave_b).unwrap().push(cave_a);
  }

  // dbg!(&caves);

  let paths = find_paths(&vec!["start"], &caves);
  // for path in &paths {
  //   println!("{}", path.join(","));
  // }
  // dbg!(&paths);

  println!("{}", paths.len());
}

fn find_paths(
  existing_path: &Vec<&str>,
  remaining_caves: &HashMap<&str, Vec<&str>>,
) -> Vec<Vec<String>> {
  let mut paths: Vec<Vec<String>> = Vec::new();

  let current_cave = existing_path[existing_path.len() - 1];

  if current_cave == "end" {
    let mut final_path: Vec<String> = Vec::new();
    for path in existing_path {
      final_path.push(String::from(*path));
    }
    paths.push(final_path);
    return paths;
  }

  let next_caves = remaining_caves.get(current_cave).unwrap();

  for next_cave in next_caves {
    if can_visit(next_cave, existing_path) {
      let mut path: Vec<&str> = existing_path.clone();
      path.push(next_cave);
      for other_path in find_paths(&path, remaining_caves) {
        paths.push(other_path);
      }
    }
  }

  return paths;
}

fn can_visit(next_cave: &str, existing_path: &Vec<&str>) -> bool {
  let mut visits: HashMap<&str, u32> = HashMap::new();
  for path in existing_path {
    visits.insert(path, *visits.get(path).unwrap_or(&0) + 1);
  }
  let mut lower_visited_twice = false;
  for (cave, num) in visits {
    if num > 1 && cave == cave.to_lowercase() {
      lower_visited_twice = true;
      break;
    }
  }
  return next_cave != "start"
    && (*next_cave == next_cave.to_uppercase()
      || !lower_visited_twice
      || !existing_path.contains(&next_cave));
}
