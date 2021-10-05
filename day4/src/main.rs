use regex::Regex;

fn main() {
    let input = include_str!("../input.txt");
    let mut result = 0; 
    
    // Part 1.
    for raw in input.split("\r\n\r\n") {
        let pass = raw.replace("\r\n", " ");
        let count = pass.split(' ').count();
        if count == 8 || (count == 7 && !pass.contains("cid")) {
            result += 1;
        } 
    }

    println!("Part 1 result: {}", result);

    // Part 2.
    result = 0; 
    for raw in input.split("\r\n\r\n") {
        let pass = raw.replace("\r\n", " ");
        let mut valid_elem_count = 0;
        for elem in pass.split(' ') {
            let kvp: Vec<&str> = elem.split(':').collect();
            if match kvp[0] {
                "byr" => (1920..=2002).contains(&kvp[1].parse().unwrap_or_default()),
                "iyr" => (2010..=2020).contains(&kvp[1].parse().unwrap_or_default()),
                "eyr" => (2020..=2030).contains(&kvp[1].parse().unwrap_or_default()),
                "hgt" => {
                    let pair: (&str, &str) = kvp[1].split_at(kvp[1].len() - 2);
                    match pair.1 {
                        "cm" => (150..=193).contains(&pair.0.parse().unwrap_or_default()),
                        "in" => (59..=76).contains(&pair.0.parse().unwrap_or_default()),
                        _ => false
                    }
                },
                "hcl" => Regex::new(r"^#(?:[0-9a-f]{6})$").unwrap().is_match(kvp[1]),
                "ecl" => matches!(kvp[1], "amb" | "blu" | "brn" | "gry" | "grn" | "hzl" | "oth"),
                "pid" => kvp[1].len() == 9 && kvp[1].parse::<i32>().is_ok(),
                "cid" => true,
                _ => false
            } { 
                valid_elem_count += 1 
            }
        }
        
        if valid_elem_count == 8 || (valid_elem_count == 7 && !pass.contains("cid")) {
            result += 1;
        } else { 
        }
    }
    
    println!("Part 2 result: {}", result);
}
