/* one downside of Rust... No F-ing default error type that can return in Result */
use std::fmt;

#[derive(Debug)]
pub struct MyFuckingError {
    msg: String,
}

impl MyFuckingError {
    pub fn new(message: &str) -> Self {
        Self{msg: message.to_string()}
    }
}

impl fmt::Display for MyFuckingError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.msg) // user-facing output
    }
}