pub fn clamp_over(n: usize, max: usize, min: usize) -> usize {
    if n > max {
        return min;
    }
    if n < min {
        return max;
    }
    return n;
}

pub trait Len {
    fn len(&self) -> usize;
}

pub fn out_of_bounds<T>(x: isize, arr: &[T]) -> bool {
    x >= arr.len() as isize || x < 0
}
