use std::cell::Cell;

#[derive(Debug, Clone)]
pub struct City
{
    visited: Cell<u32>,
    province: Cell<Option<u32>>,
}

impl City
{
    pub fn new() -> Self
    {
        Self{visited: Cell::new(0), province: Cell::new(None)}
    }

    pub fn get_visited(&self) -> bool
    {
        if self.visited.get() > 0
        {
            return true;
        }
        false
    }

    pub fn visit_city(&self)
    {
        let mut count = self.visited.get();
        count += 1;
        self.visited.set(count);
    }

    pub fn get_visit_count(&self) -> u32
    {
        self.visited.get()
    }

    pub fn reset_visit(&self)
    {
        self.visited.set(0);
    }

    pub fn set_province(&self, province: u32)
    {
        self.province.set(Some(province));
    }

    pub fn clear_province(&self)
    {
        self.province.set(None);
    }

    pub fn get_province(&self) -> Option<u32>
    {
        self.province.get()
    }
}