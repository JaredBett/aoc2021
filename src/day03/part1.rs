pub fn main() {
    let mut one_counts: Vec<i32> = Vec::new();
    let mut num_lines = 0;
    let lines = include_str!("./input.txt").lines();
    for (line_index, line) in lines.enumerate() {
        num_lines += 1;
        for (i, char) in line.chars().enumerate() {
            if line_index == 0 {
                one_counts.push(0);
            }
            if char == '1' {
                one_counts[i] += 1;
            }
        }
    }

    let half_num_lines = num_lines / 2;
    let mut gamma_rate = String::new();
    let mut epsilon_rate = String::new();
    for one_count in one_counts.iter() {
        if one_count > &half_num_lines {
            gamma_rate.push('1');
            epsilon_rate.push('0');
        } else {
            gamma_rate.push('0');
            epsilon_rate.push('1');
        }
    }

    let gamma_rate = isize::from_str_radix(&gamma_rate, 2).unwrap();
    let epsilon_rate = isize::from_str_radix(&epsilon_rate, 2).unwrap();

    println!("{}", gamma_rate * epsilon_rate)
}
