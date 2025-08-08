use std::collections::VecDeque;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum CalculatorError {
    #[error("Division by zero")]
    DivisionByZero,
    
    #[error("Overflow occurred")]
    Overflow,
    
    #[error("Invalid operation")]
    InvalidOperation,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Operation {
    Add(i64, i64),
    Subtract(i64, i64),
    Multiply(i64, i64),
    Divide(i64, i64),
}

impl Operation {
    pub fn execute(&self) -> Result<i64, CalculatorError> {
        match self {
            Operation::Add(a, b) => {
                a.checked_add(*b).ok_or(CalculatorError::Overflow)
            }
            Operation::Subtract(a, b) => {
                a.checked_sub(*b).ok_or(CalculatorError::Overflow)
            }
            Operation::Multiply(a, b) => {
                a.checked_mul(*b).ok_or(CalculatorError::Overflow)
            }
            Operation::Divide(a, b) => {
                if *b == 0 {
                    Err(CalculatorError::DivisionByZero)
                } else {
                    a.checked_div(*b).ok_or(CalculatorError::Overflow)
                }
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum CalculatorState {
    Ready,
    Computing,
    Error(String),
}

#[derive(Debug, Clone)]
pub struct Calculator {
    state: CalculatorState,
    history: VecDeque<Operation>,
    max_history: usize,
}

impl Calculator {
    pub fn new() -> Self {
        Self {
            state: CalculatorState::Ready,
            history: VecDeque::with_capacity(100),
            max_history: 100,
        }
    }
    
    pub fn with_max_history(max_history: usize) -> Self {
        Self {
            state: CalculatorState::Ready,
            history: VecDeque::with_capacity(max_history),
            max_history,
        }
    }
    
    pub fn add(&mut self, a: i64, b: i64) -> Result<i64, CalculatorError> {
        self.execute_operation(Operation::Add(a, b))
    }
    
    pub fn subtract(&mut self, a: i64, b: i64) -> Result<i64, CalculatorError> {
        self.execute_operation(Operation::Subtract(a, b))
    }
    
    pub fn multiply(&mut self, a: i64, b: i64) -> Result<i64, CalculatorError> {
        self.execute_operation(Operation::Multiply(a, b))
    }
    
    pub fn divide(&mut self, a: i64, b: i64) -> Result<i64, CalculatorError> {
        self.execute_operation(Operation::Divide(a, b))
    }
    
    fn execute_operation(&mut self, op: Operation) -> Result<i64, CalculatorError> {
        self.state = CalculatorState::Computing;
        
        match op.execute() {
            Ok(result) => {
                self.add_to_history(op);
                self.state = CalculatorState::Ready;
                Ok(result)
            }
            Err(e) => {
                self.state = CalculatorState::Error(e.to_string());
                Err(e)
            }
        }
    }
    
    fn add_to_history(&mut self, op: Operation) {
        if self.history.len() >= self.max_history {
            self.history.pop_front();
        }
        self.history.push_back(op);
    }
    
    pub fn history(&self) -> &VecDeque<Operation> {
        &self.history
    }
    
    pub fn state(&self) -> &CalculatorState {
        &self.state
    }
    
    pub fn clear_history(&mut self) {
        self.history.clear();
    }
    
    pub fn reset(&mut self) {
        self.state = CalculatorState::Ready;
        self.history.clear();
    }
}

impl Default for Calculator {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use quickcheck_macros::quickcheck;
    
    #[test]
    fn test_add() {
        let mut calc = Calculator::new();
        assert_eq!(calc.add(2, 3), Ok(5));
        assert_eq!(calc.add(-5, 10), Ok(5));
        assert_eq!(calc.add(0, 0), Ok(0));
    }
    
    #[test]
    fn test_subtract() {
        let mut calc = Calculator::new();
        assert_eq!(calc.subtract(10, 3), Ok(7));
        assert_eq!(calc.subtract(-5, -10), Ok(5));
        assert_eq!(calc.subtract(0, 0), Ok(0));
    }
    
    #[test]
    fn test_multiply() {
        let mut calc = Calculator::new();
        assert_eq!(calc.multiply(3, 4), Ok(12));
        assert_eq!(calc.multiply(-3, 4), Ok(-12));
        assert_eq!(calc.multiply(0, 100), Ok(0));
    }
    
    #[test]
    fn test_divide() {
        let mut calc = Calculator::new();
        assert_eq!(calc.divide(10, 2), Ok(5));
        assert_eq!(calc.divide(-10, 2), Ok(-5));
        assert!(calc.divide(10, 0).is_err());
    }
    
    #[test]
    fn test_overflow() {
        let mut calc = Calculator::new();
        assert!(calc.add(i64::MAX, 1).is_err());
        assert!(calc.multiply(i64::MAX, 2).is_err());
    }
    
    #[test]
    fn test_history() {
        let mut calc = Calculator::with_max_history(3);
        calc.add(1, 2).unwrap();
        calc.subtract(5, 3).unwrap();
        calc.multiply(2, 3).unwrap();
        calc.divide(10, 2).unwrap();
        
        assert_eq!(calc.history().len(), 3);
    }
    
    #[quickcheck]
    fn prop_add_commutative(a: i64, b: i64) -> bool {
        let mut calc1 = Calculator::new();
        let mut calc2 = Calculator::new();
        
        let r1 = calc1.add(a, b);
        let r2 = calc2.add(b, a);
        
        r1 == r2
    }
    
    #[quickcheck]
    fn prop_multiply_commutative(a: i32, b: i32) -> bool {
        let mut calc1 = Calculator::new();
        let mut calc2 = Calculator::new();
        
        let a = a as i64;
        let b = b as i64;
        
        let r1 = calc1.multiply(a, b);
        let r2 = calc2.multiply(b, a);
        
        r1 == r2
    }
    
    #[quickcheck]
    fn prop_add_associative(a: i32, b: i32, c: i32) -> bool {
        let mut calc = Calculator::new();
        
        let a = a as i64 / 100;
        let b = b as i64 / 100;
        let c = c as i64 / 100;
        
        let r1 = calc.add(a, b).and_then(|ab| calc.add(ab, c));
        calc.reset();
        let r2 = calc.add(b, c).and_then(|bc| calc.add(a, bc));
        
        r1 == r2
    }
}