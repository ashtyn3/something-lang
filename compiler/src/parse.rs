use crate::LexToken;
use crate::LexTokenLoc;
use crate::TokenType;
use std::collections::HashMap;

#[derive(Clone, Debug, PartialEq)]
pub enum ParseType {
    EXP,
    RPNEXP,
    OPERATOR,
    NUMBER,
    STRING,
    FNCALL,
    FNMAKE,
    LABEL,
    VARDEF,
}

#[derive(Clone, Debug, PartialEq)]
pub enum Primitives {
    INT,
    SIGINT,
    FLOAT,
    SIGFLOAT,
    STRING,
    KEYWORD,
    FUNCTION,
    OPERATOR,
}

#[derive(Clone, Debug)]
pub enum BinOperand {
    PLUS,
    SUB,
    MUL,
    DIV,
}

#[derive(Clone, Debug)]
pub struct VarInit {
    value_type: String,
    name: String,
    value: ParseTok,
}

#[derive(Clone, Debug)]
pub struct BinSeg {
    left: ParseTok,
    right: Option<ParseTok>,

    operation: BinOperand,
}

#[derive(Clone, Debug)]
pub struct Exp {
    exp_type: Primitives,
    body: Vec<BinSeg>,
}

#[derive(Clone, Debug)]
pub struct Operand {
    op_type: BinOperand,
}

#[derive(Clone, Debug)]
pub struct Number {
    num_type: Primitives,
    number: Option<i64>,
    float: Option<f64>,
}

#[derive(Clone, Debug)]
pub struct Label {
    var_type: String,
    name: String,
}

#[derive(Clone, Debug)]
pub struct StringT {
    length: usize,
    content: String,
}
#[derive(Clone, Debug)]
pub struct ParseLoc {
    start_col: usize,
    end_col: usize,
    line: u32,
}

#[derive(Clone, Debug)]
pub struct ParseTok {
    tok_type: ParseType,
    location: ParseLoc,
    expression: Option<Exp>,
    number: Option<Number>,
    string: Option<StringT>,
    operand: Option<Operand>,
    ident: Option<Label>,
    variable: Box<Option<VarInit>>,
}

#[derive(Clone, Debug)]
pub struct Parser {
    tok: LexToken,
    tree: Vec<ParseTok>,
    lex_tree: Vec<LexToken>,
    lex_id: usize,
    pub file: String,

    curr_scope: HashMap<String, ParseTok>,
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
    pub fn new(lex_tree: Vec<LexToken>, f: String, scope: HashMap<String, ParseTok>) -> Self {
        Self {
            tok: lex_tree[0].to_owned(),
            tree: vec![],
            lex_tree,
            lex_id: 0,
            file: f,
            curr_scope: scope,
        }
    }
    fn get_prim(&mut self, tok: ParseTok) -> Primitives {
        match tok.tok_type {
            ParseType::NUMBER => tok.number.unwrap().num_type,
            ParseType::STRING => Primitives::STRING,
            ParseType::OPERATOR => Primitives::OPERATOR,
            _ => {
                println!(
                    "Unknown Primitive ({line},{col}): {:?}",
                    tok.tok_type,
                    line = self.tok.loc.line,
                    col = self.tok.loc.col
                );
                std::process::exit(1);
            }
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
        if self.lex_id + 1 >= self.lex_tree.len() {
            LexToken {
                tok_type: TokenType::EOF,
                content: String::from(""),
                loc: LexTokenLoc {
                    ..self.lex_tree.last().unwrap().loc
                },
            }
        } else {
            self.lex_tree[self.lex_id + 1].clone()
        }
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
                end_col = self.tok.loc.end_col;
                break;
            }
            self.next_tok();
        }

        let mut stack: Vec<LexToken> = vec![];
        let mut op_stack: Vec<LexToken> = vec![];

