fn main() {
    let input: &str = include_str!("../input.txt");    
    println!("Part 1 => {}", part1(input));
    println!("Part 2 => {}", part2(input));
}

fn part1(input: &str) -> u32 {
    let mut result = 0;
    for group in input.split("\n\n") {
        let mut answers: Vec<char> = Vec::new();
        for line in group.lines() {
            for letter in line.chars() {
                if !answers.contains(&letter) {
                    answers.push(letter);
                }
            }
        }

        result += answers.len() as u32;
    }

    result
}

fn part2(input: &str) -> u32 {
    let mut result = 0;
    for group in input.split("\n\n") {
        let mut answers: (Vec<char>,Vec<u32>) = (Vec::new(), Vec::new());
        let mut line_count = 0;
        for line in group.lines() {
            line_count += 1;
            for letter in line.chars() {
                if !answers.0.contains(&letter) {
                    answers.0.push(letter);
                    answers.1.push(1);
                } else {
                    answers.1[answers.0.iter().position(|&x| x == letter).unwrap()] += 1;
                }
            }
        }

        for c in answers.1 {
            if c == line_count {
                result += 1;
            }  
        }
    }

    result
}