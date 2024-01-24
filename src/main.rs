use std::io;
use std::io::Write;

// Write a program that takes in rate of return and outputs the estimate years on how long it will take to double your investment.
// Inputs: rate of return
// Process: 72/rate of return
// Output: It will take n years to double your initial investment.

fn get_return_investment_years(rate: f64) -> Result<f64, &'static str> {
    if rate < 1.0 {
        return Err("Invalid Rate");
    }
    let years: f64 = 72.0 / rate;
    Ok(years)
}

#[cfg(test)]
mod tests {
    use super::get_return_investment_years;

    #[test]
    fn test_get_return_investment_years() {
        assert_eq!(get_return_investment_years(4.0), Ok(18.0));
        assert_eq!(get_return_investment_years(10.0), Ok(7.2));
        assert_eq!(get_return_investment_years(0.0), Err("Invalid Rate"));
    }
}

fn get_input<T: std::str::FromStr>(prompt: &str) -> T {
    loop {
        print!("{}", prompt);
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");

        match input.trim().parse() {
            Ok(value) => break value,
            Err(_) => println!("Invalid input. Please try again."),
        }
    }
}

fn main() {
    println!("Hello, world!");
}
