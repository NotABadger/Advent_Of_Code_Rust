mod char_trait;
mod effect_trait;
mod wizard;
mod enemy;
mod battle_scene;
mod spells;

use battle_scene::BattleScene;

use crate::battle_scene::RoundResult;


fn main() {
    // ALGORITHM:
    // create scenario's. (Pick any spell)
    // Add to next-to-play-list.
    // Copy next-to-play-list. to current-playing-list
    // For each Scenario, start thread with a list index and spell to prepare
    // if Wizard does not survive turn, delete/do not add to next-to-play-list
    // if bad guy is dead -> Add mana used and spells played to fight-done-list
    // if Wizard survives turn & bad guy alive -> add for each spell option to next-to-play-list
    // Loop back to "copy step" 
    // Note: 833 is too low! 

    println!("Starting battle wizards");
    let mut current_round: Vec<BattleScene> = Vec::new();
    let mut next_round: Vec<BattleScene> = Vec::new();
    let first_battle_scene: BattleScene = BattleScene::new();
    let mut finished_battles: Vec<i32> = Vec::new();

    let mut first_win_in: bool = false; //mark first win

    current_round.append(&mut first_battle_scene.prepare_next_possible_rounds().unwrap());
    //single threaded application
    while current_round.len() > 0
    {
        println!("Started the round with {} scenario's", current_round.len());
        for round in &mut current_round
        {
            match round.fight_round() {
                RoundResult::NextRound => {
                    match round.prepare_next_possible_rounds().as_mut()
                    {
                        Some(list_of_next_rounds) => next_round.append(list_of_next_rounds),
                        _ => (),
                    }
                },
                RoundResult::BossWin => (),
                RoundResult::PlayerWin(mana_spent) => {
                    first_win_in = true;
                    finished_battles.push(mana_spent);
                }
            }
        }

        if first_win_in {
            let mut lowest_mana_used: i32 = 9999;
            finished_battles.iter().for_each(|element: &i32 | if (*element) < lowest_mana_used {lowest_mana_used = *element} );
            current_round = Vec::new();
            for battlerscene in  next_round.iter().filter(|element | element.get_mana_spent_in_battle() < lowest_mana_used)
            {
                current_round.push(battlerscene.clone());
            }
        }
        else {
            current_round = next_round;
        }
        next_round = Vec::new();
    }

    finished_battles.sort_by(|win1, win2| win1.cmp(&win2));
    println!("Round won with least mana spent being: {}", finished_battles.first().unwrap());  
}
