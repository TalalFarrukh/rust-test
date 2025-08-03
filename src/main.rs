use std::io;

fn main() {
    let text_three = String::from("RUST!");
    let new_text_three = process_word(&text_three);
    println!("{}", new_text_three);

    // let mut text_two = String::from("POTATO!");
    // append_exclamation(&mut text_two);
    // println!("{}", text_two);

    // let text = String::from("hi");
    // let new_text = repeat_string(text);
    // println!("{}", new_text);

    // guessing_game();

    // let array = [3,2,6,4,5,8,2];
    // let slice_of_array = &array[..];
    // let result = min_max_avg(slice_of_array);
    // println!("{:?}", result);

    // let person_1: (&str, i32, f32, bool) = ("Talal", 24, 1.88, false);
    // custom_message(person_1.0, person_1.1, person_1.2, person_1.3);

    // sum_of_squares(5);
}

fn process_word(word: &String) -> String {
    if word.is_empty() {
        return word.clone();
    }

    if &word[word.len()-1..] == "!" {
        let new_word = &word[..word.len()-1];
        let reversed_string: String = new_word.to_string().chars().rev().collect();
        return reversed_string;
    } else {
        let reversed_string: String = word.chars().rev().collect();
        return reversed_string;
    }
}

fn append_exclamation(word: &mut String) {
    let slices = &word[(word.len()-1)..word.len()];
    if slices != "!" {
        word.push_str("!");
    }
}

fn repeat_string(word: String) -> String {
    let mut new_string = String::new();
    new_string.push_str(&word);
    new_string.push_str(&word);
    new_string.push_str(&word);
    new_string
}

fn guessing_game() {
    let number = 42;
    let mut attempts = 0;
    
    let guessed = loop {
        let mut user_prompt = String::new();
        io::stdin().read_line(&mut user_prompt).expect("Failed to read user prompt");
        
        let guess: i32 = user_prompt.trim().parse().unwrap();

        attempts += 1;

        if guess == number {
            break true
        } else if attempts == 5 {
            break false
        } else {
            continue;
        }
    };

    if guessed { println!("User has guessed the number!") } else { println!("User has failed to guess the number") }
}

fn min_max_avg(numbers: &[i32]) -> Option<(i32, i32, f32)> {
    if numbers.len() == 0 {
        return None;
    }

    let mut max = 0;
    let mut sum = 0;
    let length = numbers.len();
    let mut min = numbers[0];

    for i in 0..(length) {
        if numbers[i] > max {
            max = numbers[i];
        }

        if numbers[i] < min {
            min = numbers[i];
        }

        sum = sum + numbers[i];
    }

    let length: f32 = length as f32; 
    let average: f32 = sum as f32 / length;

    return Some((max, min, average));
}

fn custom_message(name: &str, age: i32, height: f32, is_student: bool) {
    let student_check = if is_student == true {
        "are"
    } else {
        "are not"
    };


    println!("{name} is {age} years old. Their height is {height} meters and they {student_check} a student");
}

fn sum_of_squares(n: i32) {
    let mut sum = 0;
    let mut index: i32 = 0;

    let result = loop {
        sum = sum + (index.pow(2));
        
        if index == n {
            break sum
        }
        
        index += 1;
        
    };

    println!("{}", result);
}