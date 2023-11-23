use crate::Item;
use crate::Combattant;

pub struct Shop{
    weapon_list: Vec<Item>,
    armor_list: Vec<Item>,
    ring_list: Vec<Item>,
    loadout_list: Vec<Combattant>
}

impl Shop
{
    pub fn try_different_loadout(&mut self) -> Combattant
    {
        //Try all combinations 
        if self.loadout_list.len() <= 0
        {
            self.calculate_all_loadout_combos();
        }
        self.loadout_list.remove(0)
    }

    pub fn try_expensive_loadout(&mut self) -> Combattant
    {
        //Try all combinations 
        if self.loadout_list.len() <= 0
        {
            self.calculate_all_loadout_combos();
        }
        self.loadout_list.pop().unwrap()
    }

    fn calculate_all_loadout_combos(&mut self)
    {
        //try all combo's and order on total cost
        // possibilities are made out of: weapon, (optional)armor, (optional) up to two rinds
        //just weapons
        self.weapon_list.iter().for_each(| weapon| { 
            let mut combattant = Combattant::default();
            combattant.weapon_item = weapon.clone();
            self.loadout_list.push(combattant); 
         });
         //weapons and armor, no rings
         self.weapon_list.iter().for_each(| weapon| { 
            self.armor_list.iter().for_each(|armor| {
                let mut combattant = Combattant::default();
                combattant.weapon_item = weapon.clone();
                combattant.armor_item = armor.clone();
                self.loadout_list.push(combattant); 
            });
         });

         //weapon, no armor + 1 ring
         self.weapon_list.iter().for_each(| weapon| { 
            self.ring_list.iter().for_each(|ring | {
                let mut combattant = Combattant::default();
                combattant.weapon_item = weapon.clone();
                combattant.ring_items[0] = ring.clone();
                self.loadout_list.push(combattant); 
            });
         });
         //weapon, no armor + 2 rings
         self.weapon_list.iter().for_each(| weapon| { 
            self.ring_list.iter().for_each(|ring | {
                self.ring_list.iter().for_each(|ring2 | {
                    if ring2.damage != ring.damage || ring2.armor != ring.armor
                    {
                        let mut combattant = Combattant::default();
                        combattant.weapon_item = weapon.clone();
                        combattant.ring_items[0] = ring.clone();
                        combattant.ring_items[1] = ring2.clone();
                        self.loadout_list.push(combattant);
                    }
                }); 
            });
        });


         //weapon, armor + 1 ring
         self.weapon_list.iter().for_each(| weapon| { 
            self.armor_list.iter().for_each(|armor| {
                self.ring_list.iter().for_each(|ring | {
                    let mut combattant = Combattant::default();
                    combattant.weapon_item = weapon.clone();
                    combattant.armor_item = armor.clone();
                    combattant.ring_items[0] = ring.clone();
                    self.loadout_list.push(combattant); 
                });
             });
         });
         //weapon, armor + 2 rings
         self.weapon_list.iter().for_each(| weapon| { 
            self.armor_list.iter().for_each(|armor| {
                self.ring_list.iter().for_each(|ring | {
                    self.ring_list.iter().for_each(|ring2 | {
                        if ring2.damage != ring.damage || ring2.armor != ring.armor
                        {
                            let mut combattant = Combattant::default();
                            combattant.weapon_item = weapon.clone();
                            combattant.armor_item = armor.clone();
                            combattant.ring_items[0] = ring.clone();
                            combattant.ring_items[1] = ring2.clone();
                            self.loadout_list.push(combattant);
                        }
                    }); 
                });
             });
         });

         self.loadout_list.sort_by(|a, b| a.get_total_value().cmp(&b.get_total_value()));

    }
}

impl Default for Shop{
    fn default() -> Self {
        Self { weapon_list: vec![Item::new_weapon(8, 4),
                                    Item::new_weapon(10, 5), 
                                    Item::new_weapon(25, 6),
                                    Item::new_weapon(40, 7),
                                    Item::new_weapon(74, 8),], 
            armor_list: vec![Item::new_armor(13, 1),
                                Item::new_armor(31, 2),
                                Item::new_armor(53, 3),
                                Item::new_armor(75, 4),
                                Item::new_armor(102, 5),], 
            ring_list: vec![Item::new_ring(25, 0, 1),
                                Item::new_ring(50, 0, 2),
                                Item::new_ring(100, 0, 3),
                                Item::new_ring(20, 1, 0),
                                Item::new_ring(40, 2, 0),
                                Item::new_ring(80, 3, 0),],
            loadout_list: Vec::new(),
        }
    }
}
