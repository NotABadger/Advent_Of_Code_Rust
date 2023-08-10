use std::cell::RefCell;
use std::rc::Rc;

pub trait Component: {
    fn add_input(&mut self, input_comp: &Rc<RefCell<dyn Component>>);
    fn validate_component(&self) -> bool;
    fn compute_value(&mut self) -> u16;
}