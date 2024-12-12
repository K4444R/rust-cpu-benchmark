pub fn bubble_sort(n: u128) -> Vec<u128> {
    if n > usize::MAX as u128 {
        panic!("The size of the array exceeds the limit of usize::MAX");
    }

    let mut arr: Vec<u128> = (0..n).collect();
    let mut swapped = true;

    while swapped {
        swapped = false;
        for i in 0..arr.len() - 1 {
            if arr[i] > arr[i + 1] {
                arr.swap(i, i + 1);
                swapped = true;
            }
        }
    }
    arr
}
