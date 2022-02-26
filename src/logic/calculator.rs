use super::shunting_yard;
use super::tokenize;
use std::collections::HashMap;
use super::enums::Token::{self, Op, Variable, Number};
// use super::enums::Number;
use super::enums::Operator::{self, *};

/// Struct for keeping track of history and variables, and performing calculations.
///
/// Example:
///
/// ```
/// let mut calculator = Calculator::new();
/// let result = calculator.calculate_infix("(1 + 2) * 3").unwrap();
///
/// assert_eq!(result, "9");
/// ```
///
/// With variables:
/// ```
/// let mut calculator = Calculator::new();
///
/// // returns a's value, in this case 1
/// let result = calculator.calculate_infix("a = 1").unwrap();
/// assert_eq!(result, "1");
///
/// let result = calulator.calculator_infix("a + 1").unwrap();
/// assert_eq!(result, "2");
/// ```
///
pub struct Calculator {
    variables: HashMap<String, f64>,
}

impl Calculator {
    pub fn new() -> Calculator {
        Calculator {
            variables: HashMap::new(),
        }
    }

    /// Enter a string with an infix expression (example: "2 * (2 + 1)") as parameter.
    /// Returns a result containing the evaluated result of the expression, or an error
    pub fn calculate_infix(&mut self, input: &str) -> Result<String, String> {

        let mut eq_position = None;

        // find the position of the '=' character, if it exists
        for (i, c) in input.chars().enumerate() {
            if c == '=' {
                eq_position = Some(i);
                break;
            }
        }

        // if the input string contains a '=', split it into two parts
        let (variable, input) = if let Some(i) = eq_position {
            (Some(tokenize(&input[..i])?), &input[i + 1..])
        } else {
            (None, input)
        };
        let tokens = tokenize(input)?;
        let postfix = shunting_yard(tokens)?;
        let result = self.eval_postfix(postfix)?;

        // if the expression is supposed to assign to a variable,
        // insert the key-value pair into `variables`
        if let Some(var_list) = variable {
            // if expression has more than one token before =
            // (e.g. "a b = 1 + 1")
            if var_list.len() > 1 {
                return Err("Too many tokens before '='".to_string());
            }
            // if expression starts with =
            // (e.g. "= 1 + 1")
            if var_list.is_empty() {
                return Err("Variable required before '='".to_string());
            }

            // Get the first (only) item from the list and insert it into `self.variables`
            // with the corresponding value
            if let Variable(variable) = &var_list[0] {
                self.variables.insert(variable.to_string(), result);
            } else {
                return Err("Malformed input before '='".to_string());
            }
        }
        Ok(format!("{result}"))
    }

    /// Calculates a postfix expression and returns a single numerical value. (Or an error if the
    /// expression is malformed)
    fn eval_postfix(&self, input: Vec<Token>) -> Result<f64, String> {
        let mut stack = Vec::new();
        for token in input {
            match token {
                Number(num) => stack.push(num),
                Op(Func(fun)) => {
                    let arg = stack.pop().ok_or("Too few numbers")?;
                    stack.push(fun.evaluate(arg));
                }
                Op(op) => {
                    let a = stack.pop().ok_or("Too many operators")?;
                    let b = stack.pop().ok_or("Too many operators")?;
                    match operate(b, a, op) {
                        Ok(result) => stack.push(result),
                        err_msg @ Err(_) => return err_msg
                    };


                }
                Variable(var) => {
                    if let Some(&val) = self.variables.get(&var) {
                        stack.push(val);
                    } else {
                        return Err(format!("Undefined variable: {var}"));
                    }
                }
            }
        }

        let res = stack.pop().ok_or("Too many operators")?;

        if !stack.is_empty() {
            return Err("Too many numbers!".to_string());
        }
        Ok(res)
    }
}

