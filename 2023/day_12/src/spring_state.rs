pub use error_type_lib::MyError;
use std::error::Error;

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum SpringState {
    Operational,
    Broken,
    Unknown,
}

impl TryFrom<char> for SpringState {
    type Error = Box<dyn Error>;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '.' => return Ok(Self::Operational),
            '#' => return Ok(Self::Broken),
            '?' => return Ok(Self::Unknown),
            _ => return Err(MyError::new_as_box("Tried to convert invalid char to SpringState", None))
        }
    }
}