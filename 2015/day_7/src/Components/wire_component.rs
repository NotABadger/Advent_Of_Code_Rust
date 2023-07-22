use crate::component_traits::Component;

pub struct Wire<'a> 
{
    input : Box<Option<&'a dyn Component>>,
    outputs : Vec<&'a dyn Component>,
    value : Option<u16>,
}

impl Component for Wire<'a>
{
    //Create new object
    fn new(name: &str) -> Self
    {
        return Wire{input: Box::new(None), outputs: vec![] , value: None};
    }
    //add other object as input
    fn add_input(&self, component: &impl Component)
    {
        self.inputs = Box::new(component);
    }

    fn add_output(&self, component: &impl Component)
    {
        self.output.push(component);
    }
    
    fn check_value(&self) -> u16
    {
        if self.value.is_none()
        {
            self.value = Some(self.input.check_value());
        }
        return self.value;
    }
    //only for setting value before resolving
    fn set_value(&self, value: u16) {
        self.value = Some(value);
    }
    
}