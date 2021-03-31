use core::f64;

#[derive(Debug)]
pub struct DivisionByZeroError;

pub fn perform(diviend: f64, divisor: f64) -> Result<f64, DivisionByZeroError> {
    if divisor == 0.0 {
        Err(DivisionByZeroError)
    } else {
        Ok(diviend / divisor)
    }
}
