use super::factorial;


pub fn factorial_sum(n: u128) -> u128 {
    (1..=n).map(|x| factorial::factorial(x)).sum::<u128>()
}
