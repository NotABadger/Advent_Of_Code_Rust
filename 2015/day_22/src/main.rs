
mod char_trait;
mod effect_trait;
mod wizard;
mod enemy;
mod attack;
mod battle_scene;

use crate::char_trait::Character;
use crate::wizard::Wizard;
use crate::enemy::Enemy;

fn main() {
    println!("Starting battle wizards");
    let mut player: Wizard = Wizard::new("Dave the magic wizard");
    let mut boss: Enemy = Enemy::new();

    //fight!
    let rounds_played = battle_scene::fight_battle(&mut player, &mut boss);
    let winner: & dyn Character;
    if player.check_alive() 
    { winner = &player} 
    else 
    {winner = &boss}
    println!("After playing {} rounds, {} won!", rounds_played, winner.get_name());

    /* Remaining todo.
        Implement all spells
        Implement spell choosing algoritm
        Win battles */
}
