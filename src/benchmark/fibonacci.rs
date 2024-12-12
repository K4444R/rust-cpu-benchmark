pub fn fibonacci(n: u128) -> u128 {
    if n <= 1 {
        return n;
    }
    
    let mut a: u128 = 0; 
    let mut b: u128 = 1; 

    for _ in 2..=n {

        let temp = a.checked_add(b); 
        
        match temp {
            Some(val) => {
                a = b;
                b = val;
            }
            None => {
                panic!("Overflow occurred during Fibonacci calculation for n={}", n);
            }
        }
    }

    b
}
