fn main() {
    let input: &str = include_str!("../input.txt");
    println!("Part 1 => {}", part1(input));
    println!("Part 2 => {}", part2(input));
}

fn part1(input: &str) -> u32 {
    let mut result = 0;
    for pass in input.lines()
    {
        let mut r = (0f32, 127f32);
        let mut c = (0f32, 7f32);
        for row in pass.chars() {
            match row {
                'F' => r.1 -= ((r.1 - r.0) * 0.5).ceil(),
                'B' => r.0 += ((r.1 - r.0) * 0.5).ceil(),
                'L' => c.1 -= ((c.1 - c.0) * 0.5).ceil(),
                'R' => c.0 += ((c.1 - c.0) * 0.5).ceil(),
                _ => ()
            }            
        }

        let id =  (r.0 * 8f32 + c.0) as u32;
        if id > result { result = id; }
    }

    result
}

fn part2(input: &str) -> u32 {
    let mut result = 0;
    let mut ids: Vec<u32> = input.lines().map(|pass| {
        let mut r = (0f32, 127f32);
        let mut c = (0f32, 7f32);
        for row in pass.chars() {
            match row {
                'F' => r.1 -= ((r.1 - r.0) * 0.5).ceil(),
                'B' => r.0 += ((r.1 - r.0) * 0.5).ceil(),
                'L' => c.1 -= ((c.1 - c.0) * 0.5).ceil(),
                'R' => c.0 += ((c.1 - c.0) * 0.5).ceil(),
                _ => ()
            }            
        }

        (r.0 * 8f32 + c.0) as u32
    }).collect();
    
    ids.sort_by(|a, b| a.partial_cmp(b).unwrap());

    let mut prev = 0;
    for id in ids {        
        if id != prev + 1 {
            result = id - 1;
        }
        
        prev = id;
    }

    result
}