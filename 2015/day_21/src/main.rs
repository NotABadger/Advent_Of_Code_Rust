mod item;
mod combattant;
mod fight_ring;
mod shop;

use crate::combattant::Combattant;
use crate::item::*;
use crate::fight_ring::won_fight;
use crate::shop::Shop;

fn main() {
    
    let mut player: Combattant = Combattant::default();
    let mut enemy: Combattant = Combattant { name: "Evil_bob".to_string(), 
                                                max_hp: 104, 
                                                current_hp: 104, 
                                                damage: 8, 
                                                armor: 1, 
                                                armor_item: Item::new_armor(0, 0), 
                                                weapon_item: Item::new_weapon(0, 0), 
                                                ring_items: [Item::new_ring(0, 0, 0), Item::new_ring(0, 0, 0)],
                                            };
    let mut shop: Shop = Shop::default();

    while !won_fight(&mut player, &mut enemy)
    {//fught until it's won
        player = shop.try_different_loadout();
    }

    println!("the minimum value required to win is: {}", player.get_total_value());
    // end of part 1;
    //prep part 2;
    shop = Shop::default();
    player = shop.try_expensive_loadout();
    let mut still_losing: Combattant = player.clone();
    
    while won_fight(&mut player, &mut enemy)
    {//Fight until loss
        still_losing = player.clone();
        player = shop.try_expensive_loadout();
    }

    println!("the maximum value and still losing is: {}", still_losing.get_total_value());

}
