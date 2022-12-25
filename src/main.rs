#[derive(Debug, PartialEq, Clone)]
enum Token {
    Number(i64),
    Plus,
    Minus,
    Asterisk,
    Slash,
    LParen,
    RParen,
    GreaterThan,
    LessThan,
    If,
    Then,
    Else,
    Print,
    SemiColon,
    LBrace,
    RBrace,
    While,
    Assign,
    EOF,
}

struct Lexer<'a> {
    input: &'a str,
    current_char: Option<String>,
    position: usize,
}

impl<'a> Lexer<'a> {
    fn new(input: &'a str) -> Self {
        let mut lexer = Lexer {
            input,
            current_char: None,
            position: 0,
        };
        lexer.advance();
        return lexer;
    }

    fn advance(&mut self) {
        if self.position >= self.input.len() {
            self.current_char = None;
        } else {
            self.current_char = Some(self.input[self.position..=self.position].to_string());
            self.position += 1;
        }
    }

    fn skip_whitespace(&mut self) {
        while let Some(c) = &self.current_char {
            if !c.chars().nth(0).unwrap().is_whitespace() {
                break;
            }
            self.advance();
        }
    }

    fn integer(&mut self) -> i64 {
        let mut result = 0;
        while let Some(c) = &self.current_char {
            if !c.chars().nth(0).unwrap().is_digit(10) {
                break;
            }
            result = result * 10 + c.chars().nth(0).unwrap().to_digit(10).unwrap() as i64;
            self.advance();
        }
        return result;
    }

    fn next_token(&mut self) -> Token {
        while let Some(c) = &self.current_char {
            if c.chars().nth(0).unwrap().is_whitespace() {
                self.skip_whitespace();
                continue;
            }

            if c.chars().nth(0).unwrap().is_digit(10) {
                return Token::Number(self.integer());
            }

            match c.as_str() {
                "+" => {
                    self.advance();
                    return Token::Plus;
                }
                "-" => {
                    self.advance();
                    return Token::Minus;
                }
                "*" => {
                    self.advance();
                    return Token::Asterisk;
                }
                "/" => {
                    self.advance();
                    return Token::Slash;
                }
                "(" => {
                    self.advance();
                    return Token::LParen;
                }
                ")" => {
                    self.advance();
                    return Token::RParen;
                }
                "\n" => {
                    self.advance();
                    return Token::EOF;
                }
                ">" => {
                    self.advance();
                    return Token::GreaterThan;
                }
                "<" => {
                    self.advance();
                    return Token::LessThan;
                }
                ";" => {
                    self.advance();
                    return Token::SemiColon;
                }
                "{" => {
                    self.advance();
                    return Token::LBrace;
                }
                "}" => {
                    self.advance();
                    return Token::RBrace;
                }
                "=" => {
                    self.advance();
                    return Token::Assign;
                }
                _ => {
                    // Check if the current character is the start of a keyword
                    if c.chars().nth(0).unwrap().is_alphabetic() {
                        let mut keyword = c.to_string();
                        self.advance();
                        while let Some(nc) = &self.current_char {
                            if nc.chars().nth(0).unwrap().is_alphabetic() {
                                keyword.push_str(nc);
                                self.advance();
                            } else {
                                break;
                            }
                        }
                        // Match the keyword and return the corresponding token
                        return match keyword.as_str() {
                            "if" => Token::If,
                            "then" => Token::Then,
                            "else" => Token::Else,
                            "print" => Token::Print,
                            "while" => Token::While,
                            _ => panic!("Invalid character"),
                        };
                    } else {
                        panic!("Invalid character");
                    }
                }
            }
        }
        return Token::EOF;
    }
}

