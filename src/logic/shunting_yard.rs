use super::calculator::Token::{self, Number, Operator};

/// Returns the precedence value for given operator, as described in
/// [here](https://en.wikipedia.org/wiki/Shunting-yard_algorithm#Detailed_example)
///
/// ```
/// precedence('+'); // Some(2)
/// precedence('h'); // None
/// ```
fn precedence(c: char) -> Option<u8> {
    match c {
        '+' | '-' => Some(2),
        '*' | '/' => Some(3),
        '^' => Some(4),
        _ => None,
    }
}

/// Performs Dijkstra's Shunting yard algorithm to convert mathematical
/// expressions from infix notation to postfix (Reverse Polish Notation)
///
/// input: &str containing a mathematical expression.
/// Returns `Result` with either a `Vec` of `Token`'s, or an error
///
/// ```
/// let input = "1 + 2 * 4";
/// let res = shunting_yard(input).unwrap();
/// // -> [Number(1), Number(2), Number(4), Operator('*'), Operator('+')]
/// ```
pub fn shunting_yard(input: &str) -> Result<Vec<Token>, &str> {
    let mut output = Vec::new();
    let mut operators = Vec::new();

    for token in input.chars() {
        if token.is_digit(10) {
            output.push(Number(token.to_digit(10).unwrap() as u64));
            continue;
        }

        if let Some(p1) = precedence(token) {
            while operators.len() > 0 {
                let last_operator = operators[operators.len() - 1];
                if last_operator == '(' {
                    break;
                }

                if let Some(p2) = precedence(last_operator) {
                    if p2 <= p1 {
                        break;
                    }
                }
                output.push(Operator(operators.pop().unwrap()));
            }

            operators.push(token);
            continue;
        }

        if token == '(' {
            operators.push(token);
            continue;
        }

        if token == ')' {
            let mut found = false;
            while let Some(op) = operators.pop() {
                if op == '(' {
                    found = true;
                    break;
                }

                output.push(Operator(op));
            }

            if !found {
                return Err("Right parenthesis without a pair found");
            }
        }
    }
    while let Some(op) = operators.pop() {
        if op == '(' {
            return Err("Left parenthesis without a pair found");
        }
        output.push(Operator(op));
    }
    Ok(output)
}

#[cfg(test)]
mod shunting_yard_tests {

    use super::*;

    #[test]
    fn single_digit_works() {
        let test_str = "1";
        let res = shunting_yard(test_str).unwrap();
        assert_eq!(res[0], Number(1));
    }

    #[test]
    fn one_plus_one_works() {
        let test_str = "1+1";
        let res = shunting_yard(test_str).unwrap();
        let correct = vec![Number(1), Number(1), Operator('+')];
        assert_eq!(res, correct);
    }

    #[test]
    fn one_plus_two_times_four_works() {
        let test_str = "1 + 2 * 4";
        let res = shunting_yard(test_str).unwrap();
        let correct = vec![
            Number(1),
            Number(2),
            Number(4),
            Operator('*'),
            Operator('+'),
        ];
        assert_eq!(res, correct);
    }

    #[test]
    fn mismatched_right_parenthesis_errors() {
        let test_str = "1 + 2 * 3)";
        let res = shunting_yard(test_str);
        assert!(res.is_err());
    }

    #[test]
    fn mismatched_left_parenthesis_errors() {
        let test_str = "1 + (2 * 3";
        let res = shunting_yard(test_str);
        assert!(res.is_err());
    }
}
