pub fn calculate_infix(input: &str) -> String {
    shunting_yard(input)
        .into_iter()
        .map(|x| format!("{x:?}"))
        .collect::<Vec<_>>()
        .join(" ")
}

fn precedence(c: char) -> Option<u8> {
    match c {
        '+' | '-' => Some(2),
        '*' | '/' => Some(3),
        '^' => Some(4),
        _ => None,
    }
}

use Token::*;

fn shunting_yard(input: &str) -> Vec<Token> {
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
                panic!("Matching left parenthesis not found");
            }
        }
    }
    while let Some(op) = operators.pop() {
        output.push(Operator(op));
    }
    output
}

#[derive(Debug, Eq, PartialEq)]
enum Token {
    Number(u64),
    Operator(char),
}

#[cfg(test)]
mod tests {
    
    use super::*;

    #[test]
    fn single_digit_works() {
        let test_str = "1";
        let res = shunting_yard(test_str);
        assert_eq!(res[0], Number(1));
    }
}
