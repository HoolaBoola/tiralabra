use super::shunting_yard;
use Token::*;

pub fn calculate_infix(input: &str) -> String {
    let res = shunting_yard(input).unwrap();
    format!("{}", eval_postfix(res).unwrap())
}

fn eval_postfix(input: Vec<Token>) -> Result<f64, &'static str> {
    let mut stack = Vec::new();
    for token in input {
        match token {
            Number(num) => stack.push(num),
            Operator(op) => {
                let a = stack.pop().ok_or("Too many operators")?;
                let b = stack.pop().ok_or("Too many operators")?;
                stack.push(operate(b, a, op))
            }
        }
    }

    let res = stack.pop().ok_or("Too many operators")?;
    Ok(res)
}

fn operate(a: f64, b: f64, c: char) -> f64 {
    match c {
        '+' => a + b,
        '-' => a - b,
        '*' => a * b,
        '/' => if b != 0.0 { a / b } else { f64::NAN },
        _ => 0.0,
    }
}

/// Token can represent either a `Number` or an `Operator`
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
}

#[cfg(test)]
mod eval_postfix_tests {
    use super::*;

    #[test]
    fn single_digit_works() {
        let test_vec = vec![Number(1.0)];
        let res = eval_postfix(test_vec).unwrap();

        assert_eq!(res, 1.0);
    }

    #[test]
    fn one_one_plus_works() {
        let test_vec = vec![Number(1.0), Number(1.0), Operator('+')];
        let res = eval_postfix(test_vec).unwrap();
        
        assert_eq!(res, 2.0);
    }

    #[test]
    fn three_two_mul_four_plus_works() {
        let test_vec = vec![Number(3.0), Number(2.0), Operator('*'), Number(4.0), Operator('+')];
        let res = eval_postfix(test_vec).unwrap();

        assert_eq!(res, 10.0);
    }

    #[test]
    fn operator_before_numbers_gives_error() {
        let test_vec = vec![Operator('+'), Number(1.0), Number(2.0)];
        let res = eval_postfix(test_vec);

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
