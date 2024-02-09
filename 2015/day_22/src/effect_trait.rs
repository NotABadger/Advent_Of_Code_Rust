use std::fmt::Debug;

pub trait Effect: Debug {
    //get name of spell
    fn get_name(&self) -> String;
    //get mana cost of the effect
    fn get_cost(&self) -> i32 {
        0
    }
    //get damage an effect does
    fn get_dmg(&self) -> i32 {
        0
    }
    //get armor an effect gives
    fn get_armor(&self) -> i32 {
        0
    }
    //get healing an effect does
    fn get_healing(&self) -> i32 {
        0
    }
    //get mana an effect gives
    fn get_mana(&self) -> i32 {
        0
    }
    //Check how many rounds this effect is active
    fn get_rounds_active(&self) -> i32 {
        0
    }
    //deduct 1 round of this effect being active
    fn deduct_rounds_active(&mut self);
    //Copy all values
    fn deep_copy_effect(&self) -> Box<dyn Effect>;
}
