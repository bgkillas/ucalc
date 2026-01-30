#[derive(Default, Debug)]
pub struct Parsed {
    pub parsed: Vec<NumOp>,
}
#[derive(Debug)]
pub enum NumOp {
    Num(f64),
    Operator(Operators),
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
impl TryFrom<&str> for Parsed {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let mut parsed = Vec::new();
        let mut operator_stack: Vec<Operators> = Vec::new();
        let mut chars = value.chars().peekable();
        let mut negate = true;
        while let Some(c) = chars.next() {
            if c.is_ascii_alphabetic() {
                let mut n = c.to_string();
                while let Some(t) = chars.peek() {
                    if t.is_ascii_alphabetic() {
                        n.push(chars.next().unwrap());
                    } else {
                        break;
                    }
                }
                operator_stack.push(Operators::Fun(Function::try_from(n.as_str())?));
                negate = true;
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
    Quadratic,
}
impl TryFrom<&str> for Function {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Ok(match value {
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
            Function::Cos | Function::Sin => 1,
            Function::Atan => 2,
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
