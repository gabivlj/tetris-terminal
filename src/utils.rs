pub fn clamp_over(n: isize, max: isize, min: isize) -> isize {
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
