use crate::parse;
use indexmap::IndexMap;
use nanoid::nanoid;

pub struct Function {
    name: String,
    is_main: bool,
    lines: Vec<String>,
}

#[derive(Debug)]
pub struct PrimType {
    pub def: String,
    pub name: String,
}

#[derive(Debug, Clone)]
pub struct DescriptorToken {
    pub token_real_type: Option<parse::Primitives>,
    pub token: parse::ParseTok,
}

pub fn prim_var_str(type_: parse::Primitives) -> Vec<String> {
    match type_ {
        parse::Primitives::INT(size) => vec!["INT".to_string(), size.to_string()],
        parse::Primitives::SIGINT(size) => vec!["SIGINT".to_string(), size.to_string()],
        parse::Primitives::FLOAT(size) => vec!["FLOAT".to_string(), size.to_string()],
        _ => vec![format!("{:?}", type_)],
    }
}

pub fn init_str_lit(definitions: &mut IndexMap<parse::Primitives, PrimType>) {
    definitions.insert(
        parse::Primitives::STRING,
        PrimType {
            def: "
struct STR_LIT {
  int length;
  string chs;
  string display() { return chs; };
  str_lit(string str) : chs(str);
};
            "
            .to_string(),
            name: String::from("STR_LIT"),
        },
    );
}

pub fn init_int_lit(definitions: &mut IndexMap<parse::Primitives, PrimType>, size: String) {
    definitions.insert(
        parse::Primitives::INT(size.parse::<i8>().unwrap()),
        PrimType {
            def: "\nstruct INT".to_owned()
                + &size
                + "_LIT {\nint_fast"
                + &size
                + "_t num;\nINT"
                + &size
                + "_LIT(int_fast"
                + &size
                + "_t i) : num(i){};\n};",
            name: String::from("INT".to_owned() + &size + "_LIT"),
        },
    );
}

pub fn init_float_lit(definitions: &mut IndexMap<parse::Primitives, PrimType>, size: String) {
    if String::from("32") == size {
        definitions.insert(
            parse::Primitives::FLOAT(32),
            PrimType {
                def: "\nstruct FLOAT".to_owned()
                    + &size
                    + "_LIT {\nfloat num;\nFLOAT"
                    + &size
                    + "_LIT(float f) : num(f){};\n};",
                name: String::from("FLOAT".to_owned() + &size + "_LIT"),
            },
        );
    }
}

pub fn init_fn_math(
    definitions: &mut IndexMap<parse::Primitives, PrimType>,
    size: String,
    type_: String,
) {
    let first_letter = type_.as_bytes()[0].to_owned() as char;
    definitions.insert(
        parse::Primitives::INSCOPE(String::from(first_letter) + &size+"_PLUS"),
        PrimType {
            def: format!(
                "{TYPE}{size}_LIT {TYPE}{size}_PLUS({TYPE}{size}_LIT x, {TYPE}{size}_LIT y) {{\nreturn x.num + y.num;}};", size=size, TYPE=type_
                 ),
            name: String::from(type_.clone()+ &size + "_LIT"),
        },
    );
    definitions.insert(
        parse::Primitives::INSCOPE(String::from(first_letter)+ &size+"_SUB"),
        PrimType {
            def: format!(
                "{TYPE}{size}_LIT {TYPE}{size}_SUB({TYPE}{size}_LIT x, {TYPE}{size}_LIT y) {{\nreturn x.num - y.num;}};", size=size, TYPE=type_
            ),
            name: String::from(type_.clone()+ &size + "_LIT"),
        },
    );
    definitions.insert(
        parse::Primitives::INSCOPE(String::from(first_letter)+ &size+"_MUL"),
        PrimType {
            def: format!(
                "{TYPE}{size}_LIT {TYPE}{size}_MUL({TYPE}{size}_LIT x, {TYPE}{size}_LIT y) {{\nreturn x.num * y.num;}};", size=size, TYPE=type_
            ),
            name: String::from(type_.clone()+ &size + "_LIT"),
        },
    );
    definitions.insert(
        parse::Primitives::INSCOPE(String::from(first_letter)+ &size+"_DIV"),
        PrimType {
            def: format!(
                "{TYPE}{size}_LIT {TYPE}{size}_DIV({TYPE}{size}_LIT x, {TYPE}{size}_LIT y) {{\nreturn x.num / y.num;}};", size=size, TYPE=type_
            ),
            name: String::from(type_.clone()+ &size + "_LIT"),
        },
    );
}

