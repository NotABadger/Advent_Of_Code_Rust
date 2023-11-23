use crate::combattant::Combattant;

const ROUNDS_OF_FIGHTING: u32 = 100;

pub fn won_fight(player: &mut Combattant, enemy: &mut Combattant) -> bool
{
    player.update_stats();
    player.reset_hp();
    enemy.reset_hp();

    for _round in 1 ..= ROUNDS_OF_FIGHTING
    {
        //player hit enemy
        {
            let mut damage: i32 = player.damage as i32;
            damage -= enemy.armor as i32;
            if damage < 1
            {
                damage = 1;
            }
            enemy.current_hp -= damage;
            if enemy.current_hp <= 0
            {
                //println!("Enemy died in round {}", round);
                break;
            }
        }
        //enemy hit player
        {
            let mut damage: i32 = enemy.damage as i32;
            damage -= player.armor as i32;
            if damage < 1
            {
                damage = 1;
            }
            player.current_hp -= damage;
            if player.current_hp <= 0
            {
                //println!("player died in round {}", round);
                break;
            }
        }
    }
    // println!("Fight is over!");
    // println!("player HP: {}", player.current_hp);
    // println!("enemy HP: {}", enemy.current_hp);
    if player.current_hp <= 0
    {
        return false;
    }
    true
}

