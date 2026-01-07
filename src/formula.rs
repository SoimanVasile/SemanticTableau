use std::fmt;

#[derive(Clone, PartialEq, Eq, Hash)]
pub enum Formula {
    Var(String),
    Not(Box<Formula>),
    And(Box<Formula>, Box<Formula>),
    Or(Box<Formula>, Box<Formula>),
    Implies(Box<Formula>, Box<Formula>),
}

impl fmt::Display for Formula {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Formula::Var(n) => write!(f, "{}", n),
            Formula::Not(inner) => write!(f, "¬{}", inner), 
            Formula::And(l, r) => write!(f, "({} ∧ {})", l, r),
            Formula::Or(l, r) => write!(f, "({} ∨ {})", l, r),
            Formula::Implies(l, r) => write!(f, "({} → {})", l, r),
        }
    }
}

impl Formula {
    pub fn var(n: &str) -> Formula { Formula::Var(n.to_string()) }
    pub fn not(f: Formula) -> Formula { Formula::Not(Box::new(f)) }
    pub fn and(l: Formula, r: Formula) -> Formula { Formula::And(Box::new(l), Box::new(r)) }
    pub fn or(l: Formula, r: Formula) -> Formula { Formula::Or(Box::new(l), Box::new(r)) }
    pub fn implies(l: Formula, r: Formula) -> Formula { Formula::Implies(Box::new(l), Box::new(r)) }
    
    pub fn is_literal(&self) -> bool {
        match self {
            Formula::Var(_) => true,
            Formula::Not(f) => matches!(**f, Formula::Var(_)),
            _ => false,
        }
    }

    pub fn is_contradiction_with(&self, other: &Formula) -> bool {
        match self {
            Formula::Not(inner) => **inner == *other,
            _ => match other {
                Formula::Not(inner) => **inner == *self,
                _ => false
            }
        }
    }
}
