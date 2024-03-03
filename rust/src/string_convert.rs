use crate::types;

pub fn step(prev: String, string_convert: &types::StringConvert) -> String {
    return prev
        .chars()
        .map(|value| match value {
            'm' => String::from(string_convert.m),
            'l' => String::from(string_convert.l),
            'f' => String::from(string_convert.f),
            _ => String::from(value),
        })
        .collect();
}

pub fn recursion(axiom: &String, string_convert: &types::StringConvert, no_steps: u8) -> String {
    let mut n = 0;
    let mut result = String::from(axiom);

    while n < no_steps {
        result = step(result, string_convert);

        n += 1;
    }

    return result;
}