fn init_lib(definitions: &mut IndexMap<parse::Primitives, PrimType>, tok_type: parse::Primitives) {
    if parse::prim_eq(&tok_type, &parse::Primitives::INT(32)) {
        let variant = prim_var_str(tok_type);
        init_int_lit(definitions, variant[1].clone());
        init_fn_math(definitions, variant[1].clone(), variant[0].clone());
    } else if parse::prim_eq(&tok_type, &parse::Primitives::FLOAT(32)) {
        let variant = prim_var_str(tok_type);
        init_float_lit(definitions, variant[1].clone());
        init_fn_math(definitions, variant[1].clone(), variant[0].clone());
    }
}

pub fn make_var_def(
    _scope: String,
    definitions: &mut IndexMap<parse::Primitives, PrimType>,
    name: String,
    var_type: parse::Primitives,
    value: DescriptorToken,
) -> String {
    init_lib(definitions, var_type.clone());
    let type_ = definitions[&var_type].name.clone();

    let mut base = vec![format!("std::unique_ptr<{}> {};", type_, name)];
    base.push(gen(value, name, definitions));

    base.join("\n")
}
pub struct ExpSeg {
    pub content: String,
    pub id: String,
}
fn gen_id() -> String {
    let alphabet: [char; 16] = [
        '1', '2', '3', '4', '5', '6', '7', '8', '9', '0', 'a', 'b', 'c', 'd', 'e', 'f',
    ];

    nanoid!(10, &alphabet) //=> "4f90d13a42"
}
pub fn make_exp_seg(
    exp_type: parse::Primitives,
    size: String,
    scope: String,
    seg: parse::BinSeg,
    last_id: String,
    definitions: &mut IndexMap<parse::Primitives, PrimType>,
) -> ExpSeg {
    let mut decl_str: Vec<String> = vec![];
    let id = scope.clone() + "_" + &gen_id();
    if parse::prim_eq(&exp_type, &parse::Primitives::INT(32)) {
        let left = gen(
            DescriptorToken {
                token: seg.left,
                token_real_type: Some(exp_type.clone()),
            },
            scope.clone(),
            definitions,
        );
        if seg.right.is_some() {
            let right = gen(
                DescriptorToken {
                    token: seg.right.unwrap(),
                    token_real_type: Some(exp_type.clone()),
                },
                scope.clone(),
                definitions,
            );
            let line = format!("std::unique_ptr<INT{size}_LIT> {name}(new INT{size}_LIT(INT{size}_{:?}({left}, {right})));",seg.operation,name=id,size=size.clone(),left=left,right=right);
            decl_str.push(line)
        } else {
            let line = format!("std::unique_ptr<INT{size}_LIT> {name}(new INT{size}_LIT(INT{size}_{:?}({left}, *{right})));",seg.operation,name=id,size=size.clone(),left=left,right=last_id);
            decl_str.push(line)
        }
    } else if parse::prim_eq(&exp_type, &parse::Primitives::SIGINT(32)) {
        let left = gen(
            DescriptorToken {
                token: seg.left,
                token_real_type: Some(exp_type.clone()),
            },
            scope.clone(),
            definitions,
        );
        if seg.right.is_some() {
            let right = gen(
                DescriptorToken {
                    token: seg.right.unwrap(),
                    token_real_type: Some(exp_type.clone()),
                },
                scope.clone(),
                definitions,
            );
            let line = format!("std::unique_ptr<SIGINT{size}_LIT> {name}(new SIGINT{size}_LIT(SIGINT{size}_{:?}({left}, {right})));",seg.operation,name=id,size=size.clone(),left=left,right=right);
            decl_str.push(line)
        } else {
            let line = format!("std::unique_ptr<SIGINT{size}_LIT> {name}(new SIGINT{size}_LIT(SIGINT{size}_{:?}({left}, *{right})));",seg.operation,name=id,size=size.clone(),left=left,right=last_id);
            decl_str.push(line)
        }
    } else if parse::prim_eq(&exp_type, &parse::Primitives::FLOAT(32)) {
        let left = gen(
            DescriptorToken {
                token: seg.left,
                token_real_type: Some(exp_type.clone()),
            },
            scope.clone(),
            definitions,
        );
        if seg.right.is_some() {
            let right = gen(
                DescriptorToken {
                    token: seg.right.unwrap(),
                    token_real_type: Some(exp_type),
                },
                scope.clone(),
                definitions,
            );
            let line = format!("std::unique_ptr<FLOAT{size}_LIT> {name}(new FLOAT{size}_LIT(FLOAT{size}_{:?}({left}, {right})));",seg.operation,name=id,size=size.clone(),left=left,right=right);
            decl_str.push(line)
        } else {
            let line = format!("std::unique_ptr<FLOAT{size}_LIT> {name}(new FLOAT{size}_LIT(FLOAT{size}_{:?}({left}, *{right})));",seg.operation,name=id,size=size.clone(),left=left,right=last_id);
            decl_str.push(line)
        }
    }
    return ExpSeg {
        content: decl_str.join(""),
        id,
    };
}
pub fn make_exp(
    scope: String,
    parent: DescriptorToken,
    definitions: &mut IndexMap<parse::Primitives, PrimType>,
) -> String {
    let body = &parent.token;

    let mut exp_type = body.clone().expression.unwrap().exp_type;
    if parent.token_real_type.is_some() {
        exp_type = parent.token_real_type.unwrap();
    }

    let exp_type_size = prim_var_str(exp_type.clone());
    let size = &exp_type_size[1];

    if exp_type_size.len() != 2 {
        println!(
            "Unknown type operation ({line}:{col}): {}",
            scope,
            line = body.location.line,
            col = body.location.start_col
        );
        std::process::exit(1);
    }
    let mut decl_strs: Vec<String> = vec![];
    let mut symbols: Vec<String> = vec![];
    for seg in body.clone().expression.unwrap().body {
        let seg = make_exp_seg(
            exp_type.clone(),
            size.to_string(),
            scope.clone(),
            seg,
            symbols.last().unwrap_or(&"".to_string()).to_string(),
            definitions,
        );
        decl_strs.push(seg.content);
        symbols.push(seg.id);
    }

    if parse::prim_eq(&exp_type, &parse::Primitives::INT(32)) {
        let line = format!(
            "{name}= std::make_unique<INT{size}_LIT>(*{v});",
            name = scope,
            size = size.clone(),
            v = symbols.last().unwrap().to_string(),
        );
        decl_strs.push(line)
    } else if parse::prim_eq(&exp_type, &parse::Primitives::SIGINT(32)) {
        let line = format!(
            "{name}= std::make_unique<SIGINT{size}_LIT>(*{v});",
            name = scope,
            size = size.clone(),
            v = symbols.last().unwrap().to_string(),
        );
        decl_strs.push(line)
    } else if parse::prim_eq(&exp_type, &parse::Primitives::FLOAT(32)) {
        let line = format!(
            "{name}= std::make_unique<FLOAT{size}_LIT>(*{v});",
            name = scope,
            size = size.clone(),
            v = symbols.last().unwrap().to_string(),
        );
        decl_strs.push(line)
    }
    decl_strs.join("\n")
}

