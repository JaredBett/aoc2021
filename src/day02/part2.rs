pub fn main() {
    let mut horiz_pos = 0;
    let mut depth = 0;
    let mut aim = 0;

    let lines = include_str!("input.txt").lines();
    for line in lines {
        let parts: Vec<&str> = line.split(' ').collect();
        let cmd = parts[0];
        let arg: i32 = parts[1].parse().expect("invalid num");
        if cmd == "forward" {
            horiz_pos += arg;
            depth += aim * arg;
        } else if cmd == "down" {
            aim += arg;
        } else if cmd == "up" {
            aim -= arg;
        }
    }

    println!("{}", horiz_pos * depth);
}
