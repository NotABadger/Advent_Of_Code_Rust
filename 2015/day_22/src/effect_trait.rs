use std::fmt::Debug;

pub trait Effect: Debug {
    //get damage an effect does
    fn get_dmg(&self) -> i32;
    //get armor an effect gives
    fn get_armor(&self) -> i32;
    //get healing an effect does
    fn get_healing(&self) -> i32;
    //get mana an effect gives
    fn get_mana(&self) -> i32;
    //target to whom the effect is applied
    fn get_target(&self) -> Target;
    //Check how many rounds this effect is active
    fn get_rounds_active(&self) -> i32;
    //deduct 1 round of this effect being active
    fn deduct_rounds_active(&mut self);
}

pub enum Target {
    CASTER,
    ENEMY,
}