pub fn make_number(
    tok: DescriptorToken,
    _definitions: &mut IndexMap<parse::Primitives, PrimType>,
) -> String {
    let mut type_ = prim_var_str(tok.token.number.clone().unwrap().num_type);
    if tok.token_real_type.is_some() {
        type_ = prim_var_str(tok.token_real_type.unwrap());
    }

    if parse::prim_eq(
        &tok.token.number.clone().unwrap().num_type,
        &parse::Primitives::INT(32),
    ) {
        format!(
            "{TYPE}{size}_LIT({v})",
            TYPE = type_[0],
            size = type_[1],
            v = tok.token.number.unwrap().number.unwrap()
        )
    } else {
        format!(
            "{TYPE}{size}_LIT({v})",
            TYPE = type_[0],
            size = type_[1],
            v = tok.token.number.unwrap().float.unwrap()
        )
    }
}

pub fn gen(
    tok: DescriptorToken,
    scope_name: String,
    definitions: &mut IndexMap<parse::Primitives, PrimType>,
) -> String {
    if tok.token.tok_type == parse::ParseType::VARDEF {
        let var = tok.token.variable.clone().unwrap();
        let value = DescriptorToken {
            token_real_type: Some(tok.token.variable.unwrap().value_type),
            token: var.value.clone(),
        };
        make_var_def(scope_name, definitions, var.name, var.value_type, value)
    } else if tok.token.tok_type == parse::ParseType::EXP {
        make_exp(scope_name, tok, definitions)
    } else if tok.token.tok_type == parse::ParseType::NUMBER {
        make_number(tok, definitions)
    } else {
        unimplemented!()
    }
}