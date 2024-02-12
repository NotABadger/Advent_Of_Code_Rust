use crate::char_trait::Character;
use crate::enemy::Enemy;
use crate::wizard::Wizard;
use crate::effect_trait::Effect;

#[derive(Debug)]
pub struct BattleScene {
    current_round: i32,
    player: Wizard,
    boss: Enemy,
    spell_history: Vec<Box<dyn Effect>>,
}

impl BattleScene {

    //new instance
    pub fn new() -> Self {
        BattleScene {
            current_round: 1,
            player: Wizard::new("bob the magic wizard"),
            boss: Enemy::new(),
            spell_history: Vec::new(),
        }
    }

    //Prepare next possible attack rounds for 
    pub fn prepare_next_possible_rounds(&self) -> Option<Vec<BattleScene>>
    {
        let mut possible_outcomes_list: Vec<Self> = Vec::new();
        let mut ret_val: Option<Vec<BattleScene>> = None;
        //check all spells
        for effect in self.player.get_spells_list()
        {
            if effect.get_rounds_active() == 0 
            {
                if effect.get_cost() < self.player.get_current_mana()
                {
                    let mut possible_outcome: BattleScene = self.clone();
                    possible_outcome.player.set_next_attack(effect);
                    possible_outcome.add_spell_to_history(effect);
                    possible_outcomes_list.push(possible_outcome);
                    continue;
                }
            }
            else if !self.player.get_active_effects().iter().any(|active | active.get_name() == effect.get_name()) 
                    && !self.boss.get_active_effects().iter().any(|active| active.get_name() == effect.get_name())
                    && effect.get_cost() < self.player.get_current_mana()
            {
                if effect.get_cost() < self.player.get_current_mana()
                {
                    let mut possible_outcome: BattleScene = self.clone();
                    possible_outcome.player.set_next_attack(effect);
                    possible_outcome.add_spell_to_history(effect);
                    possible_outcomes_list.push(possible_outcome);
                    continue;
                }
            }
        }

        if possible_outcomes_list.len() > 1
        {
            ret_val = Some(possible_outcomes_list);
        }
        ret_val
    }

    //Fight a round, return win/loss
    pub fn fight_round(&mut self) -> RoundResult
    {
        self.player.attack(&mut self.boss);
        if !self.boss.check_alive()
        {
            return RoundResult::PlayerWin(self.current_round, self.player.get_spent_mana());
        }
        self.boss.attack(&mut self.player);
        if !self.player.check_alive()
        {
            return RoundResult::BossWin(self.current_round);
        }

        self.current_round += 1;
        self.player.execute_effects();
        self.boss.execute_effects();

        if !self.boss.check_alive()
        {
            return RoundResult::PlayerWin(self.current_round, self.player.get_spent_mana());
        }
        if !self.player.check_alive()
        {
            return RoundResult::BossWin(self.current_round);
        }

        RoundResult::NextRound   
    }

    fn add_spell_to_history(&mut self, spell: &Box<dyn Effect>)
    {
        self.spell_history.push(spell.deep_copy_effect());
    }

    pub fn get_mana_spent_in_battle(&self) -> i32
    {
        self.player.get_spent_mana()
    }

}

impl Clone for BattleScene {
    fn clone(&self) -> Self {
        let mut history_cpy: Vec<Box<dyn Effect>> = Vec::new();
        for spell in &self.spell_history
        {
            history_cpy.push(spell.deep_copy_effect());
        }

        Self { current_round: self.current_round,
                player: self.player.clone(),
                boss: self.boss.clone(),
                spell_history: history_cpy
             }
    }
}

pub enum RoundResult {
    PlayerWin(i32, i32),
    BossWin(i32),
    NextRound,    
}