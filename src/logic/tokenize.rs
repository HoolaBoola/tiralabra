use super::calculator::Token::{self, Float, Number, Operator, Variable};

/// Tokenize a string into a `Vec` of Tokens.
///
/// If parsing fails, returns an error variant with reason for failing.
///
/// This function does not care about order of operations (`1 + 1` is just as valid as `1 * + /`)
/// Example:
/// ```
/// let result = tokenize("1 + 1").unwrap();
/// let correct = vec![Number(1), Operator('+'), Number(1)];
///
/// assert_eq!(result, correct);
/// ```
pub fn tokenize(input: &str) -> Result<Vec<Token>, String> {
    let mut output = Vec::new();

    let mut chars = input.chars().peekable();

    loop {
        let c = chars.next();
        if c.is_none() {
            break;
        }
        let c = c.unwrap();

        // if c is the minus sign, two operators in a row is ok (negative number) if the following
        // character is a digit
        let negative_number = if c == '-' {
            // if the previous character is an operator, or `c` is the first character of the
            // input
            matches!(
                (output.last(), chars.peek()),
                (Some(Operator(_)) | None, Some('0'..='9'))
            )
        } else {
            false
        };

        // if `c` is a digit (0 <= c <= 9) then find out how long the number is
        if c.is_digit(10) || negative_number {
            let mut num_string = String::new();
            num_string.push(c);
            let mut found_decimal = false;

            // if the current number is more than one digit (e.g. 13),
            // need to loop to find the end
            while let Some(&c) = chars.peek() {
                if c.is_digit(10) {
                    num_string.push(c);
                } else if c == '.' {
                    if found_decimal {
                        // TODO: error handling (too many decimal separators)
                    }
                    num_string.push(c);
                    found_decimal = true;
                } else {
                    break;
                }
                chars.next();
            }

            // if the number contained a decimal separator ('.'), then push a Token::Float
            // else, push a Token::Number
            if found_decimal {
                let &num = &num_string.parse::<f64>().unwrap();
                output.push(Float(num));
            } else {
                let &num = &num_string.parse::<f64>().unwrap();
                output.push(Number(num));
            }
            continue;
        }

        // handle case of `c` being one of '+', '/', etc.
        if is_operator(c) {
            output.push(Operator(c));
            continue;
        }

        // ignore whitespace. This way "1+1" and "1 + 1" are equivalent
        if c.is_whitespace() {
            continue;
        }

        // if c is a letter, it can either be a variable name or a function
        if c.is_alphabetic() {
            let mut var_string = String::new();
            var_string.push(c);

            while let Some(&c) = chars.peek() {
                if c.is_alphabetic() {
                    var_string.push(c);
                } else {
                    break;
                }
                chars.next();
            }
            output.push(Variable(var_string));
            continue;
        }

        if c == '=' {
            output.push(Operator('='));
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

#[cfg(test)]
mod tokenize_tests {
    use super::*;

    #[test]
    fn simple_case() {
        let test_str = "1";
        let result = tokenize(test_str).unwrap();

        assert_eq!(result, vec![Number(1.0)]);
    }

    #[test]
    fn one_plus_times_div() {
        let test_str = "1 + * /";
        let result = tokenize(test_str).unwrap();

        let correct = vec![Number(1.0), Operator('+'), Operator('*'), Operator('/')];

        assert_eq!(result, correct);
    }

    #[test]
    fn decimal_numbers_parsed() {
        let test_str = "1.5";
        let result = tokenize(test_str).unwrap();

        assert_eq!(result, vec![Float(1.5)]);
    }

    #[test]
    fn variables() {
        let test_str = "a + 1";
        let result = tokenize(test_str).unwrap();

        let correct = vec![Variable("a".to_string()), Operator('+'), Number(1.0)];

        assert_eq!(result, correct);
    }

    #[test]
    fn multi_letter_variables() {
        let test_str = "abcdefg";
        let result = tokenize(test_str).unwrap();
        
        assert_eq!(result[0], Variable(test_str.to_string()));
    }

    #[test]
    fn unknown_character() {
        let test_str = "¦ + 1";
        let result = tokenize(test_str);

        assert!(result.is_err());
    }
}

#[cfg(test)]
mod is_operator_tests {
    use super::*;

    #[test]
    fn returns_true_for_operators() {
        let operators = ['+', '-', '*', '/', '^', '(', ')'];

        for operator in operators {
            assert!(is_operator(operator));
        }
    }

    #[test]
    fn returns_false_for_others() {
        let not_operators = ['a', '1', '€', '?', '.'];

        for not_operator in not_operators {
            assert!(!is_operator(not_operator));
        }
    }
}