        for lex_t in &mut sub_tree.to_owned().clone() {
            let current = lex_t.clone();
            if current.tok_type == TokenType::NUMBER || current.tok_type == TokenType::NEGNUMBER {
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
        stack.insert(
            0,
            LexToken {
                tok_type: TokenType::LABEL,
                content: "RPN".to_string(),
                loc: LexTokenLoc {
                    col: 0,
                    end_col: 0,
                    line_start: 0,
                    line: self.tok.loc.line,
                },
            },
        );
        stack.insert(
            1,
            LexToken {
                tok_type: TokenType::MMARK,
                content: "!".to_string(),
                loc: LexTokenLoc {
                    col: 0,
                    end_col: 0,
                    line_start: 0,
                    line: self.tok.loc.line,
                },
            },
        );
        let mut bin_tree = Parser::new(stack.clone(), self.file.clone(), self.curr_scope.clone());
        // println!("{:#?}", &stack);
        let out = bin_tree.parse();

        self.next_tok();
        if out.expression.is_some() {
            ParseTok {
                tok_type: ParseType::EXP,
                location: ParseLoc {
                    line: stack.last().unwrap().loc.line,
                    start_col,
                    end_col,
                },
                number: None,
                expression: Some(Exp {
                    exp_type: out.clone().expression.unwrap().exp_type,
                    body: out.clone().expression.unwrap().body,
                }),
                string: None,
                operand: None,
                ident: None,
                variable: Box::new(None),
            }
        } else {
            unimplemented!()
        }
    }

    pub fn parse_int(&mut self) -> ParseTok {
        if self.tok.content.contains(".") && self.tok.content.starts_with("-") == false {
            ParseTok {
                tok_type: ParseType::NUMBER,
                location: ParseLoc {
                    start_col: self.tok.loc.col,
                    end_col: self.tok.loc.end_col,
                    line: self.tok.loc.line,
                },
                number: Some(Number {
                    num_type: Primitives::FLOAT,
                    float: Some(self.tok.content.parse::<f64>().unwrap()),
                    number: None,
                }),
                expression: None,
                string: None,
                operand: None,
                ident: None,
                variable: Box::new(None),
            }
        } else if self.tok.content.contains(".") && self.tok.content.starts_with("-") == true {
            ParseTok {
                tok_type: ParseType::NUMBER,
                location: ParseLoc {
                    start_col: self.tok.loc.col,
                    end_col: self.tok.loc.end_col,
                    line: self.tok.loc.line,
                },
                number: Some(Number {
                    num_type: Primitives::SIGFLOAT,
                    float: Some(self.tok.content.parse::<f64>().unwrap()),
                    number: None,
                }),
                expression: None,
                string: None,
                operand: None,
                ident: None,
                variable: Box::new(None),
            }
        } else if self.tok.content.contains(".") == false
            && self.tok.content.starts_with("-") == true
        {
            ParseTok {
                tok_type: ParseType::NUMBER,
                location: ParseLoc {
                    start_col: self.tok.loc.col,
                    end_col: self.tok.loc.end_col,
                    line: self.tok.loc.line,
                },
                number: Some(Number {
                    num_type: Primitives::SIGINT,
                    number: Some(self.tok.content.parse::<i64>().unwrap()),
                    float: None,
                }),
                expression: None,
                string: None,
                operand: None,
                ident: None,
                variable: Box::new(None),
            }
        } else {
            ParseTok {
                tok_type: ParseType::NUMBER,
                location: ParseLoc {
                    start_col: self.tok.loc.col,
                    end_col: self.tok.loc.end_col,
                    line: self.tok.loc.line,
                },
                number: Some(Number {
                    num_type: Primitives::INT,
                    number: Some(self.tok.content.parse::<i64>().unwrap()),
                    float: None,
                }),
                expression: None,
                string: None,
                operand: None,
                ident: None,
                variable: Box::new(None),
            }
        }
    }

    pub fn parse_operand(&mut self) -> ParseTok {
        let op_type: BinOperand = match self.tok.tok_type {
            TokenType::SUBBIN => BinOperand::SUB,
            TokenType::PLUSBIN => BinOperand::PLUS,
            TokenType::MULBIN => BinOperand::MUL,
            TokenType::DIVBIN => BinOperand::DIV,
            _ => unimplemented!(),
        };
        let tok = ParseTok {
            tok_type: ParseType::OPERATOR,
            location: ParseLoc {
                start_col: self.tok.loc.col,
                end_col: self.tok.loc.end_col,
                line: self.tok.loc.line,
            },
            number: None,
            expression: None,
            string: None,
            operand: Some(Operand { op_type }),
            ident: None,
            variable: Box::new(None),
        };
        return tok;
    }

    pub fn parse_ident(&mut self) -> ParseTok {
        let var = self.curr_scope.get(&self.tok.content);
        if var.is_none() == true {
            println!(
                "Undeclared variable ({line}:{col}): Cannot find variable {}",
                self.tok.content,
                line = self.tok.loc.line,
                col = self.tok.loc.col
            );
            std::process::exit(1)
        }

        let var_data = var.unwrap();
        ParseTok {
            tok_type: ParseType::LABEL,
            location: ParseLoc {
                start_col: self.tok.loc.col,
                end_col: self.tok.loc.end_col,
                line: self.tok.loc.line,
            },
            ident: Some(Label {
                var_type: var_data.variable.clone().unwrap().value_type,
                name: var_data.variable.clone().unwrap().name,
            }),
            expression: None,
            number: None,
            string: None,
            operand: None,
            variable: Box::new(None),
        }
    }

    pub fn parse_rpn_exp(&mut self) -> ParseTok {
        self.next_tok(); // eat RPN
        self.next_tok(); // eat !

        let mut prim_type: Primitives = Primitives::INT;
        let mut count = 0;

        let mut output_tree: Vec<BinSeg> = vec![];
        let mut working_stack = vec![];
        while true == true {
            let parse_out = Parser::new(
                vec![self.tok.clone()],
                self.file.to_owned(),
                self.curr_scope.to_owned(),
            )
            .parse();
            if count == 0 {
                let whole_type = parse_out.tok_type.clone();
                if whole_type == ParseType::NUMBER {
                    prim_type = parse_out.clone().number.unwrap().num_type;
                } else if whole_type == ParseType::STRING {
                    prim_type = Primitives::STRING;
                }
            }

            let tok_prim = self.get_prim(parse_out.clone());
            if tok_prim != prim_type && tok_prim != Primitives::OPERATOR {
                println!(
                    "Bad Types ({line},{col}): Cannot use type {:?} with type {:?}",
                    tok_prim,
                    prim_type,
                    line = self.tok.loc.line,
                    col = self.tok.loc.col
                );
                std::process::exit(1);
            }

            if parse_out.tok_type == ParseType::NUMBER {
                working_stack.push(parse_out.clone())
            } else if parse_out.tok_type == ParseType::LABEL {
                working_stack.push(parse_out.clone())
            } else if tok_prim == Primitives::OPERATOR {
                let left = working_stack.pop();
                let right = working_stack.pop();
                if prim_type == Primitives::INT
                    || prim_type == Primitives::SIGINT
                    || prim_type == Primitives::FLOAT
                    || prim_type == Primitives::SIGFLOAT
                {
                    if right.is_some() {
                        output_tree.push(BinSeg {
                            left: left.unwrap(),
                            right: Some(right.unwrap()),
                            operation: parse_out.operand.unwrap().op_type,
                        })
                    } else {
                        output_tree.push(BinSeg {
                            left: left.unwrap(),
                            right: None,
                            operation: parse_out.operand.unwrap().op_type,
                        })
                    }
                }
            } else {
                unimplemented!()
            }

            if self.peek().tok_type == TokenType::EOF {
                break;
            };
            count += 1;
            self.next_tok()
        }

        return ParseTok {
            tok_type: ParseType::EXP,
            location: ParseLoc {
                start_col: self.tok.loc.col,
                end_col: self.tok.loc.end_col,
                line: self.tok.loc.line,
            },
            expression: Some(Exp {
                exp_type: prim_type,
                body: output_tree,
            }),
            number: None,
            string: None,
            operand: None,
            ident: None,
            variable: Box::new(None),
        };
    }
    pub fn parse_var_def(&mut self) -> ParseTok {
        let start_col = self.tok.loc.col;
        let name = self.tok.content.clone();
        self.next_tok(); // consume :
        if self.peek().tok_type != TokenType::LABEL {
            println!(
                "Unknown parser token ({line}:{col}): Expected token of type LABEL instead got {:?}",
                self.peek().tok_type,
                line = self.tok.loc.line,
                col = self.tok.loc.col
            );
            std::process::exit(1);
        }
        self.next_tok();
        let var_type = self.tok.content.clone();
        if self.peek().tok_type != TokenType::COLON {
            println!(
                "Unknown parser token ({line}:{col}): Expected token of type LABEL instead got {:?}",
                self.peek().tok_type,
                line = self.tok.loc.line,
                col = self.tok.loc.col
            );
            std::process::exit(1);
        }
        //TODO: Add syntax for varName:int,int: tuple syntax
        self.next_tok();
        self.next_tok();
        let mut sub_tree = vec![];
        while self.tok.content != ";" {
            sub_tree.push(self.tok.clone());
            self.next_tok()
        }
        let body = Parser::new(sub_tree, self.file.clone(), self.curr_scope.clone()).parse();

        self.next_tok();
        return ParseTok {
            tok_type: ParseType::EXP,
            location: ParseLoc {
                start_col,
                end_col: self.tok.loc.end_col,
                line: self.tok.loc.line,
            },
            expression: None,
            number: None,
            string: None,
            operand: None,
            ident: None,
            variable: Box::new(Some(VarInit {
                value_type: var_type.to_string(),
                name: name.to_string(),
                value: body,
            })),
        };
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
        } else if self.tok.content == "RPN" && self.peek().tok_type == TokenType::MMARK {
            self.parse_rpn_exp()
        } else if self.tok.tok_type == TokenType::LABEL && self.peek().content == ":" {
            self.parse_var_def()
        } else if self.tok.tok_type == TokenType::NUMBER
            || self.tok.tok_type == TokenType::NEGNUMBER
        {
            self.parse_int()
        } else if self.tok.tok_type == TokenType::LABEL {
            self.parse_ident()
        } else if self.tok.tok_type == TokenType::PLUSBIN
            || self.tok.tok_type == TokenType::SUBBIN
            || self.tok.tok_type == TokenType::MULBIN
            || self.tok.tok_type == TokenType::DIVBIN
            || self.tok.tok_type == TokenType::COLON
        {
            self.parse_operand()
        } else {
            println!(
                "Unknown parser token ({line},{col}): {}",
                self.tok.content,
                line = self.tok.loc.line,
                col = self.tok.loc.col
            );
            println!("Of type: {:?}", self.tok.tok_type);
            std::process::exit(1);
        }
    }
    pub fn init(&mut self) {
        while true == true {
            let res = &self.parse();
            self.tree.push(res.to_owned());
            if self.tok.tok_type == TokenType::EOF {
                break;
            }
        }
    }
}
