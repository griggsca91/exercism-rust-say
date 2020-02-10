use std::env;


fn num_to_english(number: &str) -> &str {
    match number {
        "0" => "",
        "1" => "one",
        "2" => "two",
        "3" => "three",
        "4" => "four",
        "5" => "five",
        "6" => "six",
        "7" => "seven",
        "8" => "eight",
        "9" => "nine",
        "10" => "ten",
        "11" => "eleven",
        "12" => "twelve",
        "13" => "thirteen",
        "14" => "fourteen",
        "15" => "fifteen",
        "16" => "sixteen",
        "17" => "seventeen",
        "18" => "eighteen",
        "19" => "nineteen",
        _ => "err"
    }
}

fn place_value_to_english(place: &i32) -> &str {
    match place {
        1 => "hundred",
        2 => "thousand",
        3 => "million",
        4 => "billion",
        5 => "trillion",
        _ => ""
    }
}

fn help() {
    println!("usage:
num_to_english <integer>
    Converts the integer to English
    ");
}

fn main() {
    let args: Vec<String> = env::args().collect();

    match args.len() {
        2 => {
            if !is_valid_integer(&args[1]) {
                help();
                return
            }

            process_number(&args[1]);
        }
        _ => {
            help();
        }
    }
}

fn is_valid_integer(number: &String) -> bool {
    let my_int = number.parse::<i32>();

    !my_int.is_err()
}

fn process_number(number: &String) {
    println!("{}", number);

    let mut groups: Vec<Vec<String>> = Vec::new(); 
    for (i, c) in number.chars().rev().enumerate() {
        if i % 3 == 0 {
           groups.push(Vec::new());         
        }
        println!("{}", i/3);
        let group = &mut groups[i/3];
        group.push(c.to_string());
    }

    let mut count = 1;
    for group in &mut groups {
        group.reverse();

        println!("place: {}", place_value_to_english(&count));

        println!("{:?}", group);
        count += 1;
    }

    println!("{:?}", groups);
}
