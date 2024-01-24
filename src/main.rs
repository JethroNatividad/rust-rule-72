// Write a program that takes in rate of return and outputs the estimate years on how long it will take to double your investment.
// Inputs: rate of return
// Process: 72/rate of return
// Output: It will take n years to double your initial investment.

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_return_investment_years() {
        assert_eq!(get_return_investment_years(4.0), 18.0);
        assert_eq!(get_return_investment_years(10.0), 7.2);

        #[should_panic]
        get_return_investment_years(0.0);
    }
}

fn main() {
    println!("Hello, world!");
}
