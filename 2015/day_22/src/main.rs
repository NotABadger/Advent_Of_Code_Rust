
mod char_trait;
mod effect_trait;
mod wizard;
mod enemy;
mod battle_scene;
mod spells;

use crate::wizard::Wizard;
use crate::enemy::Enemy;

fn main() {
    println!("Starting battle wizards");
    let mut player: Wizard = Wizard::new("Dave the magic wizard");
    let mut boss: Enemy = Enemy::new();

    //fight!
    let rounds_played = battle_scene::fight_battle(&mut player, &mut boss);
    println!("After playing {} rounds.", rounds_played);

    /* Remaining todo.
        Implement spell choosing algoritm
        Win battles */
}
