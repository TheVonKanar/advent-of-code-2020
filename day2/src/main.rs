fn main() {    
    let input: &str = include_str!("../input.txt");
    println!("Part 1 => {}", part1(input));
    println!("Part 2 => {}", part2(input));
}

fn part1(input: &str) -> u32 {
    let mut result = 0;
    for line in input.lines() {
        let data: Vec<&str> = line.split(' ').collect();
        let range: Vec<usize> = data[0].split('-').map(|i| i.parse().unwrap()).collect(); 
        let char_count =  data[2].matches(data[1].chars().next().unwrap()).count();
        if char_count >= range[0] && char_count <= range[1] {
            result += 1;
        }
    }

    result
}

fn part2(input: &str) -> u32 {
    let mut result = 0;
    for line in input.lines() {
        let data: Vec<&str> = line.split(' ').collect();
        let letter: char = data[1].chars().next().unwrap();
        let mut match_count = 0;        
        for pos in data[0].split('-').map(|i| i.parse::<usize>().unwrap()) {
            if data[2].chars().nth(pos - 1).unwrap() == letter {
                match_count += 1;
            }
        }
        
        if match_count == 1 {
            result += 1;
        }
    }

    result
}