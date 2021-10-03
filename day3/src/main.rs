fn main() {
    // Part 1.
    println!("Part 1 result: {}", count(3, 1));
    
    // Part 2.
    println!("Part 2 result: {}", count(1, 1) * count(3, 1) * count(5, 1) * count(7, 1) * count(1, 2));
}

fn count(x_adv: usize, y_adv: usize) -> usize {
    let mut x = 0;
    let mut res = 0;
    for (y, line) in include_str!("../input.txt").lines().enumerate().skip(1)  {         
        if y % y_adv > 0 { continue };
        x = (x + x_adv) % line.len();
        if line.chars().nth(x).unwrap() == '#' {
            res += 1;
        }
    }

    res 
}
