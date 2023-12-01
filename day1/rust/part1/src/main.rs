use std::fs;

const INPUT: &str = "../../input.txt";

fn main() {
    let file = fs::read_to_string(INPUT).unwrap();

    let mut codes: Vec<(u32, u32)> = Vec::new();

    for (i, line) in file.lines().into_iter().enumerate() {
        for character in line.chars().into_iter() {
            if character.is_numeric() {
                if codes.get(i).is_some() {
                    codes[i].1 = character.to_digit(10).unwrap();
                } else {
                    // If the slot doesn't exist, we have the first number.
                    codes.push((character.to_digit(10).unwrap(), 11));
                }
            }
        }

        if codes[i].1 == 11 {
            codes[i].1 = codes[i].0;
        }
    }

    let mut final_num = 0;

    for code in codes {
        let full_num: u32 = format!("{}{}", code.0, code.1).parse().unwrap();
        final_num += full_num;
    }

    println!("{:?}", final_num);
}
