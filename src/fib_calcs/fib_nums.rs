use std::vec::Vec;
use pyo3::prelude::*;
use super::fib_num::fibonacci_number;

#[pyfunction]
pub fn fibonacci_numbers(nums: Vec<i32>) -> Vec<u64> {
    let mut res = Vec::with_capacity(nums.len());

    for n in nums.iter() {
        res.push(fibonacci_number(*n));
    }

    res
}