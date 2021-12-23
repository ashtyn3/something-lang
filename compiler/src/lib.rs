use termion::color;
use termion::cursor;
pub mod parse;

#[derive(Clone, Debug, PartialEq)]
pub enum TokenType {
    STRING,
    NUMBER,
    NEGNUMBER,
    LABEL,

    // program types
    EOF,

    // operator types
    LPAREN,
    RPAREN,
    MMARK,

    PLUSBIN,
    SUBBIN,
    MULBIN,
    DIVBIN,
    //comparison
    GCMP,
    GECMP,
    LCMP,
    LECMP,
    ECMP,

    // logic
    AND,
    OR,
}

#[derive(Clone, Debug)]
pub struct LexTokenLoc {
    line_start: usize,
    line: u32,
    col: usize,
    end_col: usize,
}

#[derive(Clone, Debug)]
pub struct LexToken {
    tok_type: TokenType,
    content: String,
    loc: LexTokenLoc,
}

#[derive(Clone, Debug)]
pub struct TokenLoc {
    line_start: usize,
    line: u32,
    col: usize,
}

#[derive(Clone, Debug)]
pub struct Lexer {
    ch: char,
    loc: TokenLoc,
    input: Vec<char>,
    tree: Vec<LexToken>,
}

impl Lexer {
    pub fn new(input: Vec<char>) -> Self {
        Self {
            ch: char::from(input[0]),
            loc: TokenLoc {
                col: 0,
                line: 1,
                line_start: 0,
            },
            input,
            tree: vec![],
        }
    }

    pub fn tree(&self) -> Vec<LexToken> {
        self.tree.to_owned()
    }

    pub fn read(&mut self) {
        self.loc.col += 1;
        if self.loc.col >= self.input.len() {
            self.ch = char::from(0)
        } else {
            self.ch = self.input[self.loc.col];
        }
    }
    pub fn peek(&mut self) -> char {
        if self.loc.col >= self.input.len() - 1 {
            return char::from(0);
        } else {
            self.input[self.loc.col + 1]
        }
    }

