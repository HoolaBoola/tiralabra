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

// #[derive(Debug, PartialEq, Clone, Copy)]
// pub enum Number {
//     Integer(i64),
//     Float(f64)
// }
// 
// use Number::{Integer, Float};
// 
// impl std::ops::Add for Number {
//     type Output = Self;
// 
//     fn add(self, other: Self) -> Self {
//         match (self, other) {
//             (Integer(a), Integer(b)) => Integer(a + b),
//             (Float(a), Integer(b)) => Float(a + b as f64),
//             (Integer(a), Float(b)) => Float(a as f64 + b),
//             (Float(a), Float(b)) => Float(a + b)
//         }
//     }
// }
// 
// impl std::ops::Sub for Number {
//     type Output = Self;
// 
//     fn sub(self, other: Self) -> Self {
//         match (self, other) {
//             (Integer(a), Integer(b)) => Integer(a - b),
//             (Float(a), Integer(b)) => Float(a - b as f64),
//             (Integer(a), Float(b)) => Float(a as f64 - b),
//             (Float(a), Float(b)) => Float(a - b)
//         }
//     }
// }
// 
// impl std::ops::Mul for Number {
//     type Output = Self;
// 
//     fn mul(self, other: Self) -> Self {
//         match (self, other) {
//             (Integer(a), Integer(b)) => Integer(a * b),
//             (Float(a), Integer(b)) => Float(a * b as f64),
//             (Integer(a), Float(b)) => Float(a as f64 * b),
//             (Float(a), Float(b)) => Float(a * b)
//         }
//     }
// }
// 
// impl std::ops::Div for Number {
//     type Output = Self;
// 
//     fn div(self, other: Self) -> Self {
//         match (self, other) {
//             (Integer(a), Integer(b)) => Float(a as f64 / b as f64),
//             (Float(a), Integer(b)) => Float(a / b as f64),
//             (Integer(a), Float(b)) => Float(a as f64 / b),
//             (Float(a), Float(b)) => Float(a / b)
//         }
//     }
// }
// 
// impl Number {
//     pub fn unwrap(self) -> f64 {
//         match self {
//             Float(a) => a,
//             Integer(a) => a as f64
//         }
//     }
// 
//     pub fn pow(self, other: Self) -> Self {
//         match (self, other) {
//             (Integer(a), Integer(b)) => if b > 0 { Integer(a.pow(b as u32)) } else {Float((a as f64).powi(b as i32))},
//             _ => Float(self.unwrap().powf(other.unwrap()))
//         }
//     }
// }
// 
// impl std::fmt::Display for Number {
//     fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
//         write!(f, "{}", self.unwrap())
//     }
// }

