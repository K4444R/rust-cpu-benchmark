pub fn sieve_of_eratosthenes(n: u128) -> Vec<u128> {
    if n < 2 {
        return Vec::new(); 
    }

    let mut primes = vec![true; (n + 1) as usize];
    let mut result = Vec::new();

    for i in 2..=n {
        if primes[i as usize] {
            result.push(i);
            
          
            if let Some(mut j) = i.checked_mul(i) {
                while j <= n {
                    primes[j as usize] = false;
                    if let Some(new_j) = j.checked_add(i) {
                        j = new_j;
                    } else {
                        break; 
                    }
                }
            }
        }
    }

    result
}
