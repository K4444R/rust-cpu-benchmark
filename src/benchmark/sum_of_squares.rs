fn fibonacci_mod_10(n: u128) -> u128 {
    // Pisano period for modulo 10 is 60
    let pisano_period = 60;
    let n = n % pisano_period;

    if n == 0 {
        return 0;
    } else if n == 1 {
        return 1;
    }

    let mut previous = 0;
    let mut current = 1;

    for _ in 2..=n {
        let new_current = (previous + current) % 10;
        previous = current;
        current = new_current;
    }

    current
}

pub fn sum_of_squares(n: u128) -> u128 {
    // Compute F(n) and F(n+1) mod 10
    let f_n = fibonacci_mod_10(n);
    let f_n_plus_1 = fibonacci_mod_10(n + 1);
    
    // Return (F(n) * F(n+1)) mod 10
    (f_n * f_n_plus_1) % 10
}
