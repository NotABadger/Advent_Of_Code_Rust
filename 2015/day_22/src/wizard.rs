use crate::char_trait::Character;
use crate::effect_trait::Effect;

use crate::spells::*;
use drain::Drain;
use magic_missile::MagicMissile;
use poison::Poison;
use recharge::Recharge;
use shield::Shield;

#[derive(Debug)]
pub struct Wizard {
    name: String,
    default_hp: i32,
    current_hp: i32,
    default_mana: i32,
    current_mana: i32,
    used_mana: i32,
    armor: i32,
    effects: Vec<Box<dyn Effect>>,
    round_nr: i32,
}

impl Wizard {
    //create instance
    pub fn new(player_name: &str) -> Self
    {
        let mut ret_val: Wizard = Wizard{name: player_name.to_string(), 
                                        default_hp: 50 , 
                                        current_hp: 50, 
                                        default_mana: 500, 
                                        current_mana: 500, 
                                        used_mana: 0,
                                        armor: 0,  
                                        effects: Vec::new(),
                                        round_nr: 0 };
        
        ret_val.effects.push(Box::new(Drain::new()));
        ret_val.effects.push(Box::new(MagicMissile::new()));
        ret_val.effects.push(Box::new(Poison::new()));
        ret_val.effects.push(Box::new(Recharge::new()));
        ret_val.effects.push(Box::new(Shield::new()));
        
        ret_val
    }
}

impl Character for Wizard {
    //reset HP & mana stats of character
    fn reset(&mut self)
    {
        self.current_hp = self.default_hp;
        self.current_hp = self.default_mana;
        self.used_mana = 0;
    }
    
    //print name string
    fn get_name(&self) -> String
    {
        self.name.clone()
    }

    //character attacks, returns damage done
    fn attack(&mut self, enemy: &mut impl Character)
    {
        //_ = enemy.take_damage(9);
        self.round_nr += 1;
        match self.round_nr {
            1 => {
                println!("Came to round {}, but we have no more plan. \n {} mana used", self.round_nr, self.used_mana);
            },
            _ =>  println!("Came to round {}, but we have no more plan. \n {} mana used", self.round_nr, self.used_mana),
        }
        //todo!("shit ton big algorithm...");
        //use Character.take_dmg and add_effect
        //or apply to self
        //every spell must consume mana
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

    //retrieve list with all active effects
    fn get_active_effects(&self) -> &Vec<Box<dyn Effect>>
    {
        &self.effects
    }

    //execute all effects
    fn execute_effects(&mut self)
    {
        self.armor = 0;
        if self.effects.len() > 1
        {
            for effect in &mut self.effects
            {
                self.armor = self.armor + effect.get_armor();
                self.current_hp -= effect.get_dmg();
                self.current_hp += effect.get_healing();
                self.current_mana += effect.get_mana();
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

impl Wizard {
    fn cast_effect_on_enemy(&self, target: &mut impl Character, effect: &Box<dyn Effect>) {
        if !target.get_active_effects().iter().any(|effect_on_target| effect.get_name() == effect_on_target.get_name())
        {
            //effect is not in list
            effect.
            if effect.get_rounds_active() > 0
            {

            }
        }
        else {
            println!("{} was already in effect on enemy, and thus not cast!", effect.get_name());
        }
    }
}


