// rooms.rs - Module for handling all room-related logic
use std::collections::HashMap;

use crate::items;
use crate::items::Item;

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
    pub damage: u32,
    pub drop: Option<Item>
}

impl Monster {
    pub fn new(name: String, description: String, health: u32, damage: u32, drop:Option<Item>) -> Self {
        Monster {
            name,
            description,
            health,
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

        description.push_str(&format!("=== {} ===\n", self.name));
        description.push_str(&format!("{}\n\n", self.description));

        // List items if any
        if !self.items.is_empty() {
            description.push_str("You see the following items:\n");
            for item in &self.items {
                description.push_str(&format!("  - {}\n", item.name));
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
                items.push_str(&format!("  [{}] {}\n", index, item.name));
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
    exits.insert(Direction::West, String::from("treasure_room"));
    exits.insert(Direction::South, String::from("boss_room"));
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
    exits.insert(Direction::South, String::from("normal_room"));
    exits.insert(Direction::East, String::from("dungeon_room"));

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
    exits.insert(Direction::South, String::from("cave_entrance"));
    exits.insert(Direction::North, String::from("treasure_room"));
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
    exits.insert(Direction::North, String::from("dungeon_room"));
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
    Room::new(
        String::from("Freedom!"),
        String::from("You emerge from the dark dungeon into brilliant sunlight. The fresh air fills your lungs as you stand at the entrance, the dragon's lair far behind you. Against all odds, you survived. The adventure is over, but the memories will last forever."),
        RoomType::Normal,
        HashMap::new(),  // No exits - game ends here,
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
}
