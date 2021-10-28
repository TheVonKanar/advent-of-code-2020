fn main() {
    let input: Vec<u32> = include_str!("../input.txt").lines().map(|i| i.parse().unwrap()).collect();
    println!("Part 1 => {}", part1(&input));
    println!("Part 2 => {}", part2(&input));
}

fn part1(input: &[u32]) -> u32 {
    let mut result = 0;
    'main: for n1 in input {
        for n2 in input {
            if n1 + n2 == 2020 {
                result = n1 * n2;
                break 'main;
            }
        }
    }

    result
}

fn part2(input: &[u32]) -> u32 {
    let mut result = 0;
    'main: for n1 in input {
        for n2 in input {
            for n3 in input {
                if n1 + n2 + n3 == 2020 {
                    result = n1 * n2 * n3;
                    break 'main;
                }
            }
        }
    }

    result
}