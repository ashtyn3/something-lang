use crate::LexToken;
use crate::LexTokenLoc;
use crate::TokenType;

#[derive(Clone, Debug, PartialEq)]
pub enum ParseType {
    EXP,
    FNCALL,
    FNMAKE,
}

#[derive(Clone, Debug)]
pub struct EXP {
    body: Vec<ParseTok>,
}

#[derive(Clone, Debug)]
pub struct ParseLoc {
    start_col: usize,
    end_col: usize,
    line: u32,
    line_start: usize,
}

#[derive(Clone, Debug)]
pub struct ParseTok {
    tok_type: ParseType,
}

#[derive(Clone, Debug)]
pub struct Parser {
    tok: LexToken,
    tree: Vec<ParseTok>,
    lex_tree: Vec<LexToken>,
    lex_id: usize,
    pub file: String,
}

fn op_prec(op: &str) -> u16 {
    match op {
        "+" => 10,
        "-" => 10,

        "*" => 20,
        "/" => 20,

        "(" => 40,
        ")" => 40,

        "&&" => 30,
        "||" => 30,
        "<" => 30,
        ">" => 30,
        "<=" => 30,
        ">=" => 30,

        _ => 0,
    }
}
fn _op_asso(op: &str) -> &str {
    match op {
        "+" => "left",
        "-" => "left",
        "/" => "left",
        "*" => "left",

        // comparison
        "&&" => "left",
        "||" => "left",
        "<" => "left",
        ">" => "left",
        "<=" => "left",
        ">=" => "left",

        _ => "right",
    }
}

//fn rpn_exp_wrapper() {}

impl Parser {
    pub fn new(lex_tree: Vec<LexToken>, f: String) -> Self {
        Self {
            tok: lex_tree[0].to_owned(),
            tree: vec![],
            lex_tree,
            lex_id: 0,
            file: f,
        }
    }
    pub fn next_tok(&mut self) {
        self.lex_id += 1;
        if self.lex_id >= self.lex_tree.len() {
            self.tok = LexToken {
                tok_type: TokenType::EOF,
                content: String::from(""),
                loc: LexTokenLoc {
                    ..self.lex_tree.last().unwrap().loc
                },
            }
        } else {
            self.tok = self.lex_tree[self.lex_id].to_owned();
        }
    }

    pub fn peek(&mut self) -> LexToken {
        self.lex_tree[self.lex_id + 1].clone()
    }

    pub fn parse_exp(&mut self) -> ParseTok {
        let mut paren_count = 0;
        let start_col = self.tok.loc.col;
        let mut end_col = start_col + 1;
        let sub_tree: &mut Vec<LexToken> = &mut vec![];

        loop {
            if self.tok.tok_type == TokenType::LPAREN {
                paren_count += 1;
                sub_tree.push(self.tok.clone());
            } else if self.tok.tok_type == TokenType::RPAREN {
                paren_count -= 1;
                sub_tree.push(self.tok.clone());
            } else {
                sub_tree.push(self.tok.clone());
            }

            if self.tok.tok_type == TokenType::RPAREN && paren_count == 0 {
                end_col = self.lex_id;
                break;
            }
            self.next_tok();
        }

        let mut stack: Vec<LexToken> = vec![];
        let mut op_stack: Vec<LexToken> = vec![];

        for lex_t in &mut sub_tree.to_owned().clone() {
            let current = lex_t.clone();
            if current.tok_type == TokenType::NUMBER || current.tok_type == TokenType::NEGNUMBER {
                self.parse();
                stack.push(current.clone());
            } else if current.tok_type == TokenType::SUBBIN
                || current.tok_type == TokenType::PLUSBIN
                || current.tok_type == TokenType::DIVBIN
                || current.tok_type == TokenType::MULBIN
            {
                while op_stack.len() != 0
                    && op_prec(&current.content) <= op_prec(&op_stack.last().unwrap().content)
                    && op_stack.last().unwrap().content != "("
                {
                    let curr_op = op_stack.pop();
                    if curr_op.is_some() {
                        stack.push(curr_op.unwrap())
                    }
                }
                op_stack.push(current);
            } else if current.tok_type == TokenType::LPAREN {
                op_stack.push(current);
            } else if current.tok_type == TokenType::RPAREN {
                let mut found_paren_match = false;
                while op_stack.last().is_some() && op_stack.last().unwrap().content != ")" {
                    if op_stack.last().unwrap().tok_type == TokenType::LPAREN {
                        op_stack.pop();
                        found_paren_match = true;
                    } else {
                        stack.push(op_stack.pop().unwrap());
                    }
                    if op_stack.len() == 0 {
                        //TODO: Add error for mismatched parens.
                        break;
                    }
                }
                if found_paren_match == false {
                    let mut content: String = String::from("");
                    for s in &mut *sub_tree {
                        content += &s.content
                    }
                    println!(
                        "Syntax Error ({line}:{col}): {content}",
                        line = current.loc.line,
                        col = sub_tree.first().unwrap().loc.col,
                        content = content
                    )
                }
            } else {
                stack.push(current);
            }
        }

        for op in op_stack.to_owned() {
            if op.content != "(" {
                stack.push(op_stack.pop().unwrap())
            }
        }

        //println!("{:?}", sub_tree);
        //println!("{:#?}", stack);
        let bin_tree = Parser::new(stack, self.file.clone());
        println!("{:?}", bin_tree.tree());

        ParseTok {
            tok_type: ParseType::EXP,
        }
    }

    pub fn tree(self) -> Vec<ParseTok> {
        self.tree
    }

    pub fn file_(self) -> String {
        self.file
    }

    pub fn parse(&mut self) -> ParseTok {
        if self.tok.tok_type == TokenType::LPAREN {
            self.parse_exp()
        } else {
            ParseTok {
                tok_type: ParseType::EXP,
            }
        }
    }
}
