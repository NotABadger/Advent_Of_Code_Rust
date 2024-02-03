use std::fmt::Debug;
use crate::effect_trait::Effect;

pub trait Character: Debug {
    //reset HP & mana stats of character
    fn reset(&mut self);

    //print HP, mana & effects
    fn print_stats(&self)
    {
        println!("{:?}", self);
    }

    //print name string
    fn get_name(&self) -> String;

    //character attacks, returns damage done
    fn attack(&mut self, enemy: &mut impl Character);

    //take damage, return remaining hp
    fn take_damage(&mut self, damage: i32) -> i32;

    //add effect of attack
    fn add_effect(&mut self, effect: Box<dyn Effect>);

    //retrieve list with all active effects
    fn get_active_effects(&self) -> &Vec<Box<dyn Effect>>;

    //execute all effects
    fn execute_effects(&mut self);

    //check if char is still alive
    fn check_alive(&self) -> bool;
    
}