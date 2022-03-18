use pyo3::prelude::*;

#[pyfunction]
pub fn fibonacci_number(n: i32) -> u64 {
    if n < 0 {
        panic!("{n} is negative!");
    }
    match n {
        0 => panic!("zero is not a right argument to fibonacci_number"),
        1 | 2 => 1,
        _ => fibonacci_number(n-1) + fibonacci_number(n-2)
    }
}