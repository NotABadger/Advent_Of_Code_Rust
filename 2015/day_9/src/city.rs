use std::cell::Cell;

#[derive(Debug, Clone)]
pub struct City
{
    name: String,
    visited: Cell<bool>,
}

impl City
{
    pub fn new(name : &str) -> Self
    {
        Self{name: name.to_string(), visited: Cell::new(false)}
    }

    pub fn get_name(&self) -> String
    {
        self.name.to_string()
    }

    pub fn get_visited(&self) -> bool
    {
        self.visited.get()
    }

    pub fn visit_city(&self)
    {
        self.visited.set(true);
    }

    pub fn reset_visit(&self)
    {
        self.visited.set(false);
    }
}