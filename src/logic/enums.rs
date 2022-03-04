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
    Op(Operator),
    Variable(String)
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Operator {
    Plus,
    Minus,
    Mul,
    Div,
    Pow,
    Lparen,
    Rparen,
    Equals,
    Func(Function)
}

impl std::fmt::Display for Operator {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let c = match self {
            Operator::Plus => "+",
            Operator::Minus => "-",
            Operator::Mul => "*",
            Operator::Div => "/",
            Operator::Pow => "^",
            Operator::Lparen => "(",
            Operator::Rparen => ")",
            Operator::Equals => "=",
            Operator::Func(fun) => fun.format()
        };
        write!(f, "{c}")
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Function {
    Sin,
    Cos,
    Tan,
    Sqrt
}

impl Function {
    pub fn evaluate(self, param: f64) -> f64 {
        match self {
            Function::Sin => param.sin(),
            Function::Cos => param.cos(),
            Function::Tan => param.tan(),
            Function::Sqrt => param.sqrt()
        }
    }

    pub fn format(self) -> &'static str {
        match self {
            Function::Sin => "sin",
            Function::Cos => "cos",
            Function::Tan => "tan",
            Function::Sqrt => "sqrt",
        }
    }
}

#[cfg(test)]
mod function_tests {
    use super::Function::*;
    #[test]
    fn evaluate_first_test() {
        let val: f64 = 1.4;
        let funcs = [Sin, Cos, Tan, Sqrt];
        let correct = [val.sin(), val.cos(), val.tan(), val.sqrt()];

        for (f, res) in funcs.iter().zip(correct) {
            assert_eq!(f.evaluate(val), res);
        }
    }

    #[test]
    fn evaluate_second_test() {
        let val: f64 = 1234.123;
        let funcs = [Sin, Cos, Tan, Sqrt];
        let correct = [val.sin(), val.cos(), val.tan(), val.sqrt()];

        for (f, res) in funcs.iter().zip(correct) {
            assert_eq!(f.evaluate(val), res);
        }
    }

    #[test]
    fn evaluate_third_test() {
        let val = f64::NAN;
        let funcs = [Sin, Cos, Tan, Sqrt];
        let correct = [val.sin(), val.cos(), val.tan(), val.sqrt()];

        for (f, res) in funcs.iter().zip(correct) {
            assert_ne!(f.evaluate(val), res);
        }
    }

    #[test]
    fn format_returns_right_values() {
        let funcs = [Sin, Cos, Tan, Sqrt];
        let correct = ["sin", "cos", "tan", "sqrt"];

        for (f, res) in funcs.iter().zip(correct) {
            assert_eq!(f.format(), res);
        }
    }
}
