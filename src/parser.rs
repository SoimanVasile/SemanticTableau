use crate::formula::Formula;

#[derive(Debug, Clone, PartialEq)]
enum Token {
    Var(String),
    And,        // &
    Or,         // |
    Not,        // ! sau ~
    Implies,    // ->
    LParen,     // (
    RParen,     // )
}

fn tokenize(input: &str) -> Vec<Token> {
    let mut tokens = Vec::new();
    let mut chars = input.chars().peekable();

    while let Some(&c) = chars.peek() {
        match c {
            ' ' | '\t' | '\n' => { chars.next(); }
            '(' => { tokens.push(Token::LParen); chars.next(); },
            ')' => { tokens.push(Token::RParen); chars.next(); },
            '&' | '^' => { tokens.push(Token::And); chars.next(); },
            '|' | 'v' => { tokens.push(Token::Or); chars.next(); },
            '!' | '~' | '¬' => { tokens.push(Token::Not); chars.next(); },
            '-' => {
                chars.next();
                if let Some('>') = chars.peek() {
                    tokens.push(Token::Implies);
                    chars.next();
                } else {
                    panic!("Caracter neașteptat după '-': așteptam '>'");
                }
            },
            _ if c.is_alphanumeric() => {
                let mut name = String::new();
                while let Some(&ch) = chars.peek() {
                    if ch.is_alphanumeric() {
                        name.push(ch);
                        chars.next();
                    } else {
                        break;
                    }
                }
                tokens.push(Token::Var(name));
            },
            _ => panic!("Caracter invalid: {}", c),
        }
    }
    tokens
}

struct Parser {
    tokens: Vec<Token>,
    pos: usize,
}

impl Parser {
    fn new(tokens: Vec<Token>) -> Self {
        Parser { tokens, pos: 0 }
    }

    fn current(&self) -> Option<&Token> {
        self.tokens.get(self.pos)
    }

    fn advance(&mut self) {
        self.pos += 1;
    }

    fn expect(&mut self, token: Token) {
        if self.current() == Some(&token) {
            self.advance();
        } else {
            panic!("Eroare de sintaxă: Așteptam {:?}, am găsit {:?}", token, self.current());
        }
    }

    pub fn parse(&mut self) -> Formula {
        self.parse_implies()
    }

    fn parse_implies(&mut self) -> Formula {
        let mut left = self.parse_or();

        while let Some(Token::Implies) = self.current() {
            self.advance();
            let right = self.parse_implies();
            left = Formula::implies(left, right);
        }
        left
    }

    fn parse_or(&mut self) -> Formula {
        let mut left = self.parse_and();

        while let Some(Token::Or) = self.current() {
            self.advance();
            let right = self.parse_and();
            left = Formula::or(left, right);
        }
        left
    }

    fn parse_and(&mut self) -> Formula {
        let mut left = self.parse_unary();

        while let Some(Token::And) = self.current() {
            self.advance();
            let right = self.parse_unary();
            left = Formula::and(left, right);
        }
        left
    }

    fn parse_unary(&mut self) -> Formula {
        match self.current() {
            Some(Token::Not) => {
                self.advance();
                let operand = self.parse_unary();
                Formula::not(operand)
            },
            Some(Token::Var(name)) => {
                let f = Formula::var(name);
                self.advance();
                f
            },
            Some(Token::LParen) => {
                self.advance();
                let expr = self.parse_implies();
                self.expect(Token::RParen);
                expr
            },
            _ => panic!("Sintaxă invalidă la poziția {}: Așteptam Variabilă, ! sau (", self.pos),
        }
    }
}

pub fn parse_formula(input: &str) -> Formula {
    let tokens = tokenize(input);
    let mut parser = Parser::new(tokens);
    parser.parse()
}
