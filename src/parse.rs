#[derive(Default, Debug, PartialEq)]
pub struct Parsed {
    pub parsed: Vec<NumOp>,
}
#[derive(Debug, PartialEq)]
pub enum NumOp {
    Num(f64),
    Operator(Operators),
}
impl From<f64> for NumOp {
    fn from(value: f64) -> Self {
        Self::Num(value)
    }
}
impl From<Operators> for NumOp {
    fn from(value: Operators) -> Self {
        Self::Operator(value)
    }
}
impl From<Function> for Operators {
    fn from(value: Function) -> Self {
        Self::Fun(value)
    }
}
impl From<Function> for NumOp {
    fn from(value: Function) -> Self {
        Self::Operator(value.into())
    }
}
impl NumOp {
    pub fn num(self) -> f64 {
        let NumOp::Num(num) = self else {
            unreachable!()
        };
        num
    }
    pub fn num_mut(&mut self) -> &mut f64 {
        let NumOp::Num(num) = self else {
            unreachable!()
        };
        num
    }
}
#[derive(Debug, PartialEq)]
pub enum ParseError {}
impl Parsed {
    pub fn rpn(value: &str) -> Result<Self, ParseError> {
        let mut parsed = Vec::new();
        let mut chars = value.chars().peekable();
        while let Some(c) = chars.next() {
            if c.is_ascii_whitespace() {
                continue;
            }
            if c.is_ascii_alphabetic() {
                let mut n = c.to_string();
                while let Some(t) = chars.peek() {
                    if t.is_ascii_alphabetic() {
                        n.push(chars.next().unwrap());
                    } else {
                        break;
                    }
                }
                if let Some(value) = Constants::get(n.as_str()) {
                    parsed.push(NumOp::Num(value));
                } else {
                    parsed.push(NumOp::Operator(Operators::Fun(
                        Function::try_from(n.as_str()).unwrap(),
                    )));
                }
            } else if c.is_ascii_digit() {
                let mut n = c.to_string();
                while let Some(t) = chars.peek() {
                    if t.is_ascii_digit() || *t == '.' {
                        n.push(chars.next().unwrap());
                    } else {
                        break;
                    }
                }
                parsed.push(NumOp::Num(n.parse().unwrap()));
            } else if let Ok(operator) = Operators::try_from(c) {
                parsed.push(NumOp::Operator(operator));
            }
        }
        Ok(Self { parsed })
    }
    pub fn infix(value: &str) -> Result<Self, ParseError> {
        let mut parsed = Vec::new();
        let mut operator_stack: Vec<Operators> = Vec::new();
        let mut chars = value.chars().peekable();
        let mut negate = true;
        while let Some(c) = chars.next() {
            if c.is_ascii_whitespace() {
                continue;
            }
            if c.is_ascii_alphabetic() {
                let mut n = c.to_string();
                while let Some(t) = chars.peek() {
                    if t.is_ascii_alphabetic() {
                        n.push(chars.next().unwrap());
                    } else {
                        break;
                    }
                }
                if let Some(value) = Constants::get(n.as_str()) {
                    parsed.push(NumOp::Num(value));
                } else {
                    operator_stack.push(Operators::Fun(Function::try_from(n.as_str()).unwrap()));
                }
                negate = false;
            } else if c.is_ascii_digit() {
                let mut n = c.to_string();
                while let Some(t) = chars.peek() {
                    if t.is_ascii_digit() || *t == '.' {
                        n.push(chars.next().unwrap());
                    } else {
                        break;
                    }
                }
                parsed.push(NumOp::Num(n.parse().unwrap()));
                negate = false;
            } else if let Ok(mut operator) = Operators::try_from(c) {
                if negate && Operators::Sub == operator {
                    operator = Operators::Negate;
                }
                if operator != Operators::LeftParenthesis {
                    while let Some(top) = operator_stack.last()
                        && *top != Operators::LeftParenthesis
                        && (top.precedence() > operator.precedence()
                            || (top.precedence() == operator.precedence()
                                && operator.associativity() == Some(Associativity::Left)))
                    {
                        parsed.push(NumOp::Operator(operator_stack.pop().unwrap()));
                    }
                }
                operator_stack.push(operator);
                negate = true;
            } else {
                match c {
                    ',' => {
                        while let Some(top) = operator_stack.last()
                            && *top != Operators::LeftParenthesis
                        {
                            parsed.push(NumOp::Operator(operator_stack.pop().unwrap()));
                        }
                        negate = true;
                    }
                    ')' => {
                        while let Some(top) = operator_stack.last()
                            && *top != Operators::LeftParenthesis
                        {
                            parsed.push(NumOp::Operator(operator_stack.pop().unwrap()));
                        }
                        operator_stack.pop();
                        if let Some(top) = operator_stack.last()
                            && matches!(top, Operators::Fun(_))
                        {
                            parsed.push(NumOp::Operator(operator_stack.pop().unwrap()));
                        }
                        negate = false;
                    }
                    _ => unreachable!(),
                }
            }
        }
        while let Some(operator) = operator_stack.pop() {
            parsed.push(NumOp::Operator(operator));
        }
        Ok(Self { parsed })
    }
}
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Operators {
    Add,
    Sub,
    Mul,
    Div,
    Pow,
    LeftParenthesis,
    Negate,
    Fun(Function),
}
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Function {
    Sin,
    Cos,
    Atan,
    Max,
    Min,
    Quadratic,
    Ln,
}
impl TryFrom<&str> for Function {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Ok(match value {
            "ln" => Function::Ln,
            "min" => Function::Min,
            "max" => Function::Max,
            "sin" => Function::Sin,
            "cos" => Function::Cos,
            "atan" => Function::Atan,
            "quadratic" => Function::Quadratic,
            _ => return Err(()),
        })
    }
}
impl TryFrom<char> for Operators {
    type Error = ();
    fn try_from(value: char) -> Result<Self, Self::Error> {
        Ok(match value {
            '^' => Operators::Pow,
            '*' => Operators::Mul,
            '/' => Operators::Div,
            '+' => Operators::Add,
            '-' => Operators::Sub,
            '_' => Operators::Negate,
            '(' => Operators::LeftParenthesis,
            _ => return Err(()),
        })
    }
}
#[derive(PartialOrd, PartialEq, Clone, Copy)]
pub enum Precedence {
    Pow = 3,
    Negate = 2,
    Mul = 1,
    Add = 0,
}
#[derive(PartialOrd, PartialEq, Clone, Copy)]
pub enum Associativity {
    Left,
    Right,
}
impl Function {
    pub fn inputs(self) -> usize {
        match self {
            Function::Cos | Function::Sin | Function::Ln => 1,
            Function::Atan => 2,
            Function::Max => 2,
            Function::Min => 2,
            Function::Quadratic => 3,
        }
    }
}
impl Operators {
    pub fn inputs(self) -> Option<usize> {
        Some(match self {
            Operators::Mul | Operators::Div | Operators::Add | Operators::Sub | Operators::Pow => 2,
            Operators::Negate => 1,
            Operators::Fun(fun) => fun.inputs(),
            Operators::LeftParenthesis => return None,
        })
    }
    pub fn precedence(self) -> Option<Precedence> {
        Some(match self {
            Operators::Add | Operators::Sub => Precedence::Add,
            Operators::Mul | Operators::Div => Precedence::Mul,
            Operators::Pow => Precedence::Pow,
            Operators::Negate => Precedence::Negate,
            Operators::LeftParenthesis | Operators::Fun(_) => return None,
        })
    }
    pub fn associativity(self) -> Option<Associativity> {
        Some(match self {
            Operators::Add | Operators::Sub | Operators::Mul | Operators::Div => {
                Associativity::Left
            }
            Operators::Pow => Associativity::Right,
            Operators::LeftParenthesis | Operators::Fun(_) | Operators::Negate => return None,
        })
    }
}
pub enum Constants {
    Pi,
    E,
}
impl TryFrom<&str> for Constants {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Ok(match value {
            "pi" => Constants::Pi,
            "e" => Constants::E,
            _ => return Err(()),
        })
    }
}
impl Constants {
    pub fn get(value: &str) -> Option<f64> {
        if let Ok(c) = Self::try_from(value) {
            Some(match c {
                Constants::Pi => std::f64::consts::PI,
                Constants::E => std::f64::consts::E,
            })
        } else {
            None
        }
    }
}
