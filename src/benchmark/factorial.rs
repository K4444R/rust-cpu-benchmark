pub fn factorial(num: u128) -> u128 {
    let mut result: u128 = 1;
    
    for i in 1..=num {
        match result.checked_mul(i) {
            Some(val) => result = val,
            None => {
                // Обработка переполнения
                panic!("Overflow occurred while calculating factorial for {}", num);
            }
        }
    }
    
    result
}