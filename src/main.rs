use std::io;

fn main() {
    println!("Convert strings to Pig Latin!");

    'main: loop {
        println!("Please enter your string (in lowercase): ");

        let mut input_str = String::new();

        io::stdin()
            .read_line(&mut input_str)
            .expect("Failed to read line");

        let input_str = input_str.trim();

        if input_str.len() < 2 {
            println!("String too short!");
            continue 'main;
        }

        let vowels: [char; 5] = ['a', 'e', 'i', 'o', 'u'];
        let letters: [char; 26] = [
            'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q',
            'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z',
        ];
        let mut is_first_vowel: bool = false;
        let first_consonant: char = input_str.chars().next().unwrap();

        'outer: for ch in input_str.chars() {
            for letter in letters {
                if ch != letter {
                    continue;
                } else {
                    continue 'outer;
                }
            }
            println!("Only lowercase letters from the English alphabets allowed!");
            continue 'main;
        }

        for vowel in vowels {
            if vowel == first_consonant {
                is_first_vowel = true;
                break;
            }
        }

        let output_str: String = if is_first_vowel {
            format!("{input_str}-hay")
        } else {
            let modified_str: &str = &input_str[1..input_str.len()];
            format!("{modified_str}-{first_consonant}ay")
        };

        println!("Pig Latin: {output_str}");
        break;
    }
}
