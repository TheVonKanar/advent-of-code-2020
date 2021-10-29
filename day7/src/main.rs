use regex::Regex;

fn main() {
    let input: &str = include_str!("../input.txt");    
    println!("Part 1 => {}", part1(input));
    println!("Part 2 => {}", part2(input));
}

fn part1(input: &str) -> u32 {
    fn check_bags(input: &str, bag: &str, valid_bags: &mut Vec<String>) {
        let re = Regex::new(&format!(r"(.*) bags contain.*{}", bag)).unwrap();
        for capture in re.captures_iter(input) {
            let bag = String::from(&capture[1]);
            if !valid_bags.contains(&bag) {
                valid_bags.push(bag);
            }
            
            check_bags(input, &capture[1], valid_bags)
        }
    }
    
    let mut valid_bags: Vec<String> = Vec::new();
    check_bags(input, "shiny gold", &mut valid_bags);
    valid_bags.len() as u32
}

fn part2(input: &str) -> u32 {
    let mut result = 0;
    fn count_bags(input: &str, bag: &str, mult: u32, result: &mut u32) {        
        let content_re = Regex::new(&format!(r"{} bags contain (.*)", bag)).unwrap();
        for content in content_re.captures_iter(input) {
            let bag_sets_re = Regex::new(r"([1-9]) (\w+ \w+) bag").unwrap();
            for bag_set in bag_sets_re.captures_iter(&content[1]) {
                let count = bag_set[1].parse::<u32>().unwrap() * mult;
                *result += count;
                count_bags(input, &bag_set[2], count, result);
            }
        }
    }

    count_bags(input, "shiny gold", 1, &mut result);

    result
}