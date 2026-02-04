// items.rs - Module for handling all item-related logic
use colored::Colorize;

/// Defines the different types of items in the game
/// Each variant can carry different data specific to that item type
#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub enum ItemType {
    /// A potion that heals the player
    Potion { healing: u32 },

    /// A poison that damages the player
    Poison { damage: u32 },

    /// A key that unlocks specific doors or chests
    Key { unlocks: String },

    /// Junk items that serve no purpose (just take up space)
    Junk,

    Gold,
}

/// Represents an item in the game
#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub struct Item {
    pub name: String,
    pub name_colored: String,
    pub description: String,
    pub item_type: ItemType,
}

impl std::fmt::Display for Item {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{} - {}", self.name, self.description)
    }
}

impl Item {
    /// Creates a new Item
    pub fn new(name: String, description: String, item_type: ItemType) -> Self {
        let name_colored = match item_type {
            ItemType::Key { unlocks: _ } | ItemType::Gold => format!("{}", name.bright_yellow().to_string()),
            ItemType::Poison { damage: _ } => format!("{}", name.red().to_string()),
            ItemType::Potion { healing: _ } => format!("{}", name.green().to_string()),
            // ItemType::Junk => format!("{}", name.white()),
            _ => format!("{} (Gold)", name),
        };
        Item {
            name,
            name_colored,
            description,
            item_type,
        }
    }

    /// Returns a formatted string describing the item
    pub fn describe(&self) -> String {
        format!("{}: {}", self.name_colored, self.description)
    }
}

// Helper functions to create common items
// These make it easier to create items without repeating code

/// Creates a health potion
pub fn create_health_potion() -> Item {
    Item::new(
        String::from("Health Potion"),
        String::from("A red potion that restores 20 health"),
        ItemType::Potion { healing: 20 },
    )
}

/// Creates a small health potion
pub fn create_small_health_potion() -> Item {
    Item::new(
        String::from("Small Health Potion"),
        String::from("A tiny red potion that restores 10 health"),
        ItemType::Potion { healing: 10 },
    )
}

/// Creates a vial of poison
pub fn create_poison_vial() -> Item {
    Item::new(
        String::from("Vial of Poison"),
        String::from("A dark liquid that causes 15 damage if consumed"),
        ItemType::Poison { damage: 15 },
    )
}

/// Creates a brass key
pub fn create_brass_key() -> Item {
    Item::new(
        String::from("Brass Key"),
        String::from("An old brass key. I wonder what it unlocks?"),
        ItemType::Key {
            unlocks: String::from("treasure_chest")
        },
    )
}

/// Creates a rusty key
pub fn create_rusty_key() -> Item {
    Item::new(
        String::from("Rusty Key"),
        String::from("A rusty key covered in grime"),
        ItemType::Key {
            unlocks: String::from("dungeon_door")
        },
    )
}

/// Creates a broken cup (junk item)
pub fn create_broken_cup() -> Item {
    Item::new(
        String::from("Broken Cup"),
        String::from("A chipped, worthless cup. Why would anyone keep this?"),
        ItemType::Junk,
    )
}

/// Creates a rusty spoon (junk item)
pub fn create_rusty_spoon() -> Item {
    Item::new(
        String::from("Rusty Spoon"),
        String::from("A bent, rusty spoon. Completely useless."),
        ItemType::Junk,
    )
}

/// Creates a torn page (junk item)
pub fn create_torn_page() -> Item {
    Item::new(
        String::from("Torn Page"),
        String::from("A torn page from an old book. The text is unreadable."),
        ItemType::Junk,
    )
}

