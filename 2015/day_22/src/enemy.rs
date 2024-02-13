use crate::char_trait::Character;
use crate::effect_trait::Effect;


#[derive(Debug)]
pub struct Enemy{
    current_hp: i32,
    active_effects: Vec<Box<dyn Effect>>,
}

impl Enemy {
    const NAME: &'static str = "Boss";
    const START_HP: i32 = 51;
    const ATTACK_DMG: i32 = 9;
    //create instance
    pub fn new() -> Self
    {
        Enemy{
                current_hp: Self::START_HP,
                active_effects: Vec::new()
            }
    }
}

impl Character for Enemy {
    //reset HP & mana stats of character
    fn reset(&mut self)
    {
        self.current_hp = Self::START_HP;
    }
    
    //print name string
    fn get_name(&self) -> String
    {
        Self::NAME.to_string()
    }

    //character attacks, returns damage done
    fn attack(&mut self, enemy: &mut impl Character)
    {
        _ = enemy.take_damage(Self::ATTACK_DMG);
    }

    //take damage, return remaining hp
    fn take_damage(&mut self, damage: i32) -> i32
    {
        self.current_hp -= damage;
        self.current_hp
    }

    //add effect of attack
    fn add_effect(&mut self, effect: &Box<dyn Effect>) 
    {
        self.active_effects.push(effect.deep_copy_effect());
    }

    //retrieve list with all active effects
    fn get_active_effects(&self) -> &Vec<Box<dyn Effect>>
    {
        &self.active_effects
    }

    //execute all effects, and clean the ones that are expired
    fn execute_effects(&mut self)
    {
        if !self.active_effects.is_empty()
        {
            for effect in &mut self.active_effects
            {
                if effect.get_rounds_active() > 0
                {
                    self.current_hp -= effect.get_dmg();
                    self.current_hp += effect.get_healing();
                    effect.deduct_rounds_active();
                }
            }

            for index in (self.active_effects.len()-1)..=0
            {
                if self.active_effects.get(index).unwrap().get_rounds_active() < 1
                {
                    _ = self.active_effects.remove(index);
                }
            }
        }
    }

    //check if char is still alive
    fn check_alive(&self) -> bool
    {
        let mut ret_val: bool = false;
        if self.current_hp > 0
        {
            ret_val = true;
        }
        ret_val
    }
}

impl Clone for Enemy {
    fn clone(&self) -> Self {
        let mut active_effects_cpy: Vec<Box<dyn Effect>> = Vec::new();
        self.active_effects.iter().for_each(| effect | active_effects_cpy.push(effect.deep_copy_effect()));

        Self { 
            current_hp: self.current_hp, 
            active_effects: active_effects_cpy 
        }
    }
}