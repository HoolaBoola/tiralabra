use super::shunting_yard;
use Token::*;

pub fn calculate_infix(input: &str) -> String {
    let res = shunting_yard(input).unwrap();
    format!("{}", eval_postfix(res).unwrap())
}

fn eval_postfix(input: Vec<Token>) -> Result<u64, &'static str> {
    let mut stack = Vec::new();
    for token in input {
        match token {
            Number(num) => stack.push(num),
            Operator(op) => {
                let a = stack.pop().ok_or("Too many operators")?;
                let b = stack.pop().ok_or("Too many operators")?;
                stack.push(perform_operation(a, b, op))
            }
        }
    }

    let res = stack.pop().ok_or("Too many operators")?;
    Ok(res)
}

fn perform_operation(a: u64, b: u64, c: char) -> u64 {
    match c {
        '+' => b + a,
        '-' => b - a,
        '*' => b * a,
        '/' => b / a,
        _ => 0,
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
#[derive(Debug, Eq, PartialEq)]
pub enum Token {
    Number(u64),
    Operator(char),
}