/// Creates a bag of gold (money for later)
pub fn create_gold() -> Item {
    Item::new(
        String::from("Bag of Gold"),
        String::from("A bag filled to the brim with gold!"),
        ItemType::Gold,
    )
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    // Test 1: Item creation
    #[test]
    fn test_item_new() {
        let item = Item::new(
            String::from("Test Item"),
            String::from("A test description"),
            ItemType::Junk,
        );

        assert_eq!(item.name, "Test Item");
        assert_eq!(item.description, "A test description");
        assert_eq!(item.item_type, ItemType::Junk);
    }

    // Test 2: Item describe method
    #[test]
    fn test_item_describe() {
        let item = Item::new(
            String::from("Magic Sword"),
            String::from("A sword that glows"),
            ItemType::Junk,
        );

        assert_eq!(item.describe(), "Magic Sword: A sword that glows");
    }

    // Test 3: Item Display trait
    #[test]
    fn test_item_display() {
        let item = Item::new(
            String::from("Shield"),
            String::from("A sturdy shield"),
            ItemType::Junk,
        );

        assert_eq!(format!("{}", item), "Shield - A sturdy shield");
    }

    // Test 4: Health potion creation
    #[test]
    fn test_create_health_potion() {
        let potion = create_health_potion();

        assert_eq!(potion.name, "Health Potion");
        assert!(potion.description.contains("20 health"));
        assert_eq!(potion.item_type, ItemType::Potion { healing: 20 });
    }

    // Test 5: Small health potion creation
    #[test]
    fn test_create_small_health_potion() {
        let potion = create_small_health_potion();

        assert_eq!(potion.name, "Small Health Potion");
        assert_eq!(potion.item_type, ItemType::Potion { healing: 10 });
    }

    // Test 6: Poison creation
    #[test]
    fn test_create_poison_vial() {
        let poison = create_poison_vial();

        assert_eq!(poison.name, "Vial of Poison");
        assert_eq!(poison.item_type, ItemType::Poison { damage: 15 });
    }

    // Test 7: Brass key creation
    #[test]
    fn test_create_brass_key() {
        let key = create_brass_key();

        assert_eq!(key.name, "Brass Key");
        assert_eq!(key.item_type, ItemType::Key {
            unlocks: String::from("treasure_chest")
        });
    }

    // Test 8: Rusty key creation
    #[test]
    fn test_create_rusty_key() {
        let key = create_rusty_key();

        assert_eq!(key.name, "Rusty Key");
        assert_eq!(key.item_type, ItemType::Key {
            unlocks: String::from("dungeon_door")
        });
    }

    // Test 9: Junk item creation
    #[test]
    fn test_create_junk_items() {
        let cup = create_broken_cup();
        let spoon = create_rusty_spoon();
        let page = create_torn_page();

        assert_eq!(cup.item_type, ItemType::Junk);
        assert_eq!(spoon.item_type, ItemType::Junk);
        assert_eq!(page.item_type, ItemType::Junk);
    }

    // Test 10: Gold creation
    #[test]
    fn test_create_gold() {
        let gold = create_gold();

        assert_eq!(gold.name, "Bag of Gold");
        assert_eq!(gold.item_type, ItemType::Gold);
    }

    // Test 11: Item equality (two identical items)
    #[test]
    fn test_item_equality() {
        let item1 = create_health_potion();
        let item2 = create_health_potion();

        assert_eq!(item1, item2);
    }

    // Test 12: Item inequality (different items)
    #[test]
    fn test_item_inequality() {
        let potion = create_health_potion();
        let poison = create_poison_vial();

        assert_ne!(potion, poison);
    }

    // Test 13: Item cloning
    #[test]
    fn test_item_clone() {
        let original = create_brass_key();
        let cloned = original.clone();

        assert_eq!(original, cloned);
        assert_eq!(original.name, cloned.name);
    }

    // Test 14: ItemType pattern matching
    #[test]
    fn test_item_type_matching() {
        let potion = create_health_potion();

        match potion.item_type {
            ItemType::Potion { healing } => {
                assert_eq!(healing, 20);
            }
            _ => panic!("Expected Potion type!"),
        }
    }

    // Test 15: Different potion values
    #[test]
    fn test_different_potion_values() {
        let big_potion = ItemType::Potion { healing: 50 };
        let small_potion = ItemType::Potion { healing: 10 };

        assert_ne!(big_potion, small_potion);
    }
}
