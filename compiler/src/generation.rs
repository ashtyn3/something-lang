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
    pub raw: Option<parse::ParseTok>,
    pub ext: bool,
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
#include <string>
struct STR_LIT {
  int length;
  std::string chs;
  std::string display() { return chs; };
  STR_LIT(std::string str) : chs(str){};
};
            "
            .to_string(),
            name: String::from("STR_LIT"),
            raw: None,
            ext: true,
        },
    );
}

pub fn init_int_lit(definitions: &mut IndexMap<parse::Primitives, PrimType>, size: String) {
    definitions.insert(
        parse::Primitives::INT(size.parse::<i8>().unwrap()),
        PrimType {
            def: "#include<string>\nstruct INT".to_owned()
                + &size
                + "_LIT {\nint_fast"
                + &size
                + "_t num;\nstd::string display() { return std::to_string(num); };\nINT"
                + &size
                + "_LIT(int_fast"
                + &size
                + "_t i) : num(i){};\n};",
            name: String::from("INT".to_owned() + &size + "_LIT"),
            raw: None,
            ext: true,
        },
    );
}

pub fn init_float_lit(definitions: &mut IndexMap<parse::Primitives, PrimType>, size: String) {
    if String::from("32") == size {
        definitions.insert(
            parse::Primitives::FLOAT(32),
            PrimType {
                def: "\n#include <string>\nstruct FLOAT".to_owned()
                    + &size
                    + "_LIT {\nfloat num;\nstd::string display() { return std::to_string(num); };\nFLOAT"
                    + &size
                    + "_LIT(float f) : num(f){};\n};",
                name: String::from("FLOAT".to_owned() + &size + "_LIT"),
            raw: None,
            ext: true,
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
            raw: None,
            ext: true,
        },
    );
    definitions.insert(
        parse::Primitives::INSCOPE(String::from(first_letter)+ &size+"_SUB"),
        PrimType {
            def: format!(
                "{TYPE}{size}_LIT {TYPE}{size}_SUB({TYPE}{size}_LIT x, {TYPE}{size}_LIT y) {{\nreturn x.num - y.num;}};", size=size, TYPE=type_
            ),
            name: String::from(type_.clone()+ &size + "_LIT"),
            raw: None,
            ext: true,
        },
    );
    definitions.insert(
        parse::Primitives::INSCOPE(String::from(first_letter)+ &size+"_MUL"),
        PrimType {
            def: format!(
                "{TYPE}{size}_LIT {TYPE}{size}_MUL({TYPE}{size}_LIT x, {TYPE}{size}_LIT y) {{\nreturn x.num * y.num;}};", size=size, TYPE=type_
            ),
            name: String::from(type_.clone()+ &size + "_LIT"),
            raw: None,
            ext: true,
        },
    );
    definitions.insert(
        parse::Primitives::INSCOPE(String::from(first_letter)+ &size+"_DIV"),
        PrimType {
            def: format!(
                "{TYPE}{size}_LIT {TYPE}{size}_DIV({TYPE}{size}_LIT x, {TYPE}{size}_LIT y) {{\nreturn x.num / y.num;}};", size=size, TYPE=type_
            ),
            name: String::from(type_.clone()+ &size + "_LIT"),
            raw: None,
            ext: true,
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
    } else if parse::prim_eq(&tok_type, &parse::Primitives::STRING) {
        init_str_lit(definitions);
    }
}

pub fn init_fn_io(definitions: &mut IndexMap<parse::Primitives, PrimType>) {
    definitions.insert(
        parse::Primitives::INSCOPE("print".to_string()),
        PrimType {
            def: format!(
                "
#include <cstdio>
int print(std::vector<std::unique_ptr<STR_LIT>>* ARGS) {{
  for (int i = 0; i < ARGS->size(); i++) {{
    STR_LIT t = *ARGS->at(i);
    printf(\"%s\", t.chs.c_str());
  }}
  return 0;
}}
                "
            )
            .to_string(),
            name: "print".to_string(),
            raw: None,
            ext: true,
        },
    );
}

pub fn make_var_def(
    _scope: String,
    definitions: &mut IndexMap<parse::Primitives, PrimType>,
    name: String,
    var_type: parse::Primitives,
    value: DescriptorToken,
) -> String {
    init_lib(definitions, var_type.clone());
    let type_: String;
    if definitions.get(&var_type).is_none() {
        type_ = "int".to_string();
        println!(
            "Warning: Unknown return type for variable {name}.",
            name = &name
        )
    } else {
        type_ = definitions[&var_type].name.clone();
    }

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

    nanoid!(5, &alphabet[10..]).to_string() + &nanoid!(10, &alphabet).to_string()
    //=> "4f90d13a42"
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
            "_".to_string(),
            definitions,
        );
        if seg.right.is_some() {
            let right = gen(
                DescriptorToken {
                    token: seg.right.unwrap(),
                    token_real_type: Some(exp_type.clone()),
                },
                "_".to_string(),
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
            "_".to_string(),
            definitions,
        );
        if seg.right.is_some() {
            let right = gen(
                DescriptorToken {
                    token: seg.right.unwrap(),
                    token_real_type: Some(exp_type.clone()),
                },
                String::from("_"),
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
            String::from("_"),
            definitions,
        );
        if seg.right.is_some() {
            let right = gen(
                DescriptorToken {
                    token: seg.right.unwrap(),
                    token_real_type: Some(exp_type),
                },
                String::from("_"),
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
            scope.clone().replace(".", "_"),
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
    scope: String,
    _definitions: &mut IndexMap<parse::Primitives, PrimType>,
) -> String {
    let mut type_ = prim_var_str(tok.token.number.clone().unwrap().num_type);
    if tok.token_real_type.is_some() {
        type_ = prim_var_str(tok.token_real_type.unwrap());
    }

    if scope == "_" {
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
    } else {
        if parse::prim_eq(
            &tok.token.number.clone().unwrap().num_type,
            &parse::Primitives::INT(32),
        ) {
            format!(
                "{scope} = std::make_unique<{TYPE}{size}_LIT>({TYPE}{size}_LIT({v}));",
                TYPE = type_[0],
                size = type_[1],
                scope = scope,
                v = tok.token.number.unwrap().number.unwrap()
            )
        } else {
            format!(
                "{scope} = std::make_unique<{TYPE}{size}_LIT>({TYPE}{size}_LIT({v}));",
                TYPE = type_[0],
                size = type_[1],
                scope = scope,
                v = tok.token.number.unwrap().float.unwrap()
            )
        }
    }
}

pub fn make_string(
    tok: DescriptorToken,
    scope_name: String,
    definitions: &mut IndexMap<parse::Primitives, PrimType>,
) -> String {
    init_str_lit(definitions);
    if scope_name == "_" {
        format!(
            "STR_LIT(\"{v}\")",
            v = tok.token.string.unwrap().clone().content
        )
    } else {
        format!(
            "{name} = std::make_unique<STR_LIT>(STR_LIT(\"{v}\"));",
            v = tok.token.string.unwrap().clone().content,
            name = scope_name
        )
    }
}

fn make_std_fncall(
    tok: DescriptorToken,
    parent_scope: Option<String>,
    definitions: &mut IndexMap<parse::Primitives, PrimType>,
) -> String {
    init_str_lit(definitions);
    let mut arg_decls: Vec<String> = vec![];
    let mut arg_types: Vec<String> = vec![];
    let scope: String = gen_id();

    // std::vector<std::unique_ptr<STR_LIT>>
    for arg in tok.token.fncall.clone().unwrap().args {
        if tok.token.fncall.clone().unwrap().name == "print" {
            if arg.tok_type == parse::ParseType::STRING {
                let lit = make_string(
                    DescriptorToken {
                        token_real_type: None,
                        token: arg,
                    },
                    String::from("_"),
                    definitions,
                );
                arg_types.push(
                    definitions
                        .get(&parse::Primitives::STRING)
                        .unwrap()
                        .name
                        .clone(),
                );
                let arg_lit = format!(
                    "std::unique_ptr<STR_LIT> {name}(new {lit});\n{scope}.push_back(std::move({name}));",
                    name = scope.clone() + "_" + &gen_id(),
                    lit = lit,
                    scope = scope
                );
                arg_decls.push(arg_lit);
            } else if arg.tok_type == parse::ParseType::NUMBER {
                let lit = make_number(
                    DescriptorToken {
                        token_real_type: None,
                        token: arg,
                    },
                    "_".to_string(),
                    definitions,
                );

                let arg_lit = format!(
                    "std::unique_ptr<STR_LIT> {name}(new STR_LIT({lit}.display()));\n{scope}.push_back(std::move({name}));",
                    name = scope.clone() + "_" + &gen_id(),
                    lit = lit,
                    scope = scope
                );
                arg_decls.push(arg_lit);
            } else if arg.tok_type == parse::ParseType::LABEL {
                let lit = make_ident(
                    DescriptorToken {
                        token_real_type: None,
                        token: arg,
                    },
                    parent_scope.clone(),
                    definitions,
                );
                let arg_lit = format!(
                    "std::unique_ptr<STR_LIT> {name}(new STR_LIT({lit}.display()));\n{scope}.push_back(std::move({name}));",
                    name = scope.clone() + "_" + &gen_id(),
                    lit = lit,
                    scope = scope
                );
                arg_decls.push(arg_lit);
            } else if arg.tok_type == parse::ParseType::EXP {
                let sub_var = scope.clone() + "_" + &gen_id();
                let type_ = arg.expression.clone().unwrap().exp_type;
                init_lib(definitions, type_.clone());

                let str_type = definitions.get(&type_).unwrap().name.clone();
                let mut base = vec![format!("std::unique_ptr<{}> {};", str_type, sub_var)];

                let lit = make_exp(
                    sub_var.clone(),
                    DescriptorToken {
                        token_real_type: None,
                        token: arg,
                    },
                    definitions,
                );
                base.push(lit);

                let arg_lit = format!(
                    "std::unique_ptr<STR_LIT> {name}(new STR_LIT({v}.display()));\n{scope}.push_back(std::move({name}));",
                    name=sub_var.clone()+"_"+&gen_id(),
                    scope = scope,
                    v = format!("(*{name})", name = &sub_var)
                );
                arg_decls.append(&mut base);
                arg_decls.push(arg_lit);
            } else {
                unimplemented!();
            }
        }
    }

    if tok.token.fncall.clone().unwrap().name == "print" {
        init_fn_io(definitions);
        let arg_lit = format!(
            "std::vector<std::unique_ptr<STR_LIT>> {name};",
            name = scope.clone(),
        );
        arg_decls.insert(0, arg_lit);
    }
    let print_call = format!(
        "{fnName}(&{name});",
        name = scope.clone(),
        fnName = tok.token.fncall.unwrap().name
    );
    arg_decls.push(print_call);
    //definitions.get(&parse::Primitives::INSCOPE("PRINT".to_string()));
    arg_decls.join("\n")
}

fn make_ident(
    tok: DescriptorToken,
    scope: Option<String>,
    definitions: &mut IndexMap<parse::Primitives, PrimType>,
) -> String {
    let lit = tok.token.ident.unwrap();
    init_lib(definitions, lit.var_type);
    if scope.clone().unwrap() == "_" {
        format!("(*{name})", name = lit.name)
    } else {
        format!(
            "{scope} = std::move({name});",
            name = lit.name,
            scope = scope.unwrap()
        )
    }
}

fn make_func(
    tok: DescriptorToken,
    definitions: &mut IndexMap<parse::Primitives, PrimType>,
) -> String {
    if definitions
        .get(&tok.token.clone().fnmake.unwrap().return_type)
        .is_none()
    {
        init_lib(definitions, tok.token.clone().fnmake.unwrap().return_type)
    }
    let mut param_decls = vec![];
    for param in tok.token.clone().fnmake.unwrap().params {
        if definitions.get(&param.value_type).is_none() {
            init_lib(definitions, param.clone().value_type)
        }
        let type_name = &definitions.get(&param.value_type).unwrap().clone().name;
        let param_prop = format!(
            "std::unique_ptr<{TYPE}> {name};\n",
            TYPE = type_name,
            name = param.name
        );
        param_decls.push(param_prop);
    }
    let mut body: Vec<String> = vec![];
    let local_scope = &mut *definitions;
    local_scope.insert(
        parse::Primitives::INSCOPE(tok.token.fnmake.clone().unwrap().name),
        PrimType {
            def: String::from(""),
            name: tok.token.fnmake.clone().unwrap().name,
            raw: Some(tok.token.clone()),
            ext: false,
        },
    );
    for line in tok.token.clone().fnmake.unwrap().body {
        let statement = gen(
            DescriptorToken {
                token: line,
                token_real_type: None,
            },
            "_".to_string(),
            local_scope,
        );
        body.push(statement);
    }
    let func = format!(
        "
    struct {name} {{
        std::unique_ptr<{ret_type}> RETURN;
        {params}
        void body() {{
            {body}
        }}
        int call() {{
            body();
            return 0;
        }}
    }};
        ",
        name = tok.token.fnmake.clone().unwrap().name,
        ret_type = definitions
            .get(&tok.token.fnmake.clone().unwrap().return_type)
            .unwrap()
            .name,
        params = param_decls.join(""),
        body = body.join("\n"),
    );
    definitions.insert(
        parse::Primitives::INSCOPE(tok.token.fnmake.clone().unwrap().name),
        PrimType {
            def: func,
            name: tok.token.fnmake.clone().unwrap().name,
            raw: Some(tok.token),
            ext: false,
        },
    );
    "".to_string()
}
pub fn make_fncall(
    tok: DescriptorToken,
    scope_name: Option<String>,
    definitions: &mut IndexMap<parse::Primitives, PrimType>,
) -> String {
    let id = gen_id();
    let name = tok.token.fncall.clone().unwrap().name;
    let mut params = vec![];
    let def = &mut definitions
        .get(&parse::Primitives::INSCOPE(name.clone().to_string()))
        .unwrap()
        .raw
        .clone();
    let arg_list = &def.clone().unwrap().fnmake.unwrap().params;
    for (i, sup_arg) in tok.token.fncall.unwrap().args.iter().enumerate() {
        let arg = &arg_list[i];
        let name = &arg.name;
        let mut sc = scope_name.to_owned();

        if sc.clone().unwrap() == "_" {
            sc = Some(format!("{id}.{name}", id = id, name = name));
        }
        let val = gen(
            DescriptorToken {
                token_real_type: None,
                token: sup_arg.clone(),
            },
            sc.clone().unwrap(),
            definitions,
        );
        let type_str = definitions.get(&arg.value_type).unwrap().name.clone();
        init_lib(definitions, arg.clone().value_type);
        if scope_name.clone().unwrap() != "_" {
            params.push(format!(
                "{id}.{name} = std::make_unique<{TYPE}>({val});",
                id = id,
                TYPE = type_str,
                val = val,
                name = name
            ))
        } else {
            params.push(val)
        }
    }
    let mut decls: Vec<String> = vec![];
    let base = format!(
        "{name} {id};
        {args}\n{id}.call();",
        name = name,
        id = id,
        args = params.join("\n")
    );
    decls.push(base.clone());
    if scope_name.is_some() && scope_name.clone().unwrap() != "_" {
        decls.push(format!(
            "{name} = *{id}->RETURN",
            name = scope_name.unwrap(),
            id = id
        ));
    }
    decls.join("\n")
}

fn make_return(
    tok: DescriptorToken,
    definitions: &mut IndexMap<parse::Primitives, PrimType>,
) -> String {
    let gen_val = gen(
        DescriptorToken {
            token_real_type: None,
            token: tok.token.fnreturn.unwrap().value.unwrap(),
        },
        String::from("RETURN"),
        definitions,
    );

    gen_val
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
            token: var.value.unwrap().clone(),
        };
        make_var_def(scope_name, definitions, var.name, var.value_type, value)
    } else if tok.token.tok_type == parse::ParseType::EXP {
        make_exp(scope_name, tok, definitions)
    } else if tok.token.tok_type == parse::ParseType::NUMBER {
        make_number(tok, scope_name, definitions)
    } else if tok.token.tok_type == parse::ParseType::FNCALL
        && tok.token.fncall.clone().unwrap().is_std == true
    {
        make_std_fncall(tok.clone(), Some(scope_name), definitions)
    } else if tok.token.tok_type == parse::ParseType::FNCALL {
        make_fncall(tok.clone(), Some(scope_name), definitions)
    } else if tok.token.tok_type == parse::ParseType::LABEL {
        make_ident(tok.clone(), Some(scope_name), definitions)
    } else if tok.token.tok_type == parse::ParseType::STRING {
        make_string(tok.clone(), scope_name, definitions)
    } else if tok.token.tok_type == parse::ParseType::FNMAKE {
        make_func(tok, definitions)
    } else if tok.token.tok_type == parse::ParseType::FNRETURN {
        make_return(tok, definitions)
    } else {
        println!("{:#?}", tok.token);
        unimplemented!()
    }
}
