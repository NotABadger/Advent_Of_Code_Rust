mod char_trait;
mod effect_trait;
mod wizard;
mod enemy;
mod battle_scene;
mod spells;

use battle_scene::BattleScene;


fn main() {
    println!("Starting battle wizards");
    let mut current_round: Vec<BattleScene> = Vec::new();
    let mut next_round: Vec<BattleScene> = Vec::new();

    current_round.push(BattleScene::new());

    // create scenario's. (Pick any spell)
    // Add to next-to-play-list.
    // Copy next-to-play-list. to current-playing-list
    // For each Scenario, start thread with a list index and spell to prepare
    // if Wizard does not survive turn, delete/do not add to next-to-play-list
    // if bad guy is dead -> Add mana used and spells played to fight-done-list
    // if Wizard survives turn & bad guy alive -> add for each spell option to next-to-play-list
    // Loop back to "copy step" 
    
}
