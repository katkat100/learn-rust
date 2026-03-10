// rooms.rs - Module for handling all room-related logic
use colored::Colorize;

use std::collections::HashMap;
use std::io::{self, Write};

use crate::items;
use crate::items::Item;

use crate::npc;
use crate::npc::NPC;

/// Defines the different types of rooms in the game

#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq)]
pub enum Direction {
    North,
    South,
    East,
    West,
}

impl Direction {
    pub fn as_str(&self) -> &str {
        match self {
            Direction::North => "north",
            Direction::South => "south",
            Direction::East => "east",
            Direction::West => "west",
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum RoomType {
    Normal,
    TreasureRoom,
    Dungeon,
    BossRoom,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Monster {
    pub name: String,
    pub description: String,
    pub health: u32,
    pub max_health: u32,
    pub damage: u32,
    pub drop: Option<Item>,
}

impl Monster {
    pub fn new(
        name: String,
        description: String,
        health: u32,
        damage: u32,
        drop: Option<Item>,
    ) -> Self {
        Monster {
            name,
            description,
            health,
            max_health: health,
            damage,
            drop,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Room {
    pub name: String,
    pub description: String,
    pub room_type: RoomType,
    pub items: Vec<Item>,
    pub exits: HashMap<Direction, String>,
    pub locked_exits: HashMap<Direction, String>,
    pub secret_exits: HashMap<Direction, (String, String)>,
    pub monster: Option<Monster>,
    pub npc: Option<NPC>,
}

impl Room {
    pub fn new(
        name: String,
        description: String,
        room_type: RoomType,
        exits: HashMap<Direction, String>,
        locked_exits: HashMap<Direction, String>,
        secret_exits: HashMap<Direction, (String, String)>,
        monster: Option<Monster>,
        npc: Option<NPC>,
    ) -> Self {
        Room {
            name,
            description,
            room_type,
            items: Vec::new(), // Start with no items
            exits,
            locked_exits,
            secret_exits,
            monster,
            npc,
        }
    }

    pub fn new_with_items(
        name: String,
        description: String,
        room_type: RoomType,
        items: Vec<Item>,
        exits: HashMap<Direction, String>,
        locked_exits: HashMap<Direction, String>,
        secret_exits: HashMap<Direction, (String, String)>,
        monster: Option<Monster>,
        npc: Option<NPC>,
    ) -> Self {
        Room {
            name,
            description,
            room_type,
            items,
            exits,
            locked_exits,
            secret_exits,
            monster,
            npc,
        }
    }

    /// Describe room
    pub fn describe(&self) -> String {
        let mut description = String::new();

        description.push_str(&format!("=== {} ===\n", self.name).bright_cyan().to_string());
        description.push_str(&format!("{}\n\n", self.description));

        if let Some(_npc) = &self.npc {
            description.push_str(&format!(
                "There's someone in the room waiting to talk to you.\n\n"
            ));
        }

        // List items if any
        if !self.items.is_empty() {
            description.push_str("You see the following items:\n");
            for item in &self.items {
                description.push_str(&format!("  - {}\n", item.name_colored));
            }
            description.push('\n');
        }

        // List exits
        if !self.exits.is_empty() {
            description.push_str("Exits: ");
            let exit_names: Vec<&str> = self.exits.keys().map(|d| d.as_str()).collect();
            description.push_str(&exit_names.join(", "));
            description.push('\n');
        }

        description
    }

    /// Get exits
    pub fn get_exit(&self, direction: &Direction) -> Option<&String> {
        self.exits.get(direction)
    }

    /// List exits of the room
    pub fn list_exits(&self) -> String {
        if self.exits.is_empty() {
            String::from("There are no visible exits")
        } else {
            let mut directions = String::from("Available exits:\n");
            for (index, direction) in self.exits.keys().enumerate() {
                directions.push_str(&format!("  [{}] Go {}\n", index, direction.as_str()));
            }

            directions
        }
    }

    /// List items in the room
    pub fn list_items(&self) -> String {
        if self.items.is_empty() {
            String::from("There are no items in this room")
        } else {
            let mut items = String::from("Available items:\n");
            for (index, item) in self.items.iter().enumerate() {
                items.push_str(&format!("  [{}] {}\n", index, item.name_colored));
            }

            items
        }
    }

    /// Take an item from the room
    pub fn take_item(&mut self, item_name: &str) -> Option<Item> {
        if let Some(index) = self.items.iter().position(|item| item.name == item_name) {
            Some(self.items.remove(index))
        } else {
            None
        }
    }

    /// Add an item to the room
    pub fn add_item(&mut self, item: Item) {
        self.items.push(item);
    }

    /// Get direction choice of the room
    pub fn get_direction_choice(&self) -> Option<Direction> {
        if self.exits.is_empty() {
            return None;
        }

        println!("{}", self.list_exits());
        println!("  [{}] Stay here\n", self.exits.len());

        // Convert HashMap keys to a Vec so we can index them
        let directions: Vec<&Direction> = self.exits.keys().collect();

        loop {
            print!("Choose a direction: ");
            io::stdout().flush().unwrap();

            let mut input = String::new();
            io::stdin().read_line(&mut input).ok()?;

            match input.trim().parse::<usize>() {
                Ok(num) if num < directions.len() => {
                    return Some(directions[num].clone()); // Return the Direction, not the index
                }
                Ok(num) if num == directions.len() => {
                    return None; // Stay here
                }
                _ => println!("Invalid choice. Try again."),
            }
        }
    }

    /// Get the item choice of the room
    pub fn get_item_choice(&self) -> Option<Item> {
        if self.items.is_empty() {
            return None;
        }

        println!("{}", self.list_items());
        println!("  [{}] Leave items\n", self.items.len());

        loop {
            println!("Choose an item to take");
            io::stdout().flush().unwrap();

            let mut input = String::new();
            io::stdin().read_line(&mut input).ok()?;
            match input.trim().parse::<usize>() {
                Ok(num) if num < self.items.len() => {
                    return Some(self.items[num].clone());
                }
                Ok(num) if num == self.items.len() => {
                    return None;
                }
                _ => println!("Invalid choice. Try again."),
            }
        }
    }

    // trigger trap
    // fight
}

/// Create a Dragon
pub fn create_dragon() -> Monster {
    Monster::new(
        String::from("Dragon"),
        String::from(
            "A massive dragon with shimmering scales and eyes that glow like molten fire. Smoke curls from its nostrils as it watches you with ancient intelligence.",
        ),
        25,
        5,
        Some(items::create_gold()),
    )
}

/// Create an Ogre
pub fn create_ogre() -> Monster {
    Monster::new(
        String::from("Ogre"),
        String::from(
            "A massive ogre with a thick beard and a belly that could hold a small village.",
        ),
        30,
        7,
        Some(items::create_gold()),
    )
}

/// Create Skeleton
pub fn create_skeleton() -> Monster {
    Monster::new(
        String::from("Skeleton"),
        String::from("A skeleton with a few knicks and dings to his bones."),
        15,
        3,
        Some(items::create_small_health_potion()),
    )
}

/// Create Three Small Skeletons in a Trench Coat
pub fn create_three_skeletons_trench_coat() -> Monster {
    Monster::new(
        String::from("Three Small Skeletons in a Trench Coat"),
        String::from(
            "A weirdly disproportionatly tall skeleton that looks like he's wearing a trench coat.",
        ),
        25,
        5,
        Some(items::create_small_health_potion()),
    )
}

/// Create a kobold
pub fn create_kobold() -> Monster {
    Monster::new(
        String::from("Kobold"),
        String::from("A small, lizard-like creature with sharp teeth and glowing red eyes."),
        5,
        2,
        Some(items::create_one_gold()),
    )
}

/// Create a goblin
pub fn create_goblin() -> Monster {
    Monster::new(
        String::from("Goblin"),
        String::from(
            "A small, humanoid creature with a mischievous grin and a penchant for mischief.",
        ),
        10,
        4,
        Some(items::create_rusty_sword()),
    )
}

/// Create a giant spider
pub fn create_giant_spider() -> Monster {
    Monster::new(
        String::from("Giant Spider"),
        String::from(
            "A large spider with a body the size of a human head and legs as thick as your arm.",
        ),
        20,
        6,
        Some(items::create_steel_sword()),
    )
}

/// Create a two headed snake
pub fn create_two_headed_snake() -> Monster {
    Monster::new(
        String::from("Two-Headed Snake"),
        String::from("A snake with two heads, each with its own set of fangs and eyes."),
        15,
        5,
        Some(items::create_iron_sword()),
    )
}

/// Create a Slime
pub fn create_slime() -> Monster {
    Monster::new(
        String::from("Slime"),
        String::from(
            "A jelly like blob wobbles at you. You can see the bones of some cave dwelling rodent in its wibbly wobbly body.",
        ),
        10,
        1,
        Some(items::create_rusty_key()),
    )
}

/// Create a Giant Slime
pub fn create_giant_slime() -> Monster {
    Monster::new(
        String::from("Giant Slime"),
        String::from(
            "A jelly like blob wobbles at you largely, its body is twice as large as a normal slime.",
        ),
        10,
        1,
        Some(items::create_wooden_shield()),
    )
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_room_new() {
        let room = Room::new(
            String::from("Room"),
            String::from("Room description"),
            RoomType::Normal,
            HashMap::new(),
            HashMap::new(),
            HashMap::new(),
            None,
            None,
        );

        assert_eq!(room.name, "Room");
        assert_eq!(room.description, "Room description");
        assert_eq!(room.room_type, RoomType::Normal);
        assert_eq!(room.exits, [].into());
        assert_eq!(room.locked_exits, [].into());
        assert_eq!(room.monster, None);
    }

    #[test]
    fn test_room_new_with_items() {
        use crate::items;
        let room = Room::new_with_items(
            String::from("Room"),
            String::from("Room description"),
            RoomType::Normal,
            vec![items::create_brass_key()],
            HashMap::new(),
            HashMap::new(),
            HashMap::new(),
            None,
            None,
        );

        assert_eq!(room.name, "Room");
        assert_eq!(room.description, "Room description");
        assert_eq!(room.room_type, RoomType::Normal);
        assert_eq!(room.items.len(), 1);
        assert_eq!(room.items[0].name, "Brass Key");
        assert_eq!(room.exits, [].into());
        assert_eq!(room.locked_exits, [].into());
        assert_eq!(room.monster, None);
    }

    #[test]
    fn test_room_describe() {
        let mut exits = HashMap::new();
        let locked_exits = HashMap::new();
        exits.insert(Direction::North, String::from("normal_room"));
        let room = Room::new_with_items(
            String::from("Start"),
            String::from(
                "You find yourself in a dark, damp cave. Water drips from the stalactites above, echoing through the chamber. The air is cold and musty. You can barely make out the rough stone walls in the dim light filtering from somewhere above.",
            ),
            RoomType::Normal,
            vec![
                items::create_broken_cup(),
                items::create_rusty_spoon(),
                items::create_poison_vial(),
            ],
            exits,
            locked_exits,
            HashMap::new(),
            None,
            Some(npc::create_npc_cleaner()),
        );
        let room_describe = room.describe();

        assert!(room_describe.contains("=== Start ==="));
        assert!(room_describe.contains("You find yourself in a dark, damp cave."));
        assert!(room_describe.contains("Broken Cup"));
        assert!(room_describe.contains("Rusty Spoon"));
        assert!(room_describe.contains("Vial of Poison"));
        assert!(room_describe.contains("Exits: north"));
    }

    #[test]
    fn test_room_get_exit() {
        let mut exits = HashMap::new();
        exits.insert(Direction::North, String::from("normal_room"));
        let room = Room::new(
            String::from("Room"),
            String::from("Room description"),
            RoomType::Normal,
            exits,
            HashMap::new(),
            HashMap::new(),
            None,
            None,
        );
        let room_exit = room.get_exit(&Direction::North);

        assert_eq!(room_exit, Some(&String::from("normal_room")));
    }

    #[test]
    fn test_room_list_exits() {
        let mut exits = HashMap::new();
        exits.insert(Direction::North, String::from("normal_room"));
        exits.insert(Direction::South, String::from("treasure_room"));
        let room = Room::new(
            String::from("Room"),
            String::from("Room description"),
            RoomType::Normal,
            exits,
            HashMap::new(),
            HashMap::new(),
            None,
            None,
        );
        let room_exits = room.list_exits();

        assert!(room_exits.contains("Available exits:"));
        assert!(room_exits.contains("Go north"));
        assert!(room_exits.contains("Go south"));
    }

    #[test]
    fn test_room_list_items() {
        use crate::items;
        let room = Room::new_with_items(
            String::from("Room"),
            String::from("Room description"),
            RoomType::Normal,
            vec![items::create_brass_key(), items::create_health_potion()],
            HashMap::new(),
            HashMap::new(),
            HashMap::new(),
            None,
            None,
        );
        let list = room.list_items();

        assert!(list.contains("Available items:"));
        assert!(list.contains("Brass Key"));
        assert!(list.contains("Health Potion"));
    }

    #[test]
    fn test_room_list_items_empty() {
        let room = Room::new(
            String::from("Room"),
            String::from("Room description"),
            RoomType::Normal,
            HashMap::new(),
            HashMap::new(),
            HashMap::new(),
            None,
            None,
        );

        assert_eq!(room.list_items(), "There are no items in this room");
    }

    #[test]
    fn test_room_take_item() {
        use crate::items;
        let mut room = Room::new_with_items(
            String::from("Room"),
            String::from("Room description"),
            RoomType::Normal,
            vec![items::create_brass_key(), items::create_health_potion()],
            HashMap::new(),
            HashMap::new(),
            HashMap::new(),
            None,
            None,
        );

        let taken = room.take_item("Brass Key");
        assert!(taken.is_some());
        assert_eq!(taken.unwrap().name, "Brass Key");
        assert_eq!(room.items.len(), 1);
        assert_eq!(room.items[0].name, "Health Potion");
    }

    #[test]
    fn test_room_take_item_not_found() {
        use crate::items;
        let mut room = Room::new_with_items(
            String::from("Room"),
            String::from("Room description"),
            RoomType::Normal,
            vec![items::create_brass_key()],
            HashMap::new(),
            HashMap::new(),
            HashMap::new(),
            None,
            None,
        );

        let taken = room.take_item("Nonexistent Item");
        assert!(taken.is_none());
        assert_eq!(room.items.len(), 1);
    }

    #[test]
    fn test_room_add_item() {
        use crate::items;
        let mut room = Room::new(
            String::from("Room"),
            String::from("Room description"),
            RoomType::Normal,
            HashMap::new(),
            HashMap::new(),
            HashMap::new(),
            None,
            None,
        );

        assert_eq!(room.items.len(), 0);
        room.add_item(items::create_health_potion());
        assert_eq!(room.items.len(), 1);
        assert_eq!(room.items[0].name, "Health Potion");
    }

    #[test]
    fn test_direction_as_str() {
        assert_eq!(Direction::North.as_str(), "north");
        assert_eq!(Direction::South.as_str(), "south");
        assert_eq!(Direction::East.as_str(), "east");
        assert_eq!(Direction::West.as_str(), "west");
    }

    #[test]
    fn test_monster_new() {
        use crate::items;
        let monster = Monster::new(
            String::from("Goblin"),
            String::from("A small goblin"),
            10,
            2,
            Some(items::create_gold()),
        );

        assert_eq!(monster.name, "Goblin");
        assert_eq!(monster.description, "A small goblin");
        assert_eq!(monster.health, 10);
        assert_eq!(monster.damage, 2);
        assert!(monster.drop.is_some());
        assert_eq!(monster.drop.unwrap().name, "Bag of Gold");
    }

    #[test]
    fn test_monster_new_no_drop() {
        let monster = Monster::new(String::from("Rat"), String::from("A small rat"), 5, 1, None);

        assert_eq!(monster.name, "Rat");
        assert_eq!(monster.health, 5);
        assert_eq!(monster.damage, 1);
        assert!(monster.drop.is_none());
    }

    #[test]
    fn test_room_describe_no_items() {
        let mut exits = HashMap::new();
        exits.insert(Direction::North, String::from("next_room"));
        let room = Room::new(
            String::from("Empty Room"),
            String::from("Nothing here"),
            RoomType::Normal,
            exits,
            HashMap::new(),
            HashMap::new(),
            None,
            None,
        );
        let desc = room.describe();

        assert!(desc.contains("=== Empty Room ==="));
        assert!(desc.contains("Nothing here"));
        assert!(!desc.contains("You see the following items"));
        assert!(desc.contains("Exits: north"));
    }

    #[test]
    fn test_room_describe_no_exits() {
        let room = Room::new(
            String::from("Dead End"),
            String::from("A dead end"),
            RoomType::Normal,
            HashMap::new(),
            HashMap::new(),
            HashMap::new(),
            None,
            None,
        );
        let desc = room.describe();

        assert!(desc.contains("=== Dead End ==="));
        assert!(!desc.contains("Exits:"));
    }

    #[test]
    fn test_room_get_exit_nonexistent() {
        let mut exits = HashMap::new();
        exits.insert(Direction::North, String::from("next_room"));
        let room = Room::new(
            String::from("Room"),
            String::from("Room description"),
            RoomType::Normal,
            exits,
            HashMap::new(),
            HashMap::new(),
            None,
            None,
        );

        assert!(room.get_exit(&Direction::South).is_none());
        assert!(room.get_exit(&Direction::East).is_none());
        assert!(room.get_exit(&Direction::West).is_none());
    }

    #[test]
    fn test_room_list_exits_empty() {
        let room = Room::new(
            String::from("Room"),
            String::from("Room description"),
            RoomType::Normal,
            HashMap::new(),
            HashMap::new(),
            HashMap::new(),
            None,
            None,
        );

        assert_eq!(room.list_exits(), "There are no visible exits");
    }

    // Monster creation tests

    #[test]
    fn test_create_dragon() {
        let dragon = create_dragon();

        assert_eq!(dragon.name, "Dragon");
        assert_eq!(dragon.health, 25);
        assert_eq!(dragon.damage, 5);
        assert!(dragon.drop.is_some());
        assert_eq!(dragon.drop.unwrap().name, "Bag of Gold");
    }

    #[test]
    fn test_create_skeleton() {
        let skeleton = create_skeleton();

        assert_eq!(skeleton.name, "Skeleton");
        assert_eq!(skeleton.health, 15);
        assert_eq!(skeleton.damage, 3);
        assert!(skeleton.drop.is_some());
        assert_eq!(skeleton.drop.unwrap().name, "Small Health Potion");
    }

    #[test]
    fn test_create_slime() {
        let slime = create_slime();

        assert_eq!(slime.name, "Slime");
        assert_eq!(slime.health, 10);
        assert_eq!(slime.damage, 1);
        assert!(slime.drop.is_some());
        assert_eq!(slime.drop.unwrap().name, "Rusty Key");
    }
}
