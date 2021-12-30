use crate::parse;

//TODO: Add string templating
// pub fn str_template() {}

pub fn print_fn(fn_call: &mut parse::ParseTok) {
    let mut count = 0;
    for arg in fn_call.clone().fncall.unwrap().args {
        if count == 0 {
            if arg.tok_type != parse::ParseType::STRING {
                println!(
                    "Expected type ({line}:{col}): Expected type of STRING instead got {:?}",
                    arg.tok_type,
                    line = fn_call.location.line,
                    col = fn_call.location.start_col
                );
                std::process::exit(1);
            }
        }
        count += 1;
    }
}
pub fn is_std_fn(fn_call: &mut parse::ParseTok) -> bool {
    match fn_call.clone().fncall.unwrap().name.as_str() {
        "print" => {
            print_fn(fn_call);
            true
        }
        _ => false,
    }
}
