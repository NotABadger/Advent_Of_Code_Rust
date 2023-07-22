pub trait Component {
    //Create new object
    fn new(name: &str) -> Self;
    //add other object as input
    fn add_input(&self, component: &impl Component);
    fn add_output(&self, component: &impl Component);
    //only for setting value before resolving
    fn set_value(&self, value: u16);
    fn check_value(&self) -> u16;
}