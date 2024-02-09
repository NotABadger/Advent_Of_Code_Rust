use crate::char_trait::Character;
use crate::enemy::Enemy;
use crate::wizard::Wizard;
use crate::effect_trait::Effect;

#[derive(Clone)]
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
        player: Wizard::new("bob the magic wizard"),
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
            let mut possible_outcome: BattleScene = self.clone();
            possible_outcome.player.set_next_attack(effect);
        }
    }

    if possible_outcomes_list.len() > 1
    {
        ret_val = Some(possible_outcomes_list);
    }
    ret_val
}



//returns amount of rounds fought
pub fn fight_battle(player: &mut impl Character,  boss: &mut impl Character) -> i32
{
    let mut current_round: i32 = 0;
    player.reset();
    boss.reset();

    loop
    {
        current_round += 1;
        player.execute_effects();
        boss.execute_effects();

        if !boss.check_alive()
        {
            println!("The Wizard won!");
            break;
        }
        if !player.check_alive()
        {
            println!("The Boss won!");
            break;
        }

        player.attack(boss);
        if !boss.check_alive()
        {
            println!("The Wizard won!");
            break;
        }
        boss.attack(player);
        if !player.check_alive()
        {
            println!("The Boss won!");
            break;
        }
    }

    current_round
}

}