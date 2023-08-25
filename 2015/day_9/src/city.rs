use std::cell::Cell;

use crate::province::Province;

#[derive(Debug, Clone)]
pub struct City<'a>
{
    visited: Cell<bool>,
    province: Option<&'a Province>,
}

impl City<'_>
{
    pub fn new() -> Self
    {
        Self{visited: Cell::new(false), province: None}
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

    pub fn set_province(&mut self, province: &Province)
    {
        self.province = Some(province);
    }

    pub fn clear_province(&mut self)
    {
        self.province = None;
    }

    pub fn get_provinde(&self) -> Option<u32>
    {
        match self.province
        {
            Some(prov) => return Some(prov.get_id()),
            None => return None,
        }
    }
}