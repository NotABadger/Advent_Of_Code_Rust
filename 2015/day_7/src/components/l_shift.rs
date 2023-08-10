use std::cell::RefCell;
use std::rc::Rc;

use crate::component_traits::Component;

#[derive(Clone)]
pub struct LShift
{
    input : Option<Rc<RefCell<dyn Component>>>,
    operation : u32,
    value : Option<u16>,
}

impl LShift
{
    pub fn new() -> Self
    {
        Self{input: None, operation: 0, value: None}
    }

    pub fn set_offset(&mut self, offset: u32)
    {
        self.operation = offset;
    }
}

impl Component for LShift
{
    fn add_input(&mut self, input_comp: &Rc<RefCell<dyn Component>>)
    {
        self.input = Some(input_comp.clone());
    }

    fn validate_component(&self) -> bool
    {
        if self.value.is_some()
        {
            return true;
        }
        false
    }

    fn compute_value(&mut self) -> u16
    {
        if self.input.is_none()
        {
            panic!("Left_shift with no inputs");
        }

        if self.value.is_none()
        {
            let mut ret_val = self.input.as_ref().unwrap().borrow_mut().compute_value();
            ret_val = ret_val << self.operation;
            self.value = Some(ret_val);
        }
        self.value.unwrap()
    }
}

