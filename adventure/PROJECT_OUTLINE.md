# Text-Based Adventure Game - Project Outline

A comprehensive guide for building a text-based adventure game in Rust as a learning project.

---

## **Phase 1: Foundation & Core Game Loop** ⭐ *Start Here*

**Learning Goals:** Basic I/O, control flow, error handling, functions

### Tasks:
1. **Set up the game loop**
   - Create a main loop that continues until the game ends
   - Learn about `loop`, `while`, and `break` keywords
   - Practice reading user input repeatedly

2. **Implement basic commands**
   - Parse user input (like "look", "help", "quit")
   - Use `match` expressions for command handling
   - Learn about string methods: `.trim()`, `.to_lowercase()`, `.split()`

3. **Create a help system**
   - Display available commands to the player
   - Practice formatting output with `println!`

### Key Rust Concepts:
- Ownership and borrowing with strings
- Result and Option types
- Pattern matching with `match`
- Basic error handling with `?` operator

### Success Criteria:
- [x] Game loop runs continuously
- [x] User can enter commands
- [x] At least 4 commands work: help, look, inventory, quit (needs inventory command + syntax fix)
- [x] Invalid commands show helpful error messages

---

## **Phase 2: Game State & Data Structures**

**Learning Goals:** Structs, enums, Vec, HashMap

### Tasks:
1. **Create a Player struct**
   - Store player data: name, health, inventory
   - Learn about struct definition and methods (`impl` blocks)
   - Practice the `&self` and `&mut self` patterns

2. **Create an Inventory system**
   - Use `Vec<String>` or `Vec<Item>` to store items
   - Implement add/remove/display methods
   - Learn about vectors and their methods

3. **Define Rooms/Locations**
   - Create a `Room` struct with description, items, exits
   - Use an enum for different room types or directions
   - Practice working with multiple structs together

### Key Rust Concepts:
- Struct methods and associated functions
- Enums and pattern matching
- Collections: Vec, HashMap
- Ownership with complex data structures
- `impl` blocks for organizing methods

### Success Criteria:
- [x] Player struct exists with at least 3 fields
- [x] Inventory can add, remove, and list items
- [x] Room struct defined with description and exits
- [x] Player has health or similar stat

---

## **Phase 3: Game World & Navigation**

**Learning Goals:** HashMap, References, lifetimes (basic)

### Tasks:
1. **Create a world map**
   - Use `HashMap<String, Room>` to store all rooms
   - Implement room connections (exits to other rooms)
   - Track current location

2. **Navigation system**
   - Commands: "go north", "go south", "go east", "go west"
   - Validate moves (can player go that direction?)
   - Update player position

3. **Room descriptions**
   - Display room name, description, visible items, available exits
   - Make it engaging with good descriptive text
   - Show what changed since last visit (optional)

### Key Rust Concepts:
- HashMap usage and ownership
- References and borrowing (`&` and `&mut`)
- Option type for handling exits that may not exist
- String vs &str
- Cloning vs borrowing

### Success Criteria:
- [x] At least 5 interconnected rooms exist
- [x] Player can navigate between rooms
- [x] Each room has a unique description
- [x] Invalid directions show appropriate messages
- [x] Current room is tracked properly

---

## **Phase 4: Items & Interactions**

**Learning Goals:** More enums, trait basics, logic building

### Tasks:
1. **Item system**
   - Create an `Item` struct (name, description, usable, etc.)
   - Place items in rooms
   - Implement "take" and "drop" commands

2. **Item interactions**
   - "use" command for items
   - Items that affect player state (potions heal, keys unlock)
   - Items that unlock new areas or options

3. **Examine system**
   - "examine" or "look at" commands for detailed descriptions
   - Learn about storing and managing more complex state
   - Hidden details revealed on examination

### Key Rust Concepts:
- Enums with data (e.g., `ItemType::Key(String)`)
- More complex ownership patterns
- Method chaining
- Moving items between collections
- Conditional logic based on inventory

### Success Criteria:
- [x] Items can be picked up from rooms
- [x] Items can be dropped in rooms
- [x] Inventory shows what player carries
- [x] At least one usable item (potion, key, etc.)
- [x] Examine command provides details
- [x] Can't pick up items that don't exist

---

## **Phase 5: Game Logic & Challenges**

