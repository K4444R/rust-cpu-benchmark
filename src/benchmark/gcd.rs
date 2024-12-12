pub fn gcd(mut a: u128, mut b: u128) -> u128 {
    if a == 0 {
        return b;
    }
    if b == 0 {
        return a;
    }

    while b != 0 {

        match a.checked_rem(b) {
            Some(rem) => {
                a = b;
                b = rem;
            }
            None => {
                panic!("Overflow occurred during GCD calculation");
            }
        }
    }

    a
}
