
#[derive(Debug)]
pub struct Replacement{
    pub find: String,
    pub replace: String,
}

impl Replacement{
    pub fn new(find: &str, replace: &str) -> Self
    {
        Self{find: find.to_string(), replace: replace.to_string()}
    }
}