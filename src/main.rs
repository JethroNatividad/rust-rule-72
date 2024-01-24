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
    use super::*;

    #[test]
    fn test_get_return_investment_years() {
        assert_eq!(get_return_investment_years(4.0), Ok(18.0));
        assert_eq!(get_return_investment_years(10.0), Ok(7.2));
        assert_eq!(get_return_investment_years(0.0), Err("Invalid Rate"));
    }
}

fn main() {
    println!("Hello, world!");
}
