use colored::Colorize;

use crate::items::{Item, ItemType};
use crate::rooms::{Room, RoomType};
use crate::{items, player};

#[derive(Debug, Clone)]
pub struct EquippedItems {
    pub head: Option<Item>,
    pub body: Option<Item>,
    pub left_hand: Option<Item>,
    pub right_hand: Option<Item>,
    pub feet: Option<Item>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum HungerState {
    Satiated, //0
    Grumbly,  //50
    Hungry,   //75
    Ravenous, //90
    Starving, //100
}

#[derive(Debug, Clone)]
pub struct Player {
    pub name: String,
    pub health: u32,
    pub max_health: u32,
    pub hunger: u32,
    pub max_hunger: u32,
    hunger_state: HungerState,
    pub damage: u32,
    pub defense: u32,
    pub inventory: Vec<Item>,
    pub gold: u32,
    pub equipped: EquippedItems,
    pub buffs: Vec<Item>,
}

impl Player {
    ///new player
    pub fn new(
        name: String,
        health: u32,
        max_hunger: u32,
        damage: u32,
        defense: u32,
        inventory: Vec<Item>,
        gold: u32,
        equipped: EquippedItems,
        buffs: Vec<Item>,
    ) -> Self {
        Player {
            name,
            health,
            max_health: health,
            hunger: 0,
            hunger_state: HungerState::Satiated,
            max_hunger,
            damage,
            defense,
            inventory,
            gold,
            equipped,
            buffs,
        }
    }

    pub fn on_move(&mut self) {
        self.hunger = (self.hunger + 1).min(self.max_hunger);
        if self.hunger >= self.max_hunger {
            self.health = self.health.saturating_sub(1);
            println!("You're starving! You lost 1 health.");
            self.hunger_state = HungerState::Starving;
        } else if self.hunger >= self.max_hunger * 9 / 10
            && self.hunger_state != HungerState::Ravenous
        {
            println!("You're ravenous! Find food before you start starving.");
            self.hunger_state = HungerState::Ravenous;
        } else if self.hunger >= self.max_hunger * 3 / 4 && self.hunger_state != HungerState::Hungry
        {
            println!("Your stomach growls loudly. You need food soon.");
            self.hunger_state = HungerState::Hungry;
        } else if self.hunger >= self.max_hunger / 2 && self.hunger_state != HungerState::Grumbly {
            println!("You're starting to feel hungry.");
            self.hunger_state = HungerState::Grumbly;
        }

        self.tick_buffs();
    }

