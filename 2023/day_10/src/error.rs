/* one downside of Rust... No F-ing default error type that can return in Result */
use std::fmt;

#[derive(Debug)]
pub struct MyError {
    msg: String,
}

impl MyError {
    pub fn new(message: &str) -> Self {
        Self{msg: message.to_string()}
    }
}

impl fmt::Display for MyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.msg) // user-facing output
    }
}