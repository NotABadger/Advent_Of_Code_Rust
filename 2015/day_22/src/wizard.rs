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
    spent_mana: i32,
    armor: i32,
    spell_list: Vec<Box<dyn Effect>>,
    active_effects: Vec<Box<dyn Effect>>,
    next_attack: Option<Box<dyn Effect>>,
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
                                        spent_mana: 0,
                                        armor: 0,  
                                        spell_list: Vec::new(),
                                        active_effects: Vec::new(),
                                        next_attack: None,
                                        round_nr: 0 };
        
        ret_val.spell_list.push(Box::new(Drain::new()));
        ret_val.spell_list.push(Box::new(MagicMissile::new()));
        ret_val.spell_list.push(Box::new(Poison::new()));
        ret_val.spell_list.push(Box::new(Recharge::new()));
        ret_val.spell_list.push(Box::new(Shield::new()));
        
        ret_val
    }

    pub fn get_spells_list(&self) -> &Vec<Box<dyn Effect>>
    {
        &self.spell_list
    }

    pub fn set_next_attack(&mut self, attack_efffect: &Box<dyn Effect>)
    {
        self.next_attack = Some(attack_efffect.deep_copy_effect());
    }

    pub fn get_current_mana(&self) -> i32
    {
        self.current_mana
    }

    pub fn get_spent_mana(&self) -> i32
    {
        self.spent_mana
    }
}

impl Character for Wizard {
    //reset HP & mana stats of character
    fn reset(&mut self)
    {
        self.current_hp = self.default_hp;
        self.current_hp = self.default_mana;
        self.spent_mana = 0;
    }
    
    //print name string
    fn get_name(&self) -> String
    {
        self.name.clone()
    }

    //character attacks, returns damage done
    fn attack(&mut self, enemy: &mut impl Character)
    {
        assert!(self.next_attack.is_some(), "No next attack was selected for the Wizard");

        let attack_unwrapped: Box<dyn Effect> = self.next_attack.as_ref().unwrap().deep_copy_effect();
        assert!(self.current_mana > attack_unwrapped.get_cost(), "Mana cost is too high for selected next spell");

        self.current_mana -= attack_unwrapped.get_cost();
        self.spent_mana += attack_unwrapped.get_cost();

        if attack_unwrapped.get_rounds_active() == 0
        {
            self.current_hp += attack_unwrapped.get_healing();
            self.current_mana += attack_unwrapped.get_mana();
            self.armor += attack_unwrapped.get_armor();
            enemy.take_damage(attack_unwrapped.get_dmg());
        }
        else {
            match attack_unwrapped.get_name().as_str()
            {
                Poison::NAME => {
                    enemy.add_effect(&attack_unwrapped);
                },
                Recharge::NAME => {
                    self.add_effect(&attack_unwrapped);
                },
                Shield::NAME => {
                    self.add_effect(&attack_unwrapped);
                },
                _ => panic!("Don't know this spell...."),
            }
        }
        self.next_attack = None;
    }

    //take damage, return remaining hp
    fn take_damage(&mut self, damage: i32) -> i32
    {
        let mut dmg_taken: i32 = damage - self.armor;
        if dmg_taken < 1
        {
            dmg_taken = 1;
        }
        self.current_hp -= dmg_taken;
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

    //execute all effects
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
                self.current_mana += effect.get_mana();
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

impl Clone for Wizard {
    fn clone(&self) -> Self {
        let mut spell_list_cpy: Vec<Box<dyn Effect>> = Vec::new();
        for spell in self.get_spells_list()
        {
            spell_list_cpy.push(spell.deep_copy_effect());
        }
        let mut active_effects_cpy: Vec<Box<dyn Effect>> = Vec::new();
        for active_effect in self.get_active_effects()
        {
            active_effects_cpy.push(active_effect.deep_copy_effect());
        }

        let next_attack_cpy: Option<Box<dyn Effect>> = match &self.next_attack
        {
            Some(effect_box) =>  Some(effect_box.deep_copy_effect()),
            None => None,
        };


        Self { name: self.get_name(), 
            default_hp: self.default_hp, 
            current_hp: self.current_hp, 
            default_mana: self.default_mana, 
            current_mana: self.current_mana, 
            spent_mana: self.spent_mana, 
            armor: self.armor, 
            spell_list: spell_list_cpy, 
            active_effects: active_effects_cpy, 
            next_attack: next_attack_cpy, 
            round_nr: self.round_nr 
        }
    }
}