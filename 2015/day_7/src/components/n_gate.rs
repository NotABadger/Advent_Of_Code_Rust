use std::cell::RefCell;
use std::rc::Rc;
use crate::component_traits::Component;

#[derive(Clone)]
pub struct NGate
{
    input : Option<Rc<RefCell<dyn Component>>>,
    value : Option<u16>,
}

impl NGate
{
    pub fn new() -> Self
    {
        Self{input: None, value: None}
    }
}

impl Component for NGate
{
    fn add_input(&mut self, input_comp: &Rc<RefCell<dyn Component>>)
    {
        self.input = Some(input_comp.clone());
    }
    
    fn validate_component(&self) -> bool
    {
        !self.input.is_none()
    }

    fn compute_value(&mut self) -> u16
    {
        if self.input.is_none()
        {
            panic!("not gate with no input!");
        }

        if self.value.is_none()
        {
            let return_val: u16 = self.input.as_ref().unwrap().borrow_mut().compute_value();
            self.value = Some(!return_val);
        }
        self.value.unwrap()
    }
}

