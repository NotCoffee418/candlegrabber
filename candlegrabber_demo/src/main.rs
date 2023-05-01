use std::io;
use candlegrabber::binance;
use candlegrabber::validation::validate_duration_str;


fn main() {
    // Get user pair
    println!("Enter a pair: ");
    let pair = get_pair_input();

    // Get user duration
    println!("Enter a duration: ");
    let duration = get_duration_input();

    // Call library and collect data
    println!("Collecting data for {} {}. This will take several minutes.", pair, duration);
    binance::grab_data_binance(&pair, &duration);

    // Hang app until manual close
    println!("\nPress ENTER to close window.");
    io::stdin().read_line(&mut String::new()).unwrap();
}

fn get_pair_input() -> String {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    let input = input.trim();
    input.chars()
        .filter(|c| c.is_alphanumeric())
        .collect()
}

fn get_duration_input() -> String {
    let mut input = String::new();
    loop {
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        let input = input.trim();
        if validate_duration_str(input) {
            return input.to_string();
        }
    }
}