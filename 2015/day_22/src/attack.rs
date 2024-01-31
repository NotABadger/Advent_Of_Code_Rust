use crate::effect_trait::*;

#[derive(Clone, Copy)]
pub struct Attack {
    damage: i32,
    rounds_active: i32,
}

impl Attack {
    //create instance
    pub fn new(damage: i32) -> Self
    {
        Attack{damage, rounds_active: 1}
    }
}

impl Effect for Attack {

    //get damage an effect does
    fn get_dmg(&self) -> i32
    {
        self.damage
    }
    //get armor an effect gives
    fn get_armor(&self) -> i32
    {
        0
    }
    //get healing an effect does
    fn get_healing(&self) -> i32
    {
        0
    }
    //get mana an effect gives
    fn get_mana(&self) -> i32
    {
        0
    }
    //target to whom the effect is applied
    fn get_target(&self) -> Target
    {
        Target::ENEMY
    }
    //Check how many rounds this effect is active
    fn get_rounds_active(&self) -> i32
    {
    self.rounds_active
    }
    //deduct 1 round of this effect being active
    fn deduct_rounds_active(&mut self)
    {
    self.rounds_active = self.rounds_active - 1;
    }
}