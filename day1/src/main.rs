fn main() {
    let input: Vec<i64> = include_str!("../input.txt").lines().map(|i| i.parse().unwrap()).collect();
    
    // Part 1
    'part1: for n1 in &input {
        for n2 in &input {
            if n1 + n2 == 2020 {
                println!("Part 1 result: {}", n1 * n2);
                break 'part1;
            }
        }
    }

    // Part 2
    'part2: for n1 in &input {
        for n2 in &input {
            for n3 in &input {
                if n1 + n2 + n3 == 2020 {
                    println!("Part 2 result: {}", n1 * n2 * n3);
                    break 'part2;
                }
            }
        }
    }
}
