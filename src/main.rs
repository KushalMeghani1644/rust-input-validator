use std::io;
fn main() {
    let mut numbers: Vec<i32> = Vec::new();

    println!("=== Number Collection Program ===");

    loop {
        println!("\nHow many numbers would you like to enter: ");
        match get_valid_input() {
            Some(count) if count > 0 => {
                collect_numbers(&mut numbers, count);
                break;
            }
            Some(_) => println!("Please enter a posistive number!"),
            None => continue,
        }
    }
    display_result(&numbers);
}
fn get_valid_input() -> Option<i32> {
    loop {
        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(_) => match input.trim().parse::<i32>() {
                Ok(num) => return Some(num),
                Err(_) => {
                    println!("That's not a valid number! please try again:");
                    continue;
                }
            },
            Err(_) => {
                println!("failed to read input! please try again!");
                continue;
            }
        }
    }
}
fn get_valid_input_with_bounds(min: i32, max: i32) -> i32 {
    loop {
        match get_valid_input() {
            Some(num) if num >= min && num <= max => return num,
            Some(num) => println!("Number must be between {} and {}", min, max),
            None => continue,
        }
    }
}
fn collect_numbers(numbers: &mut Vec<i32>, count: i32) {
    for i in 1..=count {
        println!("Enter a number {} between (-1000 and 1000): ", i);
        let number = get_valid_input_with_bounds(-1000, 1000);
        numbers.push(number);
        println!("Added {} to the collection!", number);
    }
}
fn display_result(numbers: &Vec<i32>) {
    println!("\n=== RESULT ===");
    println!("Numbers collected: {:?}", numbers);
    println!("Total count: {}", numbers.len());
    if !numbers.is_empty() {
        let sum: i32 = numbers.iter().sum();
        let average = sum as f64 / numbers.len() as f64;
        println!("sum: {}", sum);
        println!("Average: {:.2}", average);
        println!("Minimum: {}", numbers.iter().min().unwrap());
        println!("Maximum: {}", numbers.iter().max().unwrap());
    }
}