    /// Use an item from the inventory
    pub fn use_item(&mut self, room: &mut Room, item: Item) -> (bool, Option<String>) {
        match item.item_type {
            ItemType::Potion { healing } => {
                self.health += healing;
                let line = format!("You drink the {} and heal for {}!", item.name, healing);
                (true, Some(line)) // Consumed
            }
            ItemType::Poison { damage } => {
                self.health = self.health.saturating_sub(damage);
                let line = format!(
                    "Ouch! The {} dealt {} damage to you! Idiot.",
                    item.name,
                    damage.to_string().red()
                );
                (true, Some(line)) // Consumed
            }
            ItemType::Key { unlocks: _ } => {
                // Logic to check if 'unlocks' matches something in the room
                let mut line = format!("You try to use the {}...", item.name);
                if room.room_type == RoomType::TreasureRoom {
                    room.items.push(items::create_gold());
                    line += format!(
                        "\nYou use the key to open the chest and find a bag of gold inside!"
                    )
                    .as_str();
                } else {
                    line += format!("\nThere is nothing in this room to use it on.").as_str();
                }
                (false, Some(line)) // Not consumed
            }
            ItemType::Gold { gold } => {
                self.gold += gold;
                let line = format!("You pick up the gold and add it to your purse.\n");
                (true, Some(line)) // Consumed
            }
            ItemType::Junk => {
                let line = format!("You fiddle with the {}, but nothing happens.\n", item.name);
                (false, Some(line)) // Not consumed
            }
            ItemType::Equipment {
                ref slot,
                damage: _,
                defense: _,
                durability: _,
            } => {
                if slot == "Hand" {
                    let (left, right) = self.get_equipped_hands();
                    if left.is_none() || right.is_none() {
                        // There is room to equip the item
                        let line = format!(
                            "\n{} was equipped on {}\n",
                            &item.name,
                            if left.is_some() {
                                "left hand"
                            } else {
                                "right hand"
                            }
                        );
                        self.equip_item(item);
                        return (true, Some(line));
                    }
                    // There is no room to equip the item
                    let line = format!(
                        "You try to equip the {}, but your hands are full.",
                        item.name
                    );
                    return (false, Some(line));
                } else {
                    let avail_slot = self.get_equipped_slot(&slot);
                    if avail_slot.is_none() {
                        // There is room to equip the item
                        let line = format!(
                            "\n{} was equipped on {}\n",
                            &item.name,
                            &slot.to_lowercase()
                        );
                        self.equip_item(item);
                        return (true, Some(line));
                    }
                    // There is already an item equipped in that slot
                    let line = format!(
                        "You try to equip the {}, but you already have something equipped in that slot.",
                        item.name
                    );
                    return (false, Some(line));
                }
            }
            ItemType::Buff { .. } => {
                let line = format!("You drink the {} and feel a surge of power!", item.name);
                self.buffs.push(item);
                (false, Some(line))
            }
            ItemType::Food { hunger_restore } => {
                self.hunger = self.hunger.saturating_sub(hunger_restore);
                let line = format!(
                    "You eat the {} and your hunger decreases by {}.",
                    item.name, hunger_restore
                );
                if (self.hunger >= self.max_hunger) {
                    self.hunger_state = HungerState::Starving;
                } else if self.hunger >= self.max_hunger * 9 / 10 {
                    self.hunger_state = HungerState::Ravenous;
                } else if self.hunger >= self.max_hunger * 3 / 4 {
                    self.hunger_state = HungerState::Hungry;
                } else if self.hunger >= self.max_hunger / 2 {
                    self.hunger_state = HungerState::Grumbly;
                } else {
                    self.hunger_state = HungerState::Satiated;
                }
                (true, Some(line))
            }
        }
    }

    pub fn get_equipped_slot(&self, slot: &str) -> Option<&Item> {
        match slot {
            "Head" => self.equipped.head.as_ref(),
            "Body" => self.equipped.body.as_ref(),
            "Feet" => self.equipped.feet.as_ref(),
            _ => None,
        }
    }
    pub fn get_equipped_hands(&self) -> (Option<&Item>, Option<&Item>) {
        (
            self.equipped.left_hand.as_ref(),
            self.equipped.right_hand.as_ref(),
        )
    }

    pub fn equip_item(&mut self, item: Item) {
        match item.item_type {
            ItemType::Equipment {
                ref slot,
                damage: _,
                defense: _,
                durability: _,
            } => match slot.as_str() {
                "Head" => self.equipped.head = Some(item),
                "Body" => self.equipped.body = Some(item),
                "Hand" => {
                    if self.equipped.left_hand.is_none() {
                        self.equipped.left_hand = Some(item);
                    } else if self.equipped.right_hand.is_none() {
                        self.equipped.right_hand = Some(item);
                    }
                }
                "Feet" => self.equipped.feet = Some(item),
                _ => println!("You want to put that item where!?"),
            },
            _ => println!("You can't equip that item."),
        }
    }

    /// player base damage + equipped items damage
    pub fn total_damage(&self) -> u32 {
        self.damage
            + self
                .equipped
                .head
                .as_ref()
                .map_or(0, |i| i.equipment_stats().1)
            + self
                .equipped
                .body
                .as_ref()
                .map_or(0, |i| i.equipment_stats().1)
            + self
                .equipped
                .left_hand
                .as_ref()
                .map_or(0, |i| i.equipment_stats().1)
            + self
                .equipped
                .right_hand
                .as_ref()
                .map_or(0, |i| i.equipment_stats().1)
            + self
                .equipped
                .feet
                .as_ref()
                .map_or(0, |i| i.equipment_stats().1)
            + self
                .buffs
                .iter()
                .filter(|b| {
                    matches!(
                        b.item_type,
                        ItemType::Buff {
                            buff_type: items::BuffType::Damage,
                            ..
                        }
                    )
                })
                .map(|b| {
                    if let ItemType::Buff { value, .. } = &b.item_type {
                        *value
                    } else {
                        0
                    }
                })
                .sum::<u32>()
    }