**Learning Goals:** Logic, state management, modularity

### Tasks:
1. **Combat or puzzle system**
   - Simple combat with health/damage
   - OR logic puzzles to solve
   - Random number generation (use `rand` crate)

2. **Win/lose conditions**
   - Track objectives
   - Implement game over states
   - Victory conditions and ending

3. **Save/Load system** (Optional, Advanced)
   - Serialize game state
   - Use `serde` crate for JSON serialization
   - File I/O with `std::fs`

### Key Rust Concepts:
- External crates (Cargo dependencies)
- Random number generation
- State machines for game flow
- Serialization/Deserialization (optional)
- File I/O and error handling (optional)

### Success Criteria:
- [x] Game has a clear objective
- [x] Player can win or lose
- [x] At least one challenge/puzzle/enemy
- [x] Game state affects available options
- [x] Proper game over messages

---

## **Phase 6: Code Organization & Polish**

**Learning Goals:** Modules, documentation, refactoring

### Tasks:
1. **Split into modules**
   - Create separate files: `player.rs`, `room.rs`, `item.rs`, `game.rs`
   - Learn about `mod` keyword and module system
   - Practice public vs private functions (`pub` keyword)

2. **Documentation**
   - Add doc comments (`///`)
   - Write helpful descriptions for your code
   - Run `cargo doc --open` to see your docs
   - Add examples in doc comments

3. **Error handling improvements**
   - Create custom error types
   - Better error messages for players
   - Use `Result` and `?` operator throughout
   - Handle edge cases gracefully

### Key Rust Concepts:
- Module system and visibility
- Documentation comments
- Custom error types and the `Error` trait
- Code organization best practices
- `pub use` for re-exports

### Success Criteria:
- [x] Code split into at least 3 modules
- [x] Each module has clear responsibility
- [x] Functions have doc comments
- [x] `cargo clippy` runs without warnings
- [x] `cargo doc` generates readable docs

---

## **Phase 7: Testing & Quality Assurance**

**Learning Goals:** Unit testing, integration testing, test-driven development

### Tasks:
1. **Item System Tests**
   - Test item creation functions
   - Test item describe() method
   - Test ItemType equality and cloning

2. **Room System Tests**
   - Test room creation with/without items
   - Test adding/removing items from rooms
   - Test locked_exits functionality
   - Test monster presence in rooms

3. **Combat System Tests**
   - Test player taking damage
   - Test monster taking damage
   - Test death conditions (player and monster)
   - Test combat item usage (healing during battle)

4. **Inventory Tests**
   - Test adding items to inventory
   - Test removing items from inventory
   - Test inventory filtering (usable vs junk)

5. **Navigation Tests**
   - Test valid room transitions
   - Test invalid direction handling
   - Test locked door behavior
   - Test previous_room tracking for fleeing

### Key Rust Concepts:
- `#[test]` attribute and test modules
- `assert_eq!` and `assert!` macros
- `#[should_panic]` for error testing
- Integration tests in `tests/` directory
- Test organization with `#[cfg(test)]`

### Success Criteria:
- [x] All item creation functions have tests
- [x] Room manipulation functions tested
- [x] Combat logic tested (damage, death, healing)
- [x] Inventory operations tested
- [x] Navigation and locked doors tested
- [x] `cargo test` passes with >80% coverage
- [x] Edge cases handled (empty inventory, dead monsters, etc.)

---

## **Phase 8: Terminal Colors & UI Enhancement** (Optional)

**Learning Goals:** External crates, visual enhancement, user experience

### Tasks:
1. **Add colored crate**
   - Add dependency to Cargo.toml
   - Color health values (green=high, yellow=medium, red=low)
   - Color item types differently (potions=green, keys=yellow, etc.)
   - Color monster names and damage in red

2. **Improve UI**
   - Add borders around important messages
   - Color room names and descriptions
   - Highlight available commands
   - Make combat messages more dramatic with colors

3. **Status display**
   - Add colored health bar
   - Show inventory count with colors
   - Display current room name prominently

