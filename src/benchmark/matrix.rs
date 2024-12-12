pub fn matrix_multiplication(size: usize) -> Vec<Vec<u128>> {
    let mut result = vec![vec![0; size]; size];

    for i in 0..size {
        for j in 0..size {
            match (i as u128).checked_mul(j as u128) {
                Some(value) => result[i][j] = value,
                None => {
                    panic!("Overflow occurred during matrix multiplication at indices ({}, {})", i, j);
                }
            }
        }
    }

    result
}
