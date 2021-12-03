pub fn main() {
    let mut total_increased = 0;

    // let lines = include_str!("./sample.txt").lines();
    let lines = include_str!("./input.txt").lines();
    let mut prev: i32 = -1;
    for line in lines {
        let depth: i32 = line.parse().expect("not a number");
        if prev > -1 && depth > prev {
            total_increased += 1;
        }
        prev = depth;
    }

    print!("{}\n", total_increased);
}