#[derive(Debug, PartialEq)]
enum AST {
    BinOp {
        op: Token,
        left: Box<AST>,
        right: Box<AST>,
    },
    Num(i64),
    IfThenElse {
        condition: Box<AST>,
        then_branch: Box<AST>,
        else_branch: Box<AST>,
    },
    KeywordPrint(Box<AST>),
    WhileLoop {
        condition: Box<AST>,
        body: Box<AST>,
        // increment: i64,
    },
    Identifier(String),
    Assignment {
        identifier: String,
        value: Box<AST>,
    },
}

struct Parser<'a> {
    lexer: Lexer<'a>,
    current_token: Token,
}

impl<'a> Parser<'a> {
    fn new(lexer: Lexer<'a>) -> Self {
        let mut parser = Parser {
            lexer,
            current_token: Token::EOF,
        };
        parser.advance();
        return parser;
    }

    fn advance(&mut self) {
        self.current_token = self.lexer.next_token();
    }

    fn factor(&mut self) -> AST {
        let token = self.current_token.clone();
        match token {
            Token::Number(value) => {
                self.advance();
                AST::Num(value)
            }
            Token::LParen => {
                self.advance();
                let result = self.expr();
                if self.current_token != Token::RParen {
                    panic!("Expected ')'");
                }
                self.advance();
                return result;
            }
            Token::Print => {
                self.advance();
                let result = self.expr();
                return AST::KeywordPrint(Box::new(result));
            }
            _ => panic!("Unexpected token: {:?}", token),
        }
    }

    fn term(&mut self) -> AST {
        let mut result = self.factor();
        while let Some(op) = self.current_token.clone().into_op() {
            if op == Token::Asterisk || op == Token::Slash {
                self.advance();
                result = AST::BinOp {
                    op,
                    left: Box::new(result),
                    right: Box::new(self.factor()),
                }
            } else {
                break;
            }
        }
        return result;
    }

    fn eat(&mut self, token: Token) {
        if self.current_token != token {
            panic!(
                "Expected token {:?} but got {:?}",
                token, self.current_token
            );
        }
        self.current_token = self.lexer.next_token();
    }

    fn while_expr(&mut self) -> AST {
        self.eat(Token::While);
        let condition = self.expr();
        self.eat(Token::LBrace);
        let body = self.expr();
        self.eat(Token::RBrace);

        AST::WhileLoop {
            condition: Box::new(condition),
            body: Box::new(body),
        }
    }

    fn if_then_else(&mut self) -> AST {
        self.advance(); // consume the "if" keyword
        let condition = self.expr();
        if self.current_token != Token::Then {
            panic!("Expected 'then' keyword");
        }
        self.advance(); // consume the "then" keyword
        let then_branch = self.expr();
        if self.current_token != Token::Else {
            panic!("Expected 'else' keyword");
        }
        self.advance(); // consume the "else" keyword
        let else_branch = self.expr();

        AST::IfThenElse {
            condition: Box::new(condition),
            then_branch: Box::new(then_branch),
            else_branch: Box::new(else_branch),
        }
    }

    fn lowest_precedence(&mut self, mut left: AST) -> AST {
        while let Some(op) = self.current_token.clone().into_op() {
            if op == Token::Plus
                || op == Token::Minus
                || op == Token::GreaterThan
                || op == Token::LessThan
            {
                self.advance();
                left = AST::BinOp {
                    op,
                    left: Box::new(left),
                    right: Box::new(self.term()),
                };
            } else {
                break;
            }
        }
        return left;
    }

    fn expr(&mut self) -> AST {
        let mut result = self.term();
        while let Some(op) = self.current_token.clone().into_op() {
            if op == Token::Asterisk || op == Token::Slash {
                self.advance();
                result = AST::BinOp {
                    op,
                    left: Box::new(result),
                    right: Box::new(self.factor()),
                };
            } else if op == Token::While {
                self.advance();
                result = self.while_expr();
            } else {
                break;
            }
        }

        // Check for assignment operator
        if let Token::Assign = self.current_token {
            // Check if the left hand side of the assignment is a variable
            if let AST::Identifier(var) = result {
                self.advance();
                let value = self.expr();
                return AST::Assignment {
                    identifier: var,
                    value: Box::new(value),
                };
            } else {
                panic!("Invalid assignment");
            }
        }

        result = self.lowest_precedence(result);
        if self.current_token == Token::If {
            result = self.if_then_else();
        }
        result
    }
}

