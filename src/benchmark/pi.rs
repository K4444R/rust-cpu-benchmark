pub fn pi_approximation(iterations: u64) -> Option<f64> {
    let mut pi = 0.0;

    for i in 0..iterations {

        let denominator = (1.0 + (i * 2) as f64).powi(2);
        if denominator == 0.0 {

            return None;
        }


        let addition = 1.0 / denominator;
        if addition.is_infinite() || addition.is_nan() {
            return None; 
        }

        pi += addition;
    }

   
    let pi_sqrt = (pi * 8.0).sqrt();
    if pi_sqrt.is_infinite() || pi_sqrt.is_nan() {
        return None;
    }

    Some(pi_sqrt)
}
