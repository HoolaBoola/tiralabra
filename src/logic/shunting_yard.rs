use super::enums::Token::{self, Value, Variable, Op};
use super::enums::Operator::{self, Lparen, Rparen, Plus, Minus, Mul, Div, Pow};
use super::enums::Number::{Integer, Float};
/// Returns the precedence value for given operator, as described in
/// [here](https://en.wikipedia.org/wiki/Shunting-yard_algorithm#Detailed_example):
///
/// `+`, `-` -> 2,
/// `*`, `/` -> 3,
/// `^` -> 4
///
/// ```
/// precedence('+'); // Some(2)
/// precedence('h'); // None
/// ```
fn precedence(op: Operator) -> Option<u8> {
    match op {
        Plus | Minus => Some(2),
        Mul | Div => Some(3),
        Pow => Some(4),
        _ => None,
    }
}

/// Performs Dijkstra's Shunting yard algorithm to convert mathematical
/// expressions from infix notation to postfix (Reverse Polish Notation)
///
/// input: Vec<Token> formed from an expression, with e.g. the `tokenize`-function in
/// `logic::tokenize`
///
/// Returns `Result` with either a `Vec` of `Token`'s, or an error
///
/// ```
/// let input = "1 + 2 * 4";
/// let tokens = tokenize(input).unwrap();
/// let res = shunting_yard(tokens).unwrap();
/// // -> [Number(1.0), Number(2.0), Number(4.0), Operator('*'), Operator('+')]
/// ```
pub fn shunting_yard(input: Vec<Token>) -> Result<Vec<Token>, String> {
    let mut output = Vec::new();
    let mut operators = Vec::new();

    // only one operator can occur before a number
    // "1 + 1" is ok, "1 ++ 1" is not
    let mut is_operator_time = false;

    for token in input {
        match token {
            Op(Lparen) => operators.push(Lparen),
            Op(Rparen) => {
                let mut found = false;
                while let Some(op) = operators.pop() {
                    if op == Lparen {
                        found = true;
                        break;
                    }

                    output.push(Op(op));
                }

                if !found {
                    return Err("Right parenthesis without a pair found".to_string());
                }
            }
            Op(op) => {
                if !is_operator_time {
                    return Err(format!("Unexpected operator: {op}"));
                }
                is_operator_time = false;
                if let Some(p1) = precedence(op) {
                    while !operators.is_empty() {
                        let last_operator = operators[operators.len() - 1];
                        if last_operator == Lparen {
                            break;
                        }

                        if let Some(p2) = precedence(last_operator) {
                            if p2 <= p1 {
                                break;
                            }
                        }
                        output.push(Op(operators.pop().unwrap()));
                    }

                    operators.push(op);
                }
            }
            Value(Integer(_) | Float(_)) | Variable(_) => {
                if is_operator_time {
                    return Err("Too many numbers in a row".to_string());
                }
                is_operator_time = true;
                output.push(token);
            }
            _ => ()
        }
    }

    while let Some(op) = operators.pop() {
        if op == Lparen {
            return Err("Left parenthesis without a pair found".to_string());
        }
        output.push(Op(op));
    }

    Ok(output)
}

#[cfg(test)]
mod shunting_yard_tests {

    use super::*;

    #[test]
    fn single_digit_works() {
        let tokens = vec![Number(1.0)];
        let res = shunting_yard(tokens).unwrap();
        assert_eq!(res[0], Number(1.0));
    }

    #[test]
    fn one_plus_one_works() {
        let tokens = vec![Number(1.0), Operator('+'), Number(1.0)];
        let res = shunting_yard(tokens).unwrap();
        let correct = vec![Number(1.0), Number(1.0), Operator('+')];
        assert_eq!(res, correct);
    }

    #[test]
    fn one_plus_two_times_four_works() {
        let tokens = vec![
            Number(1.0),
            Operator('+'),
            Number(2.0),
            Operator('*'),
            Number(4.0),
        ];
        let res = shunting_yard(tokens).unwrap();
        let correct = vec![
            Number(1.0),
            Number(2.0),
            Number(4.0),
            Operator('*'),
            Operator('+'),
        ];
        assert_eq!(res, correct);
    }

    #[test]
    fn exponents_work() {
        let tokens = vec![
            Number(2.0),
            Operator('^'),
            Number(4.0),
            Operator('*'),
            Number(2.0),
        ];

        let res = shunting_yard(tokens).unwrap();
        let correct = vec![
            Number(2.0),
            Number(4.0),
            Operator('^'),
            Number(2.0),
            Operator('*'),
        ];

        assert_eq!(res, correct);
    }

    #[test]
    fn mismatched_right_parenthesis_errors() {
        let tokens = vec![
            Number(1.0),
            Operator('+'),
            Number(2.0),
            Operator('*'),
            Number(3.0),
            Operator(')'),
        ];
        let res = shunting_yard(tokens);
        assert!(res.is_err());
    }

    #[test]
    fn mismatched_left_parenthesis_errors() {
        let tokens = vec![
            Number(1.0),
            Operator('+'),
            Operator('('),
            Number(2.0),
            Operator('*'),
            Number(3.0),
        ];
        let res = shunting_yard(tokens);
        assert!(res.is_err());
    }

    #[test]
    fn too_many_numbers_in_a_row() {
        let tokens = vec![Number(1.0), Number(2.0), Operator('*'), Number(100.0)];

        let res = shunting_yard(tokens);

        assert!(res.is_err());
    }
}
