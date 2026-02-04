// rooms.rs - Module for handling all room-related logic
use std::collections::HashMap;
use colored::Colorize;

use crate::items;
use crate::items::{Item};

/// Defines the different types of rooms in the game

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
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
    pub drop: Option<Item>
}

impl Monster {
    pub fn new(name: String, description: String, health: u32, damage: u32, drop:Option<Item>) -> Self {
        Monster {
            name,
            description,
            health,
            max_health: health,
            damage,
            drop
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
    pub monster: Option<Monster>,
}

impl Room {
    pub fn new(
        name: String,
        description: String,
        room_type: RoomType,
        exits: HashMap<Direction, String>,
        locked_exits: HashMap<Direction, String>,
        monster: Option<Monster>
    ) -> Self {
        Room {
            name,
            description,
            room_type,
            items: Vec::new(),  // Start with no items
            exits,
            locked_exits,
            monster,
        }
    }

    pub fn new_with_items(
        name: String,
        description: String,
        room_type: RoomType,
        items: Vec<Item>,
        exits: HashMap<Direction, String>,
        locked_exits: HashMap<Direction, String>,
        monster: Option<Monster>
    ) -> Self {
        Room {
            name,
            description,
            room_type,
            items,
            exits,
            locked_exits,
            monster,
        }
    }

    pub fn describe(&self) -> String {
        let mut description = String::new();

        description.push_str(&format!("=== {} ===\n", self.name).bright_cyan().to_string());
        description.push_str(&format!("{}\n\n", self.description));

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
            let exit_names: Vec<&str> = self.exits.keys()
                .map(|d| d.as_str())
                .collect();
            description.push_str(&exit_names.join(", "));
            description.push('\n');
        }

        description
    }

    pub fn get_exit(&self, direction: &Direction) -> Option<&String> {
        self.exits.get(direction)
    }

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

    pub fn take_item(&mut self, item_name: &str) -> Option<Item> {
        if let Some(index) = self.items.iter().position(|item| item.name == item_name) {
            Some(self.items.remove(index))
        } else {
            None
        }
    }

    pub fn add_item(&mut self, item: Item) {
        self.items.push(item);
    }

    // trigger trap
    // fight
}

/// Create a Dragon
pub fn create_dragon() -> Monster {
    Monster::new(
        String::from("Dragon"),
        String::from("A massive dragon with shimmering scales and eyes that glow like molten fire. Smoke curls from its nostrils as it watches you with ancient intelligence."),
        25,
        5,
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

/// Create a Slime
pub fn create_slime() -> Monster {
    Monster::new(
        String::from("Slime"),
        String::from("A jelly like blob wobbles at you. You can see the bones of some cave dwelling rodent in its wibbly wobbly body."),
        10,
        1,
        Some(items::create_rusty_key()),
    )
}

/// Creates a entrance cave room (starting room)
pub fn create_cave_entrance() -> Room {
    let mut exits = HashMap::new();
    let locked_exits = HashMap::new();
    exits.insert(Direction::North, String::from("normal_room"));
    Room::new_with_items(
        String::from("Start"),
        String::from("You find yourself in a dark, damp cave. Water drips from the stalactites above, echoing through the chamber. The air is cold and musty. You can barely make out the rough stone walls in the dim light filtering from somewhere above."),
        RoomType::Normal,
        vec![items::create_broken_cup(), items::create_rusty_spoon(), items::create_poison_vial()],
        exits,
        locked_exits,
        None
    )
}

/// Creates a normal cave room
pub fn create_normal_room() -> Room {
    let mut exits = HashMap::new();
    let locked_exits = HashMap::new();
    exits.insert(Direction::South, String::from("cave_entrance"));
    exits.insert(Direction::West, String::from("treasure_room"));
    Room::new(
        String::from("Dark Cave"),
        String::from("A dark, damp cave. Water drips from the stalactites above, echoing through the chamber. The air is cold and musty. You can barely make out the rough stone walls in the dim light filtering from somewhere above."),
        RoomType::Normal,
        exits,
        locked_exits,
        Some(create_slime())
    )
}

/// Creates a treasure room with a chest
pub fn create_treasure_room() -> Room {
    use crate::items;
    let mut exits = HashMap::new();
    let locked_exits = HashMap::new();
    exits.insert(Direction::East, String::from("normal_room"));
    exits.insert(Direction::North, String::from("dungeon_room"));

    Room::new_with_items(
        String::from("Treasure Chamber"),
        String::from("A grand chamber with golden walls. In the center sits an ornate chest, its lock glinting in the torchlight. Ancient symbols are carved into the stone floor, and the air smells of old gold and mystery."),
        RoomType::TreasureRoom,
        vec![items::create_brass_key(), items::create_health_potion()],
        exits,
        locked_exits,
        None
    )
}

/// Creates a dungeon cell
pub fn create_dungeon_room() -> Room {
    let mut exits = HashMap::new();
    let mut locked_exits = HashMap::new();
    exits.insert(Direction::North, String::from("boss_room"));
    exits.insert(Direction::South, String::from("treasure_room"));
    locked_exits.insert(Direction::North, String::from("Rusty Key"));
    Room::new_with_items(
        String::from("Dungeon Cell"),
        String::from("The walls are made of cold, rough stone. Rusted chains hang from the walls, and the floor is covered in filthy straw. A small barred window near the ceiling lets in a sliver of pale moonlight."),
        RoomType::Dungeon,
        vec![items::create_torn_page()],
        exits,
        locked_exits,
        Some(create_skeleton()),

    )
}

/// Creates a boss room
pub fn create_boss_room() -> Room {
    let mut exits = HashMap::new();
    let locked_exits = HashMap::new();
    exits.insert(Direction::South, String::from("dungeon_room"));
    exits.insert(Direction::East, String::from("exit"));
    Room::new(
        String::from("Dragon's Lair"),
        String::from("You enter a vast cavern. The ceiling disappears into darkness high above. Piles of gold and jewels are scattered everywhere, but your eyes are drawn to the massive dragon sleeping atop the largest treasure heap. Its scales shimmer even in the dim light."),
        RoomType::BossRoom,
        exits,
        locked_exits,
        Some(create_dragon()),
    )
}

/// Creates the exit room (victory!)
pub fn create_exit_room() -> Room {
    let mut exits = HashMap::new();
    exits.insert(Direction::West, String::from("boss_room"));
    Room::new(
        String::from("Freedom!"),
        String::from("You emerge from the dark dungeon into brilliant sunlight. The fresh air fills your lungs as you stand at the entrance, the dragon's lair far behind you. Against all odds, you survived. The adventure is over, but the memories will last forever."),
        RoomType::Normal,
        exits,
        HashMap::new(),  // No exits - game ends here,
        None
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
            None
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
            None
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
        let room = create_cave_entrance();
        let room_describe = room.describe();

        assert_eq!(room_describe, "=== Start ===\nYou find yourself in a dark, damp cave. Water drips from the stalactites above, echoing through the chamber. The air is cold and musty. You can barely make out the rough stone walls in the dim light filtering from somewhere above.\n\nYou see the following items:\n  - Broken Cup\n  - Rusty Spoon\n  - Vial of Poison\n\nExits: north\n")
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
            None
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
            None
        );
        let room_exits = room.list_exits();

        assert_eq!(room_exits, "Available exits:\n  [0] Go north\n  [1] Go south\n");
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
            None
        );
        let list = room.list_items();

        assert!(list.contains("Available items:"));
        assert!(list.contains("[0] Brass Key"));
        assert!(list.contains("[1] Health Potion"));
    }

    #[test]
    fn test_room_list_items_empty() {
        let room = Room::new(
            String::from("Room"),
            String::from("Room description"),
            RoomType::Normal,
            HashMap::new(),
            HashMap::new(),
            None
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
            None
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
            None
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
            None
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
        let monster = Monster::new(
            String::from("Rat"),
            String::from("A small rat"),
            5,
            1,
            None,
        );

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
            None
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
            None
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
            None
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
            None
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

    // Room creation tests

    #[test]
    fn test_create_cave_entrance() {
        let room = create_cave_entrance();

        assert_eq!(room.name, "Start");
        assert_eq!(room.room_type, RoomType::Normal);
        assert_eq!(room.items.len(), 3);
        assert_eq!(room.items[0].name, "Broken Cup");
        assert_eq!(room.items[1].name, "Rusty Spoon");
        assert_eq!(room.items[2].name, "Vial of Poison");
        assert!(room.get_exit(&Direction::North).is_some());
        assert!(room.monster.is_none());
        assert!(room.locked_exits.is_empty());
    }

    #[test]
    fn test_create_normal_room() {
        let room = create_normal_room();

        assert_eq!(room.name, "Dark Cave");
        assert_eq!(room.room_type, RoomType::Normal);
        assert!(room.items.is_empty());
        assert!(room.get_exit(&Direction::West).is_some());
        assert!(room.get_exit(&Direction::South).is_some());
        assert!(room.monster.is_some());
        assert_eq!(room.monster.unwrap().name, "Slime");
    }

    #[test]
    fn test_create_treasure_room() {
        let room = create_treasure_room();

        assert_eq!(room.name, "Treasure Chamber");
        assert_eq!(room.room_type, RoomType::TreasureRoom);
        assert_eq!(room.items.len(), 2);
        assert_eq!(room.items[0].name, "Brass Key");
        assert_eq!(room.items[1].name, "Health Potion");
        assert!(room.get_exit(&Direction::South).is_some());
        assert!(room.get_exit(&Direction::East).is_some());
        assert!(room.monster.is_none());
    }

    #[test]
    fn test_create_dungeon_room() {
        let room = create_dungeon_room();

        assert_eq!(room.name, "Dungeon Cell");
        assert_eq!(room.room_type, RoomType::Dungeon);
        assert_eq!(room.items.len(), 1);
        assert_eq!(room.items[0].name, "Torn Page");
        assert!(room.get_exit(&Direction::South).is_some());
        assert!(room.get_exit(&Direction::North).is_some());
        assert!(!room.locked_exits.is_empty());
        assert!(room.monster.is_some());
        assert_eq!(room.monster.unwrap().name, "Skeleton");
    }

    #[test]
    fn test_create_boss_room() {
        let room = create_boss_room();

        assert_eq!(room.name, "Dragon's Lair");
        assert_eq!(room.room_type, RoomType::BossRoom);
        assert!(room.items.is_empty());
        assert!(room.get_exit(&Direction::North).is_some());
        assert!(room.get_exit(&Direction::East).is_some());
        assert!(room.monster.is_some());
        assert_eq!(room.monster.unwrap().name, "Dragon");
    }

    #[test]
    fn test_create_exit_room() {
        let room = create_exit_room();

        assert_eq!(room.name, "Freedom!");
        assert_eq!(room.room_type, RoomType::Normal);
        assert!(room.items.is_empty());
        assert!(room.exits.is_empty());
        assert!(room.locked_exits.is_empty());
        assert!(room.monster.is_none());
    }
}
