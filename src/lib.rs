use pyo3::prelude::*;
use pyo3::wrap_pyfunction;

mod fib_calcs;

use fib_calcs::fib_num::fibonacci_number;
use fib_calcs::fib_nums::fibonacci_numbers;


#[pyfunction]
fn say_hello() {
    println!("saying hello from Rust!");
}

#[pymodule]
fn fib_rs(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_wrapped(wrap_pyfunction!(say_hello));
    m.add_wrapped(wrap_pyfunction!(fibonacci_number));
    m.add_wrapped(wrap_pyfunction!(fibonacci_numbers));
    Ok(())
}
