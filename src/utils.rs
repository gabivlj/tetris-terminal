pub fn clamp_over(n: usize, max: usize, min: usize) -> usize {
    if n > max {
        return min;
    }
    if n < min {
        return max;
    }
    return n;
}

pub fn out_of_bounds<T>(x: isize, arr: &[T]) -> bool {
    x >= arr.len() as isize || x < 0
}

pub fn swap<T: Copy + Sized>(arr: &mut [T], i: usize, j: usize) {
    let t = arr[i];
    arr[i] = arr[j];
    arr[j] = t;
}
