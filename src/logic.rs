pub fn calculate_infix(input: &str) -> String {
    shunting_yard(input)
}

fn precedence(c: char) -> Option<u8> {
    match c {
        '+' | '-' => Some(2),
        '*' | '/' => Some(3),
        '^' => Some(4),
        _ => None
    }
}

fn shunting_yard(input: &str) -> String {
    let mut output = Vec::new();
    let mut operators = Vec::new();

    for token in input.chars() {
        if token.is_digit(10) {
            output.push(token);
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
                output.push(operators.pop().unwrap());
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

                output.push(op);
            }

            if !found {
                panic!("Matching left parenthesis not found");
            }
        }
    }
    while let Some(op) = operators.pop() {
        output.push(op);
    }
    output.into_iter().collect()
}
