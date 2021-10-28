fn main() {    
    let input: &str = include_str!("../input.txt");
    println!("Part 1 => {}", part1(input));
    println!("Part 2 => {}", part2(input));
}

fn count(input: &str, x_adv: usize, y_adv: usize) -> u32 {
    let mut x = 0;
    let mut result = 0;
    for (y, line) in input.lines().enumerate().skip(1)  {         
        if y % y_adv > 0 { continue };
        x = (x + x_adv) % line.len();
        if line.chars().nth(x).unwrap() == '#' {
            result += 1;
        }
    }

    result
}

fn part1(input: &str) -> u32 {
    count(input, 3, 1)
}

fn part2(input: &str) -> u32 {
    count(input, 1, 1) * count(input, 3, 1) * count(input, 5, 1) * count(input, 7, 1) * count(input, 1, 2)
}
