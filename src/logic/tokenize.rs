use super::calculator::Token::{self, Float, Number, Operator, Variable};

/// Tokenize a string into a `Vec` of Tokens
///
/// Example:
/// ```
/// let result = tokenize("1 + 1").unwrap();
/// let correct = vec![Number(1), Operator('+'), Number(1)];
///
/// assert_eq!(result, correct);
/// ```
pub fn tokenize(input: &str) -> Result<Vec<Token>, String> {
    let mut output = Vec::new();

    let mut i = 0;
    while i < input.len() {
        let &c = &input[i..i + 1].chars().next().unwrap();
        
        // handle case of `c` being one of '+', '/', etc.
        if is_operator(c) {
            output.push(Operator(c));
            i += 1;
            continue;
        }

        // if `c` is a digit (0 <= c <= 9) then find out how long the number is 
        if c.is_digit(10) {
            let mut end = i + 1;
            let mut found_decimal = false;

            // if the current number is more than one digit (e.g. 13),
            // need to loop to find the end
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

            // if the number contained a decimal separator ('.'), then push a Token::Float 
            // else, push a Token::Number
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

        // ignore whitespace. This way "1+1" and "1 + 1" are equivalent
        if c.is_whitespace() {
            i += 1;
            continue;
        }

        // if c is a letter, it can either be a variable name or a function
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

/// Return true if `c` is one of the defined mathematical operators
fn is_operator(c: char) -> bool {
    matches!(c, '+' | '-' | '*' | '/' | '(' | ')' | '^')
}
