use crate::wire::Wire;
pub trait Component<'a> {
    fn add_input(&mut self, wire: &str);
    fn get_input(&self) -> Vec<&'a str>;
    fn add_output(&mut self,  wire: &str);
    fn validate_component(&self) -> bool;
    fn compute_value(&mut self, wire_list: &mut Vec<Wire>);
}