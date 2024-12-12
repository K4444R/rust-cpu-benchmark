pub fn prime_numbers(n: u64) -> Option<Vec<u64>> {
    if n < 2 {
        return Some(Vec::new()); 
    }

    let mut primes = Vec::new();

    for num in 2..=n {
       
        let is_prime = (2..=((num as f64).sqrt() as u64)).all(|i| {
            if i == 0 {

                return false;
            }
            num % i != 0
        });

        if is_prime {
            primes.push(num);
        }
    }

    Some(primes)
}
