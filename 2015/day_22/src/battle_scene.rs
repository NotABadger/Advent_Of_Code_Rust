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
        match player.attack() {
            Some(effect) => boss.add_effect(effect),
            None => (),
        }

        boss.execute_effects();
        if !boss.check_alive()
        {//boss died;
            break;
        }
        let boss_attack = boss.attack().unwrap();
        if player.take_damage(boss_attack.get_dmg()) < 1
        {//player died
            break;
        }

        
    }

    current_round
}