trait IntoOp {
    fn into_op(self) -> Option<Token>;
}

impl IntoOp for Token {
    fn into_op(self) -> Option<Token> {
        match self {
            Token::Plus
            | Token::Minus
            | Token::Asterisk
            | Token::Slash
            | Token::GreaterThan
            | Token::LessThan => Some(self),
            _ => None,
        }
    }
}

struct Interpreter {
    symbol_table: std::collections::HashMap<String, i64>,
}

impl Interpreter {
    fn eval(&mut self, node: &AST) -> i64 {
        match *node {
            AST::Num(value) => value,
            AST::BinOp {
                ref op,
                ref left,
                ref right,
            } => {
                let left_value = self.eval(left);
                let right_value = self.eval(right);
                match *op {
                    Token::Plus => left_value + right_value,
                    Token::Minus => left_value - right_value,
                    Token::Asterisk => left_value * right_value,
                    Token::Slash => left_value / right_value,
                    Token::GreaterThan => (left_value > right_value) as i64,
                    Token::LessThan => (left_value < right_value) as i64,
                    _ => unreachable!(),
                }
            }
            AST::IfThenElse {
                ref condition,
                ref then_branch,
                ref else_branch,
            } => {
                if self.eval(condition) != 0 {
                    return self.eval(then_branch);
                } else {
                    return self.eval(else_branch);
                }
            }
            AST::KeywordPrint(ref expr) => {
                let value = self.eval(expr);
                return value;
            }

            AST::Identifier(ref var_name) => {
                // Look up the value of the variable in the symbol table
                let var_value = *self.symbol_table.get(var_name).unwrap();
                return var_value;
            }

            AST::Assignment {
                ref identifier,
                ref value,
            } => {
                let variable_value = self.eval(value);
                self.symbol_table.insert(identifier.to_string(), variable_value);
                variable_value
            }

            AST::WhileLoop {
                ref condition,
                ref body,
            } => {
                let result = 1;
                while self.eval(condition) != 0 {
                    println!("{}", self.eval(body));
                }
                return result;
            }
        }
    }
}

fn main() {
    println!("Hello, world!");
    let lexer = Lexer::new("2 + 2 * 2");
    let mut parser = Parser::new(lexer);
    let ast = parser.expr();
    println!("AST: {:?}", ast);

    let mut interpreter = Interpreter {
        symbol_table: std::collections::HashMap::new(),
    };

    println!("Result: {}", interpreter.eval(&ast));

    println!("=============================");
    let lexer_if = Lexer::new("if 3 < 2 then 1+2 else 1*4");
    let mut parser_if = Parser::new(lexer_if);
    let ast_if = parser_if.if_then_else();
    println!("AST: {:?}", ast_if);

    let mut interpreter_if = Interpreter {
        symbol_table: std::collections::HashMap::new(),
    };

    println!("Result: {}", interpreter_if.eval(&ast_if));

    println!("=============================");
    let lexer_print = Lexer::new("print 2+2*2");
    let mut parser_print = Parser::new(lexer_print);
    let ast_print = parser_print.expr();
    println!("AST: {:?}", ast_print);

    let mut interpreter_print = Interpreter {
        symbol_table: std::collections::HashMap::new(),
    };

    println!("Result: {}", interpreter_print.eval(&ast_print));

    // println!("=============================");
    // let lexer_id = Lexer::new("a=4");
    // let mut parser_id = Parser::new(lexer_id);
    // let ast_id = parser_id.expr();
    // println!("AST: {:?}", ast_id);

    // let mut interpreter_id = Interpreter {
    //     symbol_table: std::collections::HashMap::new(),
    // };

    // println!("Result: {}", interpreter_id.eval(&ast_id));
}
