//! A deliberately small calculator used by the Trusted Publishing lab.

use std::fmt;

/// Arithmetic operations accepted by [`calculate`].
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Operation {
    Add,
    Subtract,
    Multiply,
    Divide,
}

/// Errors returned for invalid calculations.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CalculatorError {
    DivisionByZero,
}

impl fmt::Display for CalculatorError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::DivisionByZero => formatter.write_str("division by zero is not allowed"),
        }
    }
}

impl std::error::Error for CalculatorError {}

/// Calculates `left <operation> right`.
pub fn calculate(left: f64, operation: Operation, right: f64) -> Result<f64, CalculatorError> {
    match operation {
        Operation::Add => Ok(left + right),
        Operation::Subtract => Ok(left - right),
        Operation::Multiply => Ok(left * right),
        Operation::Divide if right == 0.0 => Err(CalculatorError::DivisionByZero),
        Operation::Divide => Ok(left / right),
    }
}

#[cfg(test)]
mod tests {
    use super::{CalculatorError, Operation, calculate};

    #[test]
    fn performs_supported_operations() {
        assert_eq!(calculate(7.0, Operation::Add, 5.0), Ok(12.0));
        assert_eq!(calculate(7.0, Operation::Subtract, 5.0), Ok(2.0));
        assert_eq!(calculate(7.0, Operation::Multiply, 5.0), Ok(35.0));
        assert_eq!(calculate(10.0, Operation::Divide, 4.0), Ok(2.5));
    }

    #[test]
    fn rejects_division_by_zero() {
        assert_eq!(
            calculate(1.0, Operation::Divide, 0.0),
            Err(CalculatorError::DivisionByZero)
        );
    }
}
