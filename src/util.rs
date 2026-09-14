pub struct Util;

impl Util {
    pub fn to_snake_case(name: &str) -> String {
        let mut result = String::new();
        let chars: Vec<char> = name.chars().collect();

        for (i, &c) in chars.iter().enumerate() {
            if c.is_uppercase() {
                // insert underscore only if previous char exists and is lowercase
                if i > 0 && chars[i - 1].is_lowercase() {
                    result.push('_');
                }
                result.push(c.to_ascii_lowercase());
            } else {
                result.push(c);
            }
        }

        result
    }
}