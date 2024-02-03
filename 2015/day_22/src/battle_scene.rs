use crate::char_trait::Character;


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