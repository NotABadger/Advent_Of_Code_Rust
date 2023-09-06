use std::cell::Cell;

#[derive(Debug, Clone)]
pub struct Person
{
    name: String,
    seated: Cell<bool>,
}

impl Person
{
    pub fn new(name: &str) -> Self
    {
        Self{name: name.to_string(), seated: Cell::new(false)}
    }
    pub fn get_seated(&self) -> bool
    {
        return self.seated.get()
    }

    pub fn seat_person(&self)
    {
        self.seated.set(true);
    }

    pub fn reset_seating(&self)
    {
        self.seated.set(false);
    }

}