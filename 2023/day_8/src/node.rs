use std::convert::From;

#[derive(Debug, Clone)]
pub struct Node {
    pub name: String,
    pub left_node: String,
    pub right_node: String
}


impl From<&str> for Node {
    fn from(value: &str) -> Self {
        let mut ret_val: Self = Self { name: "".to_string(), left_node: "".to_string(), right_node: "".to_string() };
        let mut words_itt = value.trim().split_whitespace();
        
        if let Some(self_name_option) = words_itt.next() {
            ret_val.name = self_name_option.to_string();
        }

        _ = words_itt.next(); //should be '=' sign
        
        if let Some(self_left_option) =  words_itt.next() {
            ret_val.left_node = self_left_option.trim_matches(| char: char| char.eq(&'(') || char.eq(&',')).to_string();
        }
        if let Some(self_right_option) =  words_itt.next() {
            ret_val.right_node = self_right_option.trim_matches(')').to_string();
        }

        ret_val
    }
}
