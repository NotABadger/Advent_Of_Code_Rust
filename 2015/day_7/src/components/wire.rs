use crate::component_traits::Component;
use std::rc::Rc;
use std::cell::RefCell;

#[derive(Clone)]
pub struct Wire {
    name: String,
    value: Option<u16>,
    input_comp: Option<Rc<RefCell<dyn Component>>>,
}

impl Wire {
    pub fn new(name: &str) -> Self {
        Self { name: name.to_string(), value: None, input_comp: None }
    }

    pub fn set_value(&mut self, value: u16) {
        self.value = Some(value);
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }
}

impl Component for Wire {
    fn add_input(&mut self, input_comp: &std::rc::Rc<std::cell::RefCell<dyn Component>>) {
        self.input_comp = Some(input_comp.clone())
    }

    fn validate_component(&self) -> bool {
        self.input_comp.is_some() || self.value.is_some()
    }

    fn compute_value(&mut self) -> u16 {
        if let Some(return_val) = self.value {
            return return_val;
        }
        else {
            let rc_obj = self.input_comp.as_mut().expect("Call validate before computing!");
            return rc_obj.borrow_mut().compute_value();
        }
    }
}