/// Operate on the argument values depending on the `c` character.
///
/// The first argument, `a`, is the one the operation is applied to,
/// for example 
/// ```
/// operate(8.0, 2.0, '/')
/// ```
/// divides 8.0 by 2.0, not the other way.
///
/// If either `a` or `b` is NaN or inf, return an error (because it's not obvious how the
/// application should behave in those cases)
///
/// If dividing by zero or trying to use an unrecognized operator, an error is also returned.
///
fn operate(a: f64, b: f64, op: Operator) -> Result<f64, String> {
    use crate::logic::enums::Operator::*;
    // neither a or b should ever be NaN or infinite (should be caught beforehand), 
    // but in case it happens anyway, return an error
    if a.is_nan() || b.is_nan() {
        return Err("At least one argument is not a number (NaN)".to_string())
    }
    if a.is_infinite() || b.is_infinite() {
        return Err("At least one argument is infinite".to_string());
    }

    match op {
        Plus => Ok(a + b),
        Minus => Ok(a - b),
        Mul => Ok(a * b),
        Div => {
            if b == 0.0 {
                Err("Trying to divide by zero!".to_string())
            } else {
                Ok(a / b)
            }
        }
        Pow => Ok(a.powf(b)),
        // should not be reached ever, but in case of error elsewhere,
        // this branch will catch it
        _ => Err(format!("Unrecognized operator: {op:?}")),
    }
}


#[cfg(test)]
mod eval_postfix_tests {
    use super::*;

    #[test]
    fn single_digit_works() {
        let calculator = Calculator::new();
        let test_vec = vec![Number(1.0)];
        let res = calculator.eval_postfix(test_vec).unwrap();

        assert_eq!(res, 1.0);
    }

    #[test]
    fn one_one_plus_works() {
        let calculator = Calculator::new();
        let test_vec = vec![Number(1.0), Number(1.0), Op(Plus)];
        let res = calculator.eval_postfix(test_vec).unwrap();

        assert_eq!(res, 2.0);
    }

    #[test]
    fn three_two_mul_four_plus_works() {
        let calculator = Calculator::new();
        let test_vec = vec![
            Number(3.0),
            Number(2.0),
            Op(Mul),
            Number(4.0),
            Op(Plus)
        ];
        let res = calculator.eval_postfix(test_vec).unwrap();

        assert_eq!(res, 10.0);
    }

    #[test]
    fn operator_before_numbers_returns_error() {
        let calculator = Calculator::new();
        let test_vec = vec![Op(Plus), Number(1.0), Number(2.0)];
        let res = calculator.eval_postfix(test_vec);

        assert!(res.is_err());
    }

    #[test]
    fn too_many_operators_returns_error() {
        let calculator = Calculator::new();
        let test_vec = vec![Number(1.0), Number(1.0), Op(Mul), Op(Plus)];

        let res = calculator.eval_postfix(test_vec);

        assert!(res.is_err());
    }

    #[test]
    fn too_many_numbers_returns_error() {
        let calculator = Calculator::new();
        let test_vec = vec![Number(1.0), Number(1.0), Op(Div), Number(1.0)];
        let res = calculator.eval_postfix(test_vec);

        assert!(res.is_err());
    }
}

#[cfg(test)]
mod operate_tests {
    use super::operate;
    use super::Operator::*;

    #[test]
    fn one_plus_one_is_two() {
        let res = operate(1.0, 1.0, Plus);

        assert_eq!(res.unwrap(), 2.0);
    }

    #[test]
    fn two_times_four_is_eight() {
        let res = operate(2.0, 4.0, Mul);

        assert_eq!(res.unwrap(), 8.0);
    }

    #[test]
    fn four_div_eight_is_half() {
        let res = operate(4.0, 8.0, Div);

        assert_eq!(res.unwrap(), 0.5);
    }

    #[test]
    fn one_minus_two_is_minus_one() {
        let res = operate(1.0, 2.0, Minus);

        assert_eq!(res.unwrap(), -1.0);
    }

    #[test]
    fn unknown_operator_returns_err() {
        let res = operate(1.0, 10.0, Rparen);

        assert!(res.is_err());
    }

    #[test]
    fn divide_by_zero_returns_nan() {
        let res = operate(1.0, 0.0, Div);

        assert!(res.is_err());
    }
}

#[cfg(test)]
mod calculate_infix_tests {
    use super::Calculator;

    #[test]
    fn input_only_operator_doesnt_panic() {
        let mut calculator = Calculator::new();
        let res = calculator.calculate_infix("+");
        assert!(res.is_err());
    }

    #[test]
    fn parentheses_work() {
        let mut calculator = Calculator::new();
        let res = calculator.calculate_infix("(1 + 2) * 3");
        assert_eq!(res.unwrap(), "9");
    }
}
