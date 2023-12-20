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

#[cfg(test)]
mod polynomial_tests {
    use crate::polynomial::Polynomial;

    #[test]
    fn plug_in_correct() {
        let p = Polynomial::new(&[1,1,1]);  // x^2 + x + 1
        assert_eq!(p.plug_in(0), 1);
        assert_eq!(p.plug_in(1), 3);
        assert_eq!(p.plug_in(2), 7);
    }
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
    pub fn plug_in(&self, x_val: isize) -> isize {
        let mut result: isize = 0;
        for i in 0..self.degree+1 {
            let power_of_x = x_val.pow(i.try_into().unwrap());
            result += self.coefficients[i] * power_of_x;
        }
        result
    }
}
