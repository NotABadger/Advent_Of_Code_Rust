use crate::char_trait::Character;
use crate::effect_trait::Effect;
use crate::attack::Attack;

#[derive(Debug)]
pub struct Enemy{
    name: String,
    max_hp: i32,
    current_hp: i32,
    attack: Attack,
    armor: i32,
    effects: Vec<Box<dyn Effect>>,
}

impl Enemy {
      //create instance
      pub fn new() -> Self
      {
          Enemy{name: "Boss".to_string(), max_hp: 51 , current_hp: 51, attack: Attack::new(9),  armor: 0,  effects: Vec::new() }
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
    fn attack(&mut self) -> Option<Box<dyn Effect>>
    {
        Some(Box::new(self.attack.clone()))
    }

    //take damage, return remaining hp
    fn take_damage(&mut self, damage: i32) -> i32
    {
        self.current_hp -= damage;
        self.current_hp
    }

    //add effect of attack
    fn add_effect(&mut self, effect: Box<dyn Effect>) 
    {
        self.effects.push(effect);
    }

    //execute all effects, and clean the ones that are expired
    fn execute_effects(&mut self)
    {
        self.armor = 0;
        for effect in &mut self.effects
        {
            self.armor = self.armor + effect.get_armor();
            self.current_hp -= effect.get_dmg();
            self.current_hp += effect.get_healing();
            effect.deduct_rounds_active();
        }

        for index in self.effects.len()-1 ..=0
        {
            if self.effects[index].get_rounds_active() < 1
            {
                self.effects.remove(index);
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

