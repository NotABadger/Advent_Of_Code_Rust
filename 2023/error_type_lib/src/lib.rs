/* one downside of Rust... No default error type that can return in Result */
use std::fmt;
pub use std::error::Error;

#[derive(Debug)]
pub struct MyError {
    msg: String,
    original_err: Option<Box<dyn Error>>,
}

impl MyError {
    pub fn new(message: &str, original_error: Option<Box<dyn Error>>) -> Self {
        Self{msg: message.to_string(), original_err: original_error}
    }
    pub fn new_as_box(message: &str, original_error: Option<Box<dyn Error>>) -> Box<Self> {
        Box::new(Self{msg: message.to_string(), original_err: original_error})
    }
}

impl Error for MyError {}

impl fmt::Display for MyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut ret_val = writeln!(f, "{}", self.msg); // user-facing output
        if let Some(original_error) = &self.original_err {
            ret_val = writeln!(f, "{}", original_error);
        }
        ret_val
    }
}