    pub fn lex(&mut self) {
        loop {
            if self.ch.is_whitespace() {
                self.read()
            } else if self.ch.is_alphabetic() {
                let start_col = self.loc.col;
                let mut name: String = String::from(self.ch);

                while self.peek().is_alphanumeric() {
                    self.read();
                    name += &self.ch.to_string();
                    //        println!("{}", self.loc.col >= self.input.len());
                }

                self.read();
                self.tree.push(LexToken {
                    tok_type: TokenType::LABEL,
                    content: name,
                    loc: LexTokenLoc {
                        line_start: self.loc.line_start,
                        col: start_col,
                        end_col: self.loc.col,
                        line: self.loc.line,
                    },
                })
            } else if self.ch == '"' {
                let start_col = self.loc.col;
                self.read(); // consume starting "
                let mut str_content: String = String::from(self.ch);

                loop {
                    if self.peek() == '"' {
                        break self.read();
                    }
                    self.read();
                    str_content += &self.ch.to_string();
                }
                self.read();
                self.tree.push(LexToken {
                    tok_type: TokenType::STRING,
                    content: str_content,
                    loc: LexTokenLoc {
                        line_start: self.loc.line_start,
                        col: start_col,
                        end_col: self.loc.col,
                        line: self.loc.line,
                    },
                })
            } else if self.ch.is_numeric() {
                let start_col = self.loc.col;
                let mut num: String = String::from(self.ch);

                while self.peek().is_numeric() || self.peek() == '.' {
                    self.read();
                    num += &self.ch.to_string();
                    //        println!("{}", self.loc.col >= self.input.len());
                }

                self.read();
                self.tree.push(LexToken {
                    tok_type: TokenType::NUMBER,
                    content: num,
                    loc: LexTokenLoc {
                        line_start: self.loc.line_start,
                        col: start_col,
                        end_col: self.loc.col,
                        line: self.loc.line,
                    },
                })
            } else if self.ch == '(' {
                self.read();
                self.tree.push(LexToken {
                    tok_type: TokenType::LPAREN,
                    content: String::from("("),
                    loc: LexTokenLoc {
                        line_start: self.loc.line_start,
                        col: self.loc.col - 1,
                        end_col: self.loc.col,
                        line: self.loc.line,
                    },
                })
            } else if self.ch == ')' {
                self.read();
                self.tree.push(LexToken {
                    tok_type: TokenType::RPAREN,
                    content: String::from(")"),
                    loc: LexTokenLoc {
                        line_start: self.loc.line_start,
                        col: self.loc.col,
                        end_col: self.loc.col,
                        line: self.loc.line,
                    },
                })
            } else if self.ch == '!' {
                self.read();
                self.tree.push(LexToken {
                    tok_type: TokenType::MMARK,
                    content: String::from("!"),
                    loc: LexTokenLoc {
                        line_start: self.loc.line_start,
                        col: self.loc.col,
                        end_col: self.loc.col,
                        line: self.loc.line,
                    },
                })
            } else if self.ch == '-' && self.peek().is_numeric() {
                let start_col = self.loc.col;

                let mut num: String = String::from(self.ch);

                while self.peek().is_numeric() || self.peek() == '.' {
                    self.read();
                    num += &self.ch.to_string();
                }

                self.read();
                self.tree.push(LexToken {
                    tok_type: TokenType::NEGNUMBER,
                    content: num,
                    loc: LexTokenLoc {
                        line_start: self.loc.line_start,
                        col: start_col,
                        end_col: self.loc.col,
                        line: self.loc.line,
                    },
                })
            } else if self.ch == '/' && self.peek() == '/' {
                while self.peek() != '\n' {
                    self.read();
                }
                self.read();
            } else if self.ch == '+'
                || self.ch == '-'
                || (self.ch == '/' && self.peek() != '/')
                || self.ch == '*'
            {
                match self.ch {
                    '+' => self.tree.push(LexToken {
                        tok_type: TokenType::PLUSBIN,
                        content: String::from("+"),
                        loc: LexTokenLoc {
                            line_start: self.loc.line_start,
                            col: self.loc.col,
                            end_col: self.loc.col,
                            line: self.loc.line,
                        },
                    }),
                    '-' => self.tree.push(LexToken {
                        tok_type: TokenType::SUBBIN,
                        content: String::from("+"),
                        loc: LexTokenLoc {
                            line_start: self.loc.line_start,
                            col: self.loc.col,
                            end_col: self.loc.col,
                            line: self.loc.line,
                        },
                    }),

                    '/' => self.tree.push(LexToken {
                        tok_type: TokenType::MULBIN,
                        content: String::from("/"),
                        loc: LexTokenLoc {
                            line_start: self.loc.line_start,
                            col: self.loc.col,
                            end_col: self.loc.col,
                            line: self.loc.line,
                        },
                    }),
                    '*' => self.tree.push(LexToken {
                        tok_type: TokenType::MULBIN,
                        content: String::from("*"),
                        loc: LexTokenLoc {
                            line_start: self.loc.line_start,
                            col: self.loc.col,
                            end_col: self.loc.col,
                            line: self.loc.line,
                        },
                    }),
                    _ => unimplemented!(),
                }
                self.read();
            } else if self.ch == '\n' {
                self.read();
                self.loc.line_start = self.loc.col + 1
            } else if self.ch == char::from(0) {
                break;
            } else {
                let mut code: String = self.input.clone().into_iter().collect();

                code.replace_range(
                    self.loc.col..self.loc.col + 1,
                    &format!(
                        "{}{}{}",
                        termion::style::Bold,
                        color::Fg(color::Red),
                        code.chars().collect::<Vec<char>>()[self.loc.col]
                    ),
                );
                let header = format!(
                    "Unexpected token ({line}:{col}): ",
                    line = self.loc.line,
                    col = self.loc.col,
                );
                eprintln!("{}{}", header, code);
                std::process::exit(1);
            }
        }
    }
}
