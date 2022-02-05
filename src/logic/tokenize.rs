use super::calculator::Token::{self, Float, Number, Operator, Variable};

pub fn tokenize(input: &str) -> Result<Vec<Token>, String> {
    let mut output = Vec::new();

    let mut i = 0;
    while i < input.len() {
        let &c = &input[i..i + 1].chars().next().unwrap();
        if is_operator(c) {
            output.push(Operator(c));
            i += 1;
            continue;
        }

        if c.is_digit(10) {
            let mut end = i + 1;
            let mut found_decimal = false;

            while end < input.len() {
                let &c = &input[end..end + 1].chars().next().unwrap();

                if c.is_digit(10) {
                    end += 1;
                } else if c == '.' {
                    if found_decimal {
                        // TODO: error handling (too many decimal separators)
                    }
                    end += 1;

                    found_decimal = true;
                } else {
                    break;
                }
            }

            if found_decimal {
                let &num = &input[i..end].parse::<f64>().unwrap();
                output.push(Float(num));
            } else {
                let &num = &input[i..end].parse::<f64>().unwrap();
                output.push(Number(num));
            }
            i = end;
            continue;
        }

        if c.is_whitespace() {
            i += 1;
            continue;
        }

        if c.is_alphabetic() {
            let mut end = i + 1;

            while end < input.len() {
                let &c = &input[end..end + 1].chars().next().unwrap();

                if c.is_alphabetic() {
                    end += 1;
                } else {
                    break;
                }
            }

            let variable = &input[i..end];
            output.push(Variable(variable.to_string()));
            i = end;
            continue;
        }

        if c == '=' {
            output.push(Operator('='));
            i += 1;
            continue;
        }
        return Err(format!("unknown character: {c}"));
    }
    Ok(output)
}

fn is_operator(c: char) -> bool {
    match c {
        '+' | '-' | '*' | '/' | '(' | ')' => true,
        _ => false,
    }
}