    /// player base defense + equipped items defense
    pub fn total_defense(&self) -> u32 {
        self.defense
            + self
                .equipped
                .head
                .as_ref()
                .map_or(0, |i| i.equipment_stats().2)
            + self
                .equipped
                .body
                .as_ref()
                .map_or(0, |i| i.equipment_stats().2)
            + self
                .equipped
                .left_hand
                .as_ref()
                .map_or(0, |i| i.equipment_stats().2)
            + self
                .equipped
                .right_hand
                .as_ref()
                .map_or(0, |i| i.equipment_stats().2)
            + self
                .equipped
                .feet
                .as_ref()
                .map_or(0, |i| i.equipment_stats().2)
            + self
                .buffs
                .iter()
                .filter(|b| {
                    matches!(
                        b.item_type,
                        ItemType::Buff {
                            buff_type: items::BuffType::Defense,
                            ..
                        }
                    )
                })
                .map(|b| {
                    if let ItemType::Buff { value, .. } = &b.item_type {
                        *value
                    } else {
                        0
                    }
                })
                .sum::<u32>()
    }

    pub fn wear_damage_equipment(&mut self) {
        if let Some(Item {
            item_type: ItemType::Equipment { damage, .. },
            ..
        }) = self.equipped.head
        {
            if damage > 0 {
                tick(&mut self.equipped.head);
            }
        }
        if let Some(Item {
            item_type: ItemType::Equipment { damage, .. },
            ..
        }) = self.equipped.body
        {
            if damage > 0 {
                tick(&mut self.equipped.body);
            }
        }
        if let Some(Item {
            item_type: ItemType::Equipment { damage, .. },
            ..
        }) = self.equipped.left_hand
        {
            if damage > 0 {
                tick(&mut self.equipped.left_hand);
            }
        }
        if let Some(Item {
            item_type: ItemType::Equipment { damage, .. },
            ..
        }) = self.equipped.right_hand
        {
            if damage > 0 {
                tick(&mut self.equipped.right_hand);
            }
        }
        if let Some(Item {
            item_type: ItemType::Equipment { damage, .. },
            ..
        }) = self.equipped.feet
        {
            if damage > 0 {
                tick(&mut self.equipped.feet);
            }
        }
    }

    pub fn wear_defense_equipment(&mut self) {
        if let Some(Item {
            item_type: ItemType::Equipment { defense, .. },
            ..
        }) = self.equipped.head
        {
            if defense > 0 {
                tick(&mut self.equipped.head);
            }
        }
        if let Some(Item {
            item_type: ItemType::Equipment { defense, .. },
            ..
        }) = self.equipped.body
        {
            if defense > 0 {
                tick(&mut self.equipped.body);
            }
        }
        if let Some(Item {
            item_type: ItemType::Equipment { defense, .. },
            ..
        }) = self.equipped.left_hand
        {
            if defense > 0 {
                tick(&mut self.equipped.left_hand);
            }
        }
        if let Some(Item {
            item_type: ItemType::Equipment { defense, .. },
            ..
        }) = self.equipped.right_hand
        {
            if defense > 0 {
                tick(&mut self.equipped.right_hand);
            }
        }
        if let Some(Item {
            item_type: ItemType::Equipment { defense, .. },
            ..
        }) = self.equipped.feet
        {
            if defense > 0 {
                tick(&mut self.equipped.feet);
            }
        }
    }

    /// color health based on percentage
    pub fn color_health(&self) -> colored::ColoredString {
        let text = self.health.to_string();
        let percentage = (self.health * 100) / self.max_health;
        if percentage >= 75 {
            text.green()
        } else if percentage >= 30 {
            text.yellow()
        } else {
            text.red()
        }
    }

