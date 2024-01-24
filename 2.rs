use rand::Rng;
use std::time::Instant;

// approximates a f64 to i16 + i8/i8 fractional format.
// i16 is the whole part, i8 is the numerator, i8 is the denominator
// generate random float inputs
// save the floats and approximations to vectors
// for each input float in the vector, check if it is smaller than i16::MAX and check for i8 overflow
// if it is, then approximate it and save the approximation to the approximations vector
// the approximation function should find the closest i8/i8 approximation to the input float. Make sure to respect the sign of the input float and stop when overflow would have happened

struct Fractional {
    whole: i16,
    num: i8,
    denom: i8,
}

fn can_approximate(input_value: f64) -> bool {
    let fractional_part = input_value.fract().abs();
    let approx = (fractional_part * (i8::MAX as f64)).round();
    (approx - approx.floor()).abs() < f64::EPSILON
}

fn separate_parts(value: f64) -> (i16, f64) {
    let whole_part = value as i16;
    let fractional_part = value - whole_part as f64;
    (whole_part, fractional_part)
}

// everything up to this point is finished

fn approximate_float(input_value: f64) -> Fractional {
    if !can_approximate(input_value) {
        panic!("Cannot approximate the input value with i16 + i8/i8 format");
    }

    let (whole_part, fractional_part) = separate_parts(input_value);

    let mut denominator = 1;
    let mut numerator = (fractional_part * denominator as f64).round() as i8;

    // Main approximation loop
    while (fractional_part - (numerator as f64 / denominator as f64)).abs() > f64::EPSILON
        && denominator <= i8::MAX as usize
    {
        denominator += 1;
        numerator = (fractional_part * denominator as f64).round() as i8;
    }

    Fractional {
        whole: whole_part as i16,
        num: numerator,
        denom: denominator as i8,
    }
}

fn main() {
    let start = Instant::now(); // Start timer
    println!("Timer started");

    let num_iterations = 10;
    let mut float_numbers: Vec<f64> = Vec::new();
    let mut approximations: Vec<Fractional> = Vec::new();

    for _ in 0..num_iterations {
        let input_value = rand::thread_rng().gen_range(-100.0..100.0);
        float_numbers.push(input_value);

        if can_approximate(input_value) {
            println!("CAN APPROXIMATE {}", input_value);
            approximations.push(approximate_float(input_value));
        } else {
            println!("CANNOT APPROXIMATE {}", input_value);
        }
    }
    println!("Finished generating/checking input floats \n");

    for (i, frac) in approximations.iter().enumerate() {
        let result = frac.whole as f64 + (frac.num as f64 / frac.denom as f64);
        let percent_error = ((result - float_numbers[i]).abs() / float_numbers[i]) * 100.0;

        println!(
            "Input: {}, Approximation: {} + {}/{}, Result: {}, Percent Error: {}%",
            float_numbers[i],
            frac.whole,
            frac.num,
            frac.denom,
            result,
            percent_error
        );
    }

    let duration = start.elapsed(); // Stop timer
    println!("Timer stopped");
    println!("Time elapsed is: {:?}", duration);
}
