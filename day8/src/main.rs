use regex::Regex;

fn main() {
    let input: &str = include_str!("../input.txt");    
    println!("Part 1 => {}", part1(input));
    println!("Part 2 => {}", part2(input));
}

fn part1(input: &str) -> i32 {
    let re = Regex::new(r"([a-z]{3}) ([+-]\d+)").unwrap();
    let mut index: usize = 0;
    let mut acc: i32 = 0;
    let mut hist: Vec<usize> = Vec::new();
    let lines: Vec<&str> = input.lines().collect();
    hist.push(index);
    loop {        
        let caps = re.captures(lines[index]).unwrap();
        let arg: i32 = caps[2].parse().unwrap();
        match &caps[1] {
            "acc" => {
                acc += arg; 
                index += 1;
            },
            "jmp" => index = (index as i32 + arg) as usize,
            _ => index += 1
        };

        if hist.contains(&index) || index >= lines.len() {
            break;
        } else {
            hist.push(index);
        }
    }

    acc
}

fn part2(input: &str) -> i32 {
    let re = Regex::new(r"([a-z]{3}) ([+-]\d+)").unwrap();
    let mut index: usize = 0;
    let mut acc: i32 = 0;
    let mut hist: Vec<usize> = Vec::new();
    let lines: Vec<&str> = input.lines().collect();
    hist.push(index);
    loop {        
        let caps = re.captures(lines[index]).unwrap();
        let arg: i32 = caps[2].parse().unwrap();
        match &caps[1] {
            "acc" => {
                acc += arg; 
                index += 1;
            },
            "jmp" => index = (index as i32 + arg) as usize,
            _ => index += 1
        };

        if hist.contains(&index) || index >= lines.len() {
            break;
        } else {
            hist.push(index);
        }
    }

    let highest_index = hist.iter().max().unwrap();

    acc
}