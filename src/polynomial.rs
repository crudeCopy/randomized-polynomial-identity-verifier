/**
 * A struct describing a polynomial.
 **/

/* Polynomials are here represented
 * as a vector of their coefficients,
 * with a degree value for convenience. */
pub struct Polynomial {
    degree: usize,
    coefficients: Vec<isize>,
}

impl Polynomial {
    pub fn new(in_coefficients: &[isize]) -> Polynomial {
        let mut coefficients: Vec<isize> = Vec::new();
        coefficients.extend_from_slice(in_coefficients);
        Polynomial {
            degree: in_coefficients.len() - 1,
            coefficients: coefficients,
        }
    }
}
