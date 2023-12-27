use std::time::{Duration, Instant};
const EPSILON: f64 = f64::EPSILON;

fn separate_parts(value: f64) -> (i64, f64) {
    
    let whole_part = value as i64;
    let fractional_part = value - whole_part as f64;
    
    (whole_part, fractional_part)
}

fn approximate_float(input_value: f64, accuracy_score: f64) -> (i64, i64, i64) {

    let (whole_part, fractional_part) = separate_parts(input_value);

    let mut denominator = 1;
    let mut numerator = (fractional_part * denominator as f64).round() as i64;

    println!("Starting approximation");

    // Main approximation loop
    while (fractional_part - (numerator as f64 / denominator as f64)).abs() > accuracy_score {
        denominator += 1;
        numerator = (fractional_part * denominator as f64).round() as i64;
    }

    (whole_part, numerator, denominator)
}

fn main() {

    let start = Instant::now(); // Start timer
    println!("Timer started");

    let input_value = 3.14159265358979323846264338327950288419716939937510;
    let accuracy_score = 0.0000000000001;

    let (whole_part, numerator, denominator) = approximate_float(input_value, accuracy_score);

    let result = whole_part as f64 + numerator as f64 / denominator as f64;
    let mut percentage_accuracy = (input_value / result) * 100.0;
    if percentage_accuracy > 100.0 {
        percentage_accuracy = (result / input_value) * 100.0;
    }

    let duration = start.elapsed(); // stop timer
    println!("Timer stopped");
    
    println!(
        "{} approximates to {} + {}/{}, which equals {}. This approximation is {}% accurate.",
        input_value,
        whole_part,
        numerator,
        denominator,
        result,
        percentage_accuracy
    );

    println!("Time elapsed is: {:?}", duration);
    
}
