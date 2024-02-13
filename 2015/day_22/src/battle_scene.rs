use crate::char_trait::Character;
use crate::enemy::Enemy;
use crate::wizard::Wizard;

#[derive(Debug)]
pub struct BattleScene {
    current_round: i32,
    player: Wizard,
    boss: Enemy,
}

impl BattleScene {

    //new instance
    pub fn new() -> Self {
        BattleScene {
            current_round: 0,
            player: Wizard::new(),
            boss: Enemy::new(),
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
                    possible_outcomes_list.push(possible_outcome);
                    continue;
                }
            }
            else if !self.player.get_active_effects().iter().any(|active | active.get_name() == effect.get_name()) 
                    && !self.boss.get_active_effects().iter().any(|active| active.get_name() == effect.get_name())
            {
                if effect.get_cost() < self.player.get_current_mana()
                {
                    let mut possible_outcome: BattleScene = self.clone();
                    possible_outcome.player.set_next_attack(effect);
                    possible_outcomes_list.push(possible_outcome);
                    continue;
                }
            }
        }

        if !possible_outcomes_list.is_empty()
        {
            ret_val = Some(possible_outcomes_list);
        }
        ret_val
    }

    //Fight a round, return win/loss
    pub fn fight_round(&mut self) -> RoundResult
    {
        //make choice for attack
        self.player.attack(&mut self.boss);
        if !self.boss.check_alive()
        {
            return RoundResult::PlayerWin(self.player.get_spent_mana());
        }

        // begin boss turn
        self.boss.execute_effects();
        if !self.boss.check_alive()
        {
            return RoundResult::PlayerWin(self.player.get_spent_mana());
        }
        self.player.execute_effects();
        if !self.player.check_alive()
        {
            return RoundResult::BossWin;
        }

        self.boss.attack(&mut self.player);
        if !self.player.check_alive()
        {
            return RoundResult::BossWin;
        }
        //next round
        //begin player turn
        self.player.take_damage(1);
        if !self.player.check_alive()
        {
            return RoundResult::BossWin;
        }
        self.boss.execute_effects();
        if !self.boss.check_alive()
        {
            return RoundResult::PlayerWin(self.player.get_spent_mana());
        }
        self.player.execute_effects();
        if !self.player.check_alive()
        {

            return RoundResult::BossWin;
        }

        RoundResult::NextRound   
    }

    pub fn get_mana_spent_in_battle(&self) -> i32
    {
        self.player.get_spent_mana()
    }

}

impl Clone for BattleScene {
    fn clone(&self) -> Self {
        Self { current_round: self.current_round,
                player: self.player.clone(),
                boss: self.boss.clone(),
             }
    }
}

pub enum RoundResult {
    PlayerWin(i32),
    BossWin,
    NextRound,    
}