use crate::effect_trait::Effect;
pub trait Character {
    //reset HP & mana stats of character
    fn reset(&mut self);

    //print HP, mana & effects
    fn print_stats(&self);

    //character attacks, returns damage done
    fn attack(&mut self) -> Option<Box<dyn Effect>>;

    //take damage, return remaining hp
    fn take_damage(&mut self, damage: i32) -> i32;

    //execute all effects
    fn execute_effects(&mut self);

    //check if char is still alive
    fn check_alive(&self) -> bool;

}