    pub fn tick_buffs(&mut self) {
        for buff in &mut self.buffs {
            if let ItemType::Buff {
                turns_remaining, ..
            } = &mut buff.item_type
            {
                *turns_remaining = turns_remaining.saturating_sub(1);
            }
        }
        self.buffs.retain(|b| {
            if let ItemType::Buff {
                turns_remaining, ..
            } = &b.item_type
            {
                *turns_remaining > 0
            } else {
                true
            }
        });
    }
}

fn tick(slot: &mut Option<Item>) {
    if let Some(item) = slot {
        if let ItemType::Equipment { durability, .. } = &mut item.item_type {
            *durability = durability.saturating_sub(1);
            if *durability == 0 {
                println!("Your {} broke!", item.name);
                *slot = None;
            }
        }
    }
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    fn create_test_player() -> Player {
        Player {
            name: String::from("Test Player"),
            health: 20,
            max_health: 20,
            hunger: 20,
            max_hunger: 20,
            hunger_state: HungerState::Satiated,
            damage: 5,
            defense: 0,
            inventory: Vec::new(),
            gold: 0,
            equipped: EquippedItems {
                head: None,
                body: None,
                left_hand: None,
                right_hand: None,
                feet: None,
            },
            buffs: Vec::new(),
        }
    }

    fn create_test_room(room_type: RoomType) -> Room {
        Room::new(
            String::from("Test Room"),
            String::from("A test room"),
            room_type,
            HashMap::new(),
            HashMap::new(),
            HashMap::new(),
            None,
            None,
        )
    }

    // use_item tests

    #[test]
    fn test_use_item_potion() {
        let mut player = create_test_player();
        player.health = 10;
        let mut room = create_test_room(RoomType::Normal);
        let potion = items::create_health_potion();

        let (consumed, _) = player.use_item(&mut room, potion);

        assert!(consumed);
        assert_eq!(player.health, 30); // 10 + 20 healing
    }

    #[test]
    fn test_use_item_small_potion() {
        let mut player = create_test_player();
        player.health = 5;
        let mut room = create_test_room(RoomType::Normal);
        let potion = items::create_small_health_potion();

        let (consumed, _) = player.use_item(&mut room, potion);

        assert!(consumed);
        assert_eq!(player.health, 15); // 5 + 10 healing
    }

    #[test]
    fn test_use_item_poison() {
        let mut player = create_test_player();
        let mut room = create_test_room(RoomType::Normal);
        let poison = items::create_poison_vial();

        let (consumed, _) = player.use_item(&mut room, poison);

        assert!(consumed);
        assert_eq!(player.health, 5); // 20 - 15 damage
    }

    #[test]
    fn test_use_item_poison_saturating() {
        let mut player = create_test_player();
        player.health = 5;
        let mut room = create_test_room(RoomType::Normal);
        let poison = items::create_poison_vial(); // 15 damage

        let (consumed, _) = player.use_item(&mut room, poison);

        assert!(consumed);
        assert_eq!(player.health, 0); // saturating_sub: 5 - 15 = 0
    }

    #[test]
    fn test_use_item_key_in_treasure_room() {
        let mut player = create_test_player();
        let mut room = create_test_room(RoomType::TreasureRoom);
        let key = items::create_brass_key();

        assert_eq!(room.items.len(), 0);

        let (consumed, _) = player.use_item(&mut room, key);

        assert!(!consumed); // Keys are not consumed
        assert_eq!(room.items.len(), 1); // Gold added to room
        assert_eq!(room.items[0].name, "Bag of Gold");
    }

    #[test]
    fn test_use_item_key_in_normal_room() {
        let mut player = create_test_player();
        let mut room = create_test_room(RoomType::Normal);
        let key = items::create_brass_key();

        let (consumed, _) = player.use_item(&mut room, key);

        assert!(!consumed);
        assert_eq!(room.items.len(), 0); // No gold added
    }

    #[test]
    fn test_use_item_gold() {
        let mut player = create_test_player();
        let mut room = create_test_room(RoomType::Normal);
        let gold = items::create_gold();

        let (consumed, _) = player.use_item(&mut room, gold);

        assert!(consumed);
        assert_eq!(player.health, 20); // Health unchanged
    }

    #[test]
    fn test_use_item_junk() {
        let mut player = create_test_player();
        let mut room = create_test_room(RoomType::Normal);
        let junk = items::create_broken_cup();

        let (consumed, _) = player.use_item(&mut room, junk);

        assert!(!consumed);
        assert_eq!(player.health, 20); // Health unchanged
    }
}