4. **Visited rooms map**
   - Track which rooms the player has visited using a `HashSet<String>`
   - Add a "map" command that displays visited rooms and connections
   - Show the player's current room highlighted on the map
   - Mark unvisited but known exits (rooms you haven't entered yet)
   - Use colors to distinguish current room, visited rooms, and unexplored exits

### Key Rust Concepts:
- External crate usage
- String formatting with colors
- ANSI color codes
- HashSet for tracking visited state
- Building text-based visual layouts from data structures

### Success Criteria:
- [x] `colored` crate added to project
- [x] Health displayed with color coding
- [x] Items colored by type
- [x] Combat messages more visually engaging
- [x] Room transitions clearly visible
- [x] "map" command shows visited rooms and connections
- [x] Current room highlighted on map
- [x] Unvisited exits shown as unexplored

---

## **Phase 9: NPC System & Dialogue** (Optional)

**Learning Goals:** Complex state management, dialogue trees, character interaction

### Tasks:
1. **Create NPC struct**
   - Name, description, dialogue options
   - Inventory for trading
   - Quest flags and states

2. **Dialogue system**
   - "talk" command to initiate dialogue
   - Multiple dialogue options with branching
   - NPCs remember previous conversations
   - Quest giving and completion

3. **Add NPCs to world**
   - Friendly merchant who trades items
   - Quest giver who provides objectives
   - Mysterious stranger with hints
   - Imprisoned ally who needs rescue

### Key Rust Concepts:
- State machines for dialogue
- Complex enum patterns
- Reference counting with Rc/RefCell (if needed)
- Pattern matching for dialogue trees

### Success Criteria:
- [ ] NPC struct with dialogue options
- [ ] "talk" command implemented
- [ ] At least 2 NPCs placed in world
- [ ] Dialogue branches based on player choices
- [ ] NPC can give/complete quests
- [ ] Trading system for items

---

## **Phase 10: Content Expansion** (Optional)

**Learning Goals:** Game design, balancing, content creation

### Tasks:
1. **More Items**
   - Add 5+ new potions with different effects
   - Add 3+ new keys for different doors
   - Add equipment system (weapons, armor)
   - Add consumables (food, scrolls, etc.)

2. **More Monsters**
   - Add 5+ new enemy types
   - Different monster behaviors (aggressive, defensive, fleeing)
   - Mini-bosses in special rooms
   - Monster drops and loot tables

3. **More Rooms**
   - Expand to 15+ rooms
   - Add different biomes (forest, swamp, castle, etc.)
   - Secret rooms with valuable loot
   - Puzzle rooms requiring items to solve

4. **Equipment System**
   - Weapons increase damage
   - Armor reduces damage taken
   - "equip" and "unequip" commands
   - Show equipped items in status

### Success Criteria:
- [ ] 10+ new items added
- [ ] 8+ new monsters added
- [ ] World expanded to 15+ rooms
- [ ] Equipment system functional
- [ ] Loot drops from monsters
- [ ] Secret areas discoverable

---

## **Phase 11: Procedural Generation** (Optional, Advanced)

**Learning Goals:** Algorithms, randomization, procedural content

### Tasks:
1. **Random room generation**
   - Use `rand` crate to generate rooms
   - Ensure all rooms are connected
   - Randomize room descriptions and types

2. **Random item placement**
   - Place items randomly in rooms
   - Balance loot distribution
   - Ensure key items are obtainable

3. **Random monster spawns**
   - Spawn monsters based on difficulty
   - Balance encounter difficulty
   - Boss placement logic

4. **Seed system**
   - Allow players to input seed for reproducible worlds
   - Save/display current world seed

### Key Rust Concepts:
- `rand` crate advanced usage
- Graph algorithms (for room connectivity)
- Seeded random number generation
- Algorithm design

### Success Criteria:
- [ ] Rooms generated procedurally
- [ ] Items placed randomly but fairly
- [ ] Monsters spawn with appropriate difficulty
- [ ] All areas remain accessible
- [ ] Seed system for reproducibility
- [ ] No dead ends or impossible situations

---

## **Phase 12: Save/Load System** (Optional, Advanced)

**Learning Goals:** Serialization, file I/O, persistence

### Tasks:
1. **Add serde dependencies**
   - Add `serde` and `serde_json` to Cargo.toml
   - Derive Serialize/Deserialize on structs

2. **Save system**
   - "save" command to write game state to file
   - Save player stats, inventory, current room
   - Save world state (monsters defeated, items taken)
   - Save to JSON file in user directory

3. **Load system**
   - "load" command to restore game state
   - Handle missing or corrupted save files
   - Allow multiple save slots

4. **Auto-save**
   - Auto-save on room transitions
   - Auto-save after combat
   - Restore from auto-save on crash

### Key Rust Concepts:
- Serialization with serde
- File I/O with std::fs
- Error handling for I/O operations
- JSON parsing and generation

### Success Criteria:
- [ ] Game state serializable
- [ ] Save command writes to file
- [ ] Load command restores state
- [ ] Multiple save slots supported
- [ ] Corrupted saves handled gracefully
- [ ] Auto-save functionality

---

## **Current Project Status**

### Completed:
- [x] Basic project setup
- [x] Player name input
- [x] Started creating game world
- [x] Basic error handling structure
- [x] Phase 1: Foundation & Core Game Loop ✅
- [x] Phase 2: Game State & Data Structures ✅
- [x] Phase 3: Game World & Navigation ✅
- [x] Phase 4: Items & Interactions ✅
- [x] Phase 5: Game Logic & Challenges ✅
- [x] Phase 6: Code Organization & Polish ✅

### Current Phase: **Phase 7 - Testing & Quality Assurance**

### Immediate Next Steps:
1. Create test module in items.rs for item tests
2. Create test module in rooms.rs for room/monster tests
3. Write tests for combat damage calculations
4. Write tests for inventory operations
5. Write integration tests for game flow
6. Achieve >80% test coverage with `cargo test`

---

## **Learning Resources**

### Rust Concepts to Study:
- **The Rust Book**: https://doc.rust-lang.org/book/
  - Chapter 3: Common Programming Concepts
  - Chapter 4: Ownership
  - Chapter 6: Enums and Pattern Matching
  - Chapter 8: Collections
  - Chapter 10: Generics, Traits, and Lifetimes

### Useful Crates:
- `rand`: Random number generation
- `serde` + `serde_json`: Serialization for save/load
- `colored`: Terminal colors
- `rustyline`: Better command line input (optional)

---

## **Tips for Success**

1. **Start small**: Don't try to implement everything at once
2. **Test frequently**: Run `cargo run` often to catch errors early
3. **Read compiler messages**: Rust's error messages are very helpful
4. **Use `cargo clippy`**: Get suggestions for better Rust code
5. **Commit often**: Use git to save progress between phases
6. **Ask questions**: When stuck, try to understand *why* the code works
7. **Refactor**: Come back and improve code as you learn more
8. **Have fun**: Make the game interesting to you!

---

## **Common Beginner Mistakes to Avoid**

1. **Fighting the borrow checker**: If borrowing is confusing, try cloning first (`.clone()`), then optimize later
2. **Not using enums**: Enums are powerful in Rust - use them!
3. **Ignoring warnings**: `cargo clippy` and compiler warnings help you learn
4. **Overcomplicating**: Start simple, add complexity gradually
5. **Not reading errors**: Rust errors tell you exactly what's wrong
6. **Avoiding documentation**: Writing docs helps you understand your own code

---

## **Victory Conditions for This Project**

You'll know you've succeeded when:
- [ ] Game runs without panicking
- [ ] Player can navigate multiple rooms
- [ ] Inventory system works
- [ ] At least one puzzle or challenge exists
- [ ] Game has a clear beginning and end
- [ ] Code is organized in modules
- [ ] You understand what each line does
- [ ] You can explain ownership in your code
- [ ] You're proud to show someone your game!

---

**Remember**: The goal isn't just to finish the game, but to understand *how* and *why* the Rust code works. Take your time, experiment, and learn from mistakes. Happy coding! 🦀


Item options
``` // Create items like this:
let health_potion = Item {
    name: String::from("Health Potion"),
    description: String::from("A red potion that restores 20 health"),
    item_type: ItemType::Potion { healing: 20 },
};

let small_poison = Item {
    name: String::from("Vial of Poison"),
    description: String::from("A dark liquid that causes 15 damage"),
    item_type: ItemType::Poison { damage: 15 },
};

let brass_key = Item {
    name: String::from("Brass Key"),
    description: String::from("Opens the treasure chest"),
    item_type: ItemType::Key { unlocks: String::from("treasure_chest") },
};

let broken_cup = Item {
    name: String::from("Broken Cup"),
    description: String::from("A chipped, worthless cup"),
    item_type: ItemType::Junk,
};```
