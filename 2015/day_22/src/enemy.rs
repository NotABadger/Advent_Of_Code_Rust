use crate::char_trait::Character;
use crate::effect_trait::Effect;


#[derive(Debug)]
pub struct Enemy{
    name: String,
    max_hp: i32,
    current_hp: i32,
    attack_damage: i32,
    armor: i32,
    active_effects: Vec<Box<dyn Effect>>,
}

impl Enemy {
      //create instance
      pub fn new() -> Self
      {
          Enemy{ name: "Boss".to_string(), 
                    max_hp: 51 , 
                    current_hp: 51, 
                    attack_damage: 9,  
                    armor: 0,  
                    active_effects: Vec::new()
                }
      }
}

impl Character for Enemy {
    //reset HP & mana stats of character
    fn reset(&mut self)
    {
        self.current_hp = self.max_hp;
    }
    
    //print name string
    fn get_name(&self) -> String
    {
        self.name.clone()
    }

    //character attacks, returns damage done
    fn attack(&mut self, enemy: &mut impl Character)
    {
        _ = enemy.take_damage(self.attack_damage);
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
        self.armor = 0;
        if self.active_effects.len() > 1
        {
            for effect in &mut self.active_effects
            {
                self.armor = self.armor + effect.get_armor();
                self.current_hp -= effect.get_dmg();
                self.current_hp += effect.get_healing();
                effect.deduct_rounds_active();
            }

            for index in self.active_effects.len()-1 ..=0
            {
                if self.active_effects[index].get_rounds_active() < 1
                {
                    self.active_effects.remove(index);
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
        for active_effect in self.get_active_effects()
        {
            active_effects_cpy.push(active_effect.deep_copy_effect());
        }

        Self { name: self.get_name(), 
            max_hp: self.max_hp, 
            current_hp: self.current_hp, 
            attack_damage: self.attack_damage, 
            armor: self.armor, 
            active_effects: active_effects_cpy 
        }
    }
}