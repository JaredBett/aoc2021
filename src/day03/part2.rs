pub fn main() {
    let lines = include_str!("./input.txt").lines();
    let numbers: Vec<&str> = lines.collect();
    let num_bits = numbers[0].len();
    // dbg!(num_bits);

    let mut oxy_numbers = numbers.clone();
    let oxy_rating = loop {
        for i in 0..num_bits {
            let x = most_common(&oxy_numbers, i);
            filter(&mut oxy_numbers, i, x);
            if oxy_numbers.len() == 1 {
                break;
            }
        }
        break oxy_numbers[0];
    };
    let oxy_rating = isize::from_str_radix(&oxy_rating, 2).unwrap();
    println!("oxy_rating: {}", oxy_rating);

    let mut co2_numbers = numbers.clone();
    let co2_rating = loop {
        for i in 0..num_bits {
            let x = least_common(&co2_numbers, i);
            filter(&mut co2_numbers, i, x);
            if co2_numbers.len() == 1 {
                break;
            }
        }
        break co2_numbers[0];
    };
    let co2_rating = isize::from_str_radix(&co2_rating, 2).unwrap();
    println!("co2_rating: {}", co2_rating);

    println!("{}", oxy_rating * co2_rating);
}

fn one_zero_counts(numbers: &Vec<&str>, bit_index: usize) -> (usize, usize) {
    let mut num_ones = 0;
    for num in numbers {
        if num.as_bytes()[bit_index] as char == '1' {
            num_ones += 1;
        }
    }
    let num_zeroes = numbers.len() - num_ones;
    return (num_ones, num_zeroes);
}

fn most_common(numbers: &Vec<&str>, bit_index: usize) -> char {
    let (num_ones, num_zeroes) = one_zero_counts(numbers, bit_index);
    return if num_ones >= num_zeroes { '1' } else { '0' };
}

fn least_common(numbers: &Vec<&str>, bit_index: usize) -> char {
    let (num_ones, num_zeroes) = one_zero_counts(numbers, bit_index);
    return if num_ones >= num_zeroes { '0' } else { '1' };
}

fn filter(numbers: &mut Vec<&str>, bit_index: usize, x: char) {
    numbers.retain(|&num| num.as_bytes()[bit_index] as char == x);
}
