use super::shunting_yard;
use super::tokenize;
use Token::*;
use std::collections::HashMap;


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
    history: Vec<String>,
    variables: HashMap<String, f64>
}

impl Calculator {
    pub fn new() -> Calculator {
        Calculator {
            history: Vec::new(),
            variables: HashMap::new()
        }
    }

    /// Enter a string with an infix expression (example: "2 * (2 + 1)") as parameter.
    /// Returns a result containing the evaluated result of the expression, or an error
    pub fn calculate_infix(&mut self, input: &str) -> Result<String, String> {
        self.history.push(input.to_string());
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

    fn eval_postfix(&self, input: Vec<Token>) -> Result<f64, String> {
        let mut stack = Vec::new();
        for token in input {
            match token {
                Number(num) => stack.push(num),
                Float(num) => stack.push(num),
                Operator(op) => {
                    let a = stack.pop().ok_or("Too many operators")?;
                    let b = stack.pop().ok_or("Too many operators")?;
                    stack.push(operate(b, a, op))
                },
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
        Ok(res)
    }

}

fn operate(a: f64, b: f64, c: char) -> f64 {
    match c {
        '+' => a + b,
        '-' => a - b,
        '*' => a * b,
        '/' => {
            if b != 0.0 {
                a / b
            } else {
                f64::NAN
            }
        },
        '^' => a.powf(b),
        _ => 0.0,
    }
}

/// Token can represent either a `Number`, a `Float`, a `Variable` or an `Operator`
///
/// Now, one can create a `Vec<Token>` with numbers and operators mixed without
/// losing type safety.
///
/// ```
/// let a = Number(1);
/// let b = Operator('+');
/// let output = vec![a, b];
/// ```
///
#[derive(Debug, PartialEq)]
pub enum Token {
    Number(f64),
    Operator(char),
    Float(f64),
    Variable(String),
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
        let  calculator = Calculator::new();
        let test_vec = vec![Number(1.0), Number(1.0), Operator('+')];
        let res = calculator.eval_postfix(test_vec).unwrap();

        assert_eq!(res, 2.0);
    }

    #[test]
    fn three_two_mul_four_plus_works() {
        let  calculator = Calculator::new();
        let test_vec = vec![
            Number(3.0),
            Number(2.0),
            Operator('*'),
            Number(4.0),
            Operator('+'),
        ];
        let res = calculator.eval_postfix(test_vec).unwrap();

        assert_eq!(res, 10.0);
    }

    #[test]
    fn operator_before_numbers_gives_error() {
        let  calculator = Calculator::new();
        let test_vec = vec![Operator('+'), Number(1.0), Number(2.0)];
        let res = calculator.eval_postfix(test_vec);

        assert!(res.is_err());
    }
}

#[cfg(test)]
mod operate_tests {
    use super::operate;

    #[test]
    fn one_plus_one_is_two() {
        let res = operate(1.0, 1.0, '+');

        assert_eq!(res, 2.0);
    }

    #[test]
    fn two_times_four_is_eight() {
        let res = operate(2.0, 4.0, '*');

        assert_eq!(res, 8.0);
    }

    #[test]
    fn four_div_eight_is_half() {
        let res = operate(4.0, 8.0, '/');

        assert_eq!(res, 0.5);
    }

    #[test]
    fn one_minus_two_is_minus_one() {
        let res = operate(1.0, 2.0, '-');

        assert_eq!(res, -1.0);
    }

    #[test]
    fn unknown_operator_returns_zero() {
        let res = operate(1.0, 10.0, '?');

        assert_eq!(res, 0.0);
    }

    #[test]
    fn divide_by_zero_returns_nan() {
        let res = operate(1.0, 0.0, '/');

        assert!(res.is_nan());
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
