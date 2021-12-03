pub fn main() {
    let mut total_increased = 0;

    // let lines = include_str!("./sample.txt").lines();
    let lines = include_str!("./input.txt").lines();
    let mut prev: i32 = -1;
    let mut prev_prev: i32 = -1;
    let mut prev_sum: i32 = -1;
    for line in lines {
        let depth: i32 = line.parse().expect("not a number");
        if prev > -1 && prev_prev > -1 {
            let sum = depth + prev + prev_prev;
            if prev_sum > -1 && sum > prev_sum {
                total_increased += 1;
            }
            prev_sum = sum;
        }
        prev_prev = prev;
        prev = depth;
    }

    print!("{}\n", total_increased);
}
