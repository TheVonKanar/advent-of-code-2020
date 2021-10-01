fn main() {    
    let input: Vec<&str> = include_str!("../input.txt").lines().collect();
    
    // Part 1
    let mut valid_pwd_count = 0;
    for line in &input {
        let data: Vec<&str> = line.split(' ').collect();
        let range: Vec<usize> = data[0].split('-').map(|i| i.parse().unwrap()).collect(); 
        let char_count =  data[2].matches(data[1].chars().next().unwrap()).count();
        if char_count >= range[0] && char_count <= range[1] {
            valid_pwd_count += 1;
        }
    }

    println!("Part 1 result: {}", valid_pwd_count);

    // Part 2
    let mut valid_pwd_count = 0;
    for line in &input {
        let data: Vec<&str> = line.split(' ').collect();
        let letter: char = data[1].chars().next().unwrap();
        let mut match_count = 0;        
        for pos in data[0].split('-').map(|i| i.parse::<usize>().unwrap()) {
            if data[2].chars().nth(pos - 1).unwrap() == letter {
                match_count += 1;
            }
        }
        
        if match_count == 1 {
            valid_pwd_count += 1;
        }
    }

    println!("Part 2 result: {}", valid_pwd_count);
}
