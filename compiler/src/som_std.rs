use crate::parse;

//TODO: Add string templating
// pub fn str_template() {}

pub fn is_std_fn(fn_call: &mut parse::ParseTok) -> bool {
    match fn_call.clone().fncall.unwrap().name.as_str() {
        "print" => true,
        _ => false,
    }
}
