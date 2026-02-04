use colored::Colorize;

use std::io::{self, Write};
use std::collections::{HashMap, HashSet, VecDeque};

mod items;
use items::{Item, ItemType};

mod rooms;
use rooms::{Room, Direction};
use crate::rooms::RoomType;

struct Player {
    name: String,
    health: u32,
    max_health: u32,
    damage: u32,
    inventory: Vec<Item>
}

const ACTIONS_STR: &str = "(look/move/take/inventory/map/help/quit)";

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("{}", "Welcome to the adventure!".bright_yellow());

    println!("What is your name adventurer?");
    let mut name_str = String::new();
    io::stdin().read_line(&mut name_str)?;

    let mut player = Player {
        name: name_str.trim().to_string(),
        health: 20,
        max_health: 20,
        damage: 5,
        inventory: Vec::new()
    };

    println!("\nHello {}!", player.name);

    let mut world_map:HashMap<String, Room> = HashMap::new();
    let mut visited_rooms: HashSet<String> = HashSet::new();
    world_map.insert(String::from("cave_entrance"), rooms::create_cave_entrance());
    world_map.insert(String::from("dungeon_room"), rooms::create_dungeon_room());
    world_map.insert(String::from("treasure_room"), rooms::create_treasure_room());
    world_map.insert(String::from("normal_room"), rooms::create_normal_room());
    world_map.insert(String::from("boss_room"), rooms::create_boss_room());
    world_map.insert(String::from("exit"), rooms::create_exit_room());
    let mut current_room_id = String::from("cave_entrance");
    visited_rooms.insert(current_room_id.clone());
    let mut previous_room_id = String::from("cave_entrance");

    println!("\n");
    println!("---------");

    loop {
        let monster_info = world_map.get(&current_room_id)
            .and_then(|room| room.monster.as_ref())
            .map(|m| (m.name.clone(), m.health, m.description.clone(), m.drop.clone()));

        if let Some((monster_name, monster_health, monster_description, monster_drop)) = monster_info {
            if monster_health == 0 {
                if let Some(room) = world_map.get_mut(&current_room_id) {
                    room.monster = None;
                    println!("The {} has been defeated!", monster_name);

                    if let Some(drop) = monster_drop {
                        room.add_item(drop);
                    }
                }
                continue;
            }

            println!("\nThere's a {} in this room!\n{}\nWhat will you do?", monster_name.to_string().magenta(), monster_description);
            println!("  [0] Attack");
            println!("  [1] Use Item");
            println!("  [2] Run Away");

            // actions
            let mut action = String::new();
            io::stdin().read_line(&mut action)?;
            match action.trim().parse::<usize>() {
                Ok(0) => {
                    // Player attacks
                    if let Some(room) = world_map.get_mut(&current_room_id)
                        && let Some(monster) = &mut room.monster {
                            monster.health = monster.health.saturating_sub(player.damage);
                            println!("You hit the {} for  {} damage!", monster.name.to_string().magenta(), player.damage.to_string().red());

                            if monster.health > 0 {
                                player.health = player.health.saturating_sub(monster.damage);
                                println!("The {} hits you for {} damage! Health: {}", monster.name.to_string().magenta(), monster.damage.to_string().red(), color_health(player.health, player.max_health));

                                if player.health == 0 {
                                    println!("{}","\n╔════════════════════════════════════╗".red());
                                    println!("{}","║         💀 GAME OVER 💀            ║".red());
                                    println!("{}","╚════════════════════════════════════╝".red());
                                    println!("You were defeated by the {}...", monster.name.magenta());
                                    return Ok(());
                                }
                            }
                        }
                }
                Ok(1) => {
                    println!("Inventory check!");
                    loop{
                        println!("You have multiple usable items");
                        let useable_items: Vec<Item> = player.inventory.iter()
                            .filter(|x| matches!(x.item_type, ItemType::Potion { .. }))
                            .cloned()
                            .collect();
                        for (index, item) in useable_items.iter().enumerate() {
                            println!("[{}] {}", index, item.name);
                        }
                        println!("[{}] Cancel", useable_items.len());

                        let mut input = String::new();
                        io::stdin().read_line(&mut input)?;

                        match input.trim().parse::<usize>() {
                            Ok(num) if num < useable_items.len() => {
                                let item = useable_items[num].clone();
                                if let Some(room) = world_map.get_mut(&current_room_id) {
                                    let item_used = use_item(&mut player, room, item.clone());
                                    if item_used {
                                        let item_name = item.name.clone();
                                        player.inventory.retain(|inv_item| inv_item.name != item_name);
                                    }
                                }
                            }
                            Ok(num) if num == useable_items.len() => {
                                break;
                            }
                            Ok(_) => println!("Invalid choice. Try Again."),
                            Err(_) => println!("Invalid choice. Try Again."),
                        }
                    }
                }
                Ok(2) => {
                    if let Some(room) = world_map.get_mut(&current_room_id)
                        && let Some(monster) = &room.monster {
                            println!("You flee from the {}!", monster.name.to_string().magenta());
                            current_room_id = previous_room_id.clone();
                            println!("\nYou retreat to the {}.", world_map[&current_room_id].name);
                            continue;
                        }
                }
                Ok(_) => println!("Invalid choice. Try Again."),
                Err(_) => println!("Invalid choice. Try Again."),
            }
            continue; // Loop back to check monster status
        }
        println!("\nWhat action would you like to take? {}", &ACTIONS_STR);

        // actions
        let mut action = String::new();
        io::stdin().read_line(&mut action)?;

        match action.trim() {
            "look" => {
                println!("\n{}", world_map[&current_room_id].describe());
                continue;
            }
            "move" => {
                let direction = {
                    let room = &world_map[&current_room_id];
                    get_direction_choice(room)
                };

                if let Some(direction) = direction {
                    if let Some(room) = world_map.get_mut(&current_room_id)
                        && let Some(required_key) = room.locked_exits.get(&direction) {
                            if player.inventory.iter().any(|item| &item.name == required_key) {
                                println!("You use the {} to unlock the door!", required_key);
                                room.locked_exits.remove(&direction);
                            } else {
                                println!("This door is locked. You need a {}.", required_key);
                                return Ok(());
                            }
                        }
                    let next_room_id = world_map[&current_room_id].get_exit(&direction).cloned();
                    if let Some(new_room_id) = next_room_id {
                        previous_room_id = current_room_id.clone();
                        current_room_id = new_room_id;
                        visited_rooms.insert(current_room_id.clone());
                        println!("\nYou go {}...\n", direction.as_str());

                        // Check if reached exit
                        if current_room_id == "exit" {
                            println!("{}", world_map[&current_room_id].describe());
                            println!("{}","\n╔════════════════════════════════════╗".yellow());
                            println!("{}","║      🏆 VICTORY ACHIEVED! 🏆       ║".yellow());
                            println!("{}","╚════════════════════════════════════╝".yellow());
                            println!();
                            println!("Your adventure statistics:");
                            println!("  - Health remaining: {} HP", player.health);
                            println!("  - Items collected: {}", player.inventory.len());
                            println!();
                            println!("Thank you for playing, {}!", player.name);
                            println!("The dragon's treasure is yours...");
                            break;
                        }

                        println!("{}", world_map[&current_room_id].describe());
                    } else {
                        println!("You can't go that way!");
                    }
                } else {
                    println!("You decide to stay put.");
                }
                continue;
            }
            "examine" => {
                loop {
                    println!("Which item in the room do you want to examine?");
                    for (index, item) in world_map[&current_room_id].items.iter().enumerate() {
                        println!("  [{}] {}", index, item.describe());
                    }
                    println!("  [{}] Back", &world_map[&current_room_id].items.len());

                    let mut input = String::new();
                    io::stdin().read_line(&mut input)?;

                    match input.trim().parse::<usize>() {
                        Ok(num) if num < world_map[&current_room_id].items.len() => {
                            let item = &world_map[&current_room_id].items[num];
                            println!("{}: {}", item.name, item.description);
                        }
                        Ok(num) if num == world_map[&current_room_id].items.len() => {
                            break;
                        }
                        _ => println!("Invalid choice. Try again.")
                    }
                }
            }
            "use" => {
                loop{
                    println!("You have multiple usable items");
                    let useable_items: Vec<Item> = player.inventory.iter()
                        .filter(|x| x.item_type != ItemType::Junk && x.item_type != ItemType::Gold)
                        .cloned()
                        .collect();
                    for (index, item) in useable_items.iter().enumerate() {
                        println!("[{}] {}", index, item.name);
                    }
                    println!("[{}] Cancel", useable_items.len());

                    let mut input = String::new();
                    io::stdin().read_line(&mut input)?;

                    match input.trim().parse::<usize>() {
                        Ok(num) if num < useable_items.len() => {
                            let item = useable_items[num].clone();
                            if let Some(room) = world_map.get_mut(&current_room_id) {
                                let item_used = use_item(&mut player, room, item.clone());
                                if item_used {
                                    let item_name = item.name.clone();
                                    player.inventory.retain(|inv_item| inv_item.name != item_name);
                                }
                            }
                        }
                        Ok(num) if num == useable_items.len() => {
                            break;
                        }
                        _ => println!("Invalid choice. Try Again."),
                    }
                }
            }
            "take" => {
                // First, figure out what item to take (immutable borrow)
                let item_name = {
                    let current_room = &world_map[&current_room_id];
                    if let Some(item) = get_item_choice(current_room) {
                        Some(item.name.clone())
                    } else {
                        None
                    }
                }; // Immutable borrow ends here

                // Then take it (mutable borrow)
                if let Some(name) = item_name
                    && let Some(room) = world_map.get_mut(&current_room_id)
                        && let Some(taken_item) = room.take_item(&name) {
                            println!("Added {} to your inventory", &taken_item.name);
                            player.inventory.push(taken_item);
                        }

                continue;
            }
            "inventory" | "inv" | "i" => {
                if player.inventory.is_empty() {
                    println!("Your inventory is empty.");
                } else {
                    loop {
                        println!("\n=== Your Inventory ===");
                        for (index, item) in player.inventory.iter().enumerate() {
                            println!("  [{}] {}", index, item.describe());
                        }
                        println!("  [{}] Close Inventory", player.inventory.len());

                        let mut input = String::new();
                        io::stdin().read_line(&mut input)?;


                        match input.trim().parse::<usize>() {
                            Ok(num) if num < player.inventory.len() => {
                                let (item_display, is_junk) = {
                                    let item = &player.inventory[num];
                                    (item.to_string(), item.item_type == ItemType::Junk)
                                };

                                println!("What would you like to do with the {}", item_display);
                                println!("  [0] Drop");
                                println!("  [1] Examine");
                                if is_junk {
                                    println!("  [2] Do Nothing");
                                } else {
                                    println!("  [2] Use");
                                    println!("  [3] Do Nothing");
                                }

                                let mut inv_input = String::new();
                                io::stdin().read_line(&mut inv_input)?;

                                match inv_input.trim().parse::<usize>() {
                                    Ok(0) => {
                                        let item = player.inventory.remove(num);
                                        println!("Dropped {} in the room.", &item.name);
                                        if let Some(room) = world_map.get_mut(&current_room_id) {
                                            room.add_item(item);
                                        }
                                        break;
                                    }
                                    Ok(1) => {
                                        println!("\n{}: {}", player.inventory[num].name, player.inventory[num].description);
                                    }
                                    Ok(2) => {
                                        if is_junk {
                                            break;
                                        } else {
                                            println!("using item...");
                                            let item = player.inventory.remove(num);
                                            if let Some(room) = world_map.get_mut(&current_room_id) {
                                                let item_used = use_item(&mut player, room, item.clone());
                                                if !item_used {
                                                    player.inventory.push(item);
                                                }
                                                break;
                                            }
                                        }
                                    }
                                    Ok(3) => {
                                        break;
                                    }
                                    _ => println!("Invalid choice. Try again.")
                                }
                            }
                            Ok(num) if num == player.inventory.len() => {
                                break;
                            }
                            _ => println!("Invalid choice. Try again.")
                        }
                    }
                }
            }
            "map" => {
                display_map(&world_map, &visited_rooms, &current_room_id);
            }
            "help" | "?" | "h" => {
                println!("So you need some help...");
                println!("look          # See current room");
                println!("move          # Navigate between rooms");
                println!("take          # Pick up the item in the room");
                println!("inventory     # Check your items (should be empty)");
                println!("  -drop          # Drop an item in the room");
                println!("help          # See available commands");
                println!("quit          # Exit game");
                continue;
                // create help fn
            }
            "quit" => {
                println!("Goodbye traveler!");
                break;
            }
            // "test" => {
                // add_item(&mut player);
                // continue;
                // let test_potion = items::create_health_potion();
                // println!("{}", test_potion.describe());
            // }
            _ => {
                println!("Use a valid action please! {}", &ACTIONS_STR);
                continue;
            }
        }
    }

    Ok(())
}

/// Player
/// Use an item from the inventory
fn use_item(player: &mut Player, room: &mut Room, item: Item) -> bool {
    match item.item_type {
        ItemType::Potion { healing } => {
            player.health += healing;
            println!("You drink the {} and heal for {}!", item.name, healing);
            true // Consumed
        }
        ItemType::Poison { damage } => {
            player.health = player.health.saturating_sub(damage);
            println!("Ouch! The {} dealt {} damage!", item.name, damage);
            true // Consumed
        }
        ItemType::Key { unlocks: _ } => {
            // Logic to check if 'unlocks' matches something in the room
            println!("You try to use the {}...", item.name);
            if room.room_type == RoomType::TreasureRoom {
                room.items.push(items::create_gold());
                println!("You use the key to open the chest and find a bag of gold inside!")
            } else {
                println!("There is nothing in this room to use it on.");
            }
            false // Not consumed
        }
        ItemType::Gold => {
            println!("Nothing to use the gold on here...");
            false
        }
        ItemType::Junk => {
            println!("You fiddle with the {}, but nothing happens.", item.name);
            false // Not consumed
        }
    }
}

// color health based on percentage
fn color_health(current: u32, total: u32) -> colored::ColoredString {
    let text = current.to_string();
    let percentage = (current * 100) / total;
    if  percentage >= 75 {
        text.green()
    } else if percentage >= 30 {
        text.yellow()
    } else {
        text.red()
    }
}

fn display_map(world_map: &HashMap<String, Room>,visited_rooms: &HashSet<String>, current_room_id: &str) {
    let mut queue: VecDeque<String> = VecDeque::new();
    queue.push_back(String::from("cave_entrance"));

    let mut positions: HashMap<String, (i32, i32)> = HashMap::new();
    positions.insert(String::from("cave_entrance"), (0, 0));

    while let Some(current_id) = queue.pop_front() {
        if let Some(room) = world_map.get(&current_id) {
            for (direction, neighbor_id) in &room.exits {
                // check if exit id matches queue
                // if not add to queue and add to pso_x and pos_y
                // insert to positions
                if !positions.contains_key(neighbor_id.as_str()) {
                    queue.push_back(neighbor_id.clone());

                    let (cx, cy) = positions[&current_id];
                    // then for each exit direction:
                    let (nx, ny) = match direction {
                        Direction::North => (cx, cy - 1),
                        Direction::South => (cx, cy + 1),
                        Direction::East  => (cx + 1, cy),
                        Direction::West  => (cx - 1, cy),
                    };
                    positions.insert(neighbor_id.clone(), (nx, ny));
                }
            }
        }
    }
    println!("{:?}", &positions);

    // Normalize positions
    let min_x = positions.values().map(|&(x, _)| x).min().unwrap();
    let min_y = positions.values().map(|&(_, y)| y).min().unwrap();
    let max_x = positions.values().map(|&(x, _)| x).max().unwrap();
    let max_y = positions.values().map(|&(_, y)| y).max().unwrap();
    let width = max_x - min_x + 1;
    let height = max_y - min_y + 1;
    let mut normalized_positions = HashMap::new();
    for (id, (x, y)) in positions {
        normalized_positions.insert(id, (x - min_x, y - min_y));
    }

    println!("{:?}", &normalized_positions);

    // Build render grid
    let mut grid  = vec![vec![String::from("     "); (width * 2 + 1) as usize]; (height * 2 + 1) as usize];
    for (id, (x, y)) in normalized_positions {
        let room = world_map.get(&id).unwrap();

        // add room to grid
        if current_room_id == &id {
            grid[y as usize * 2][x as usize * 2] = format!("{}", room.name.magenta());
        } else if visited_rooms.contains(&id) {
            grid[y as usize * 2][x as usize * 2] = format!("{}", room.name);
        } else {
            grid[y as usize * 2][x as usize * 2] = format!(" ??? ", );
        }

        // add room exits to grid
        for (direction, _) in &room.exits {
            let gx = x * 2;
              let gy = y * 2;
              let (cx, cy) = match direction {
                  Direction::North => (gx, gy - 1),
                  Direction::South => (gx, gy + 1),
                  Direction::East  => (gx + 1, gy),
                  Direction::West  => (gx - 1, gy),
              };
              if cx >= 0 && cy >= 0 && (cy as usize) < grid.len() && (cx as
                usize) < grid[0].len() {
                grid[cy as usize][cx as usize] = match direction {
                    Direction::North => format!("  |  "),
                    Direction::South => format!("  |  "),
                    Direction::East  => format!("-----"),
                    Direction::West  => format!("-----"),
                };
            }
          }
    }

    for row in &grid {
         println!("{}", row.join(""));
    }

}


/// Room
/// Get direction choice of the room
fn get_direction_choice(room: &Room) -> Option<Direction> {
    if room.exits.is_empty() {
        return None;
    }

    println!("{}", room.list_exits());
    println!("  [{}] Stay here\n", room.exits.len());

    // Convert HashMap keys to a Vec so we can index them
    let directions: Vec<&Direction> = room.exits.keys().collect();

    loop {
        print!("Choose a direction: ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).ok()?;

        match input.trim().parse::<usize>() {
            Ok(num) if num < directions.len() => {
                return Some(directions[num].clone());  // Return the Direction, not the index
            }
            Ok(num) if num == directions.len() => {
                return None;  // Stay here
            }
            _ => println!("Invalid choice. Try again.")
        }
    }
}

/// Get the item choice of the room
fn get_item_choice(room: &Room) -> Option<Item> {
    if room.items.is_empty() {
        return None;
    }

    println!("{}", room.list_items());
    println!("  [{}] Leave items\n", room.items.len());

    loop {
        println!("Choose an item to take");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).ok()?;
        match input.trim().parse::<usize>() {
            Ok(num) if num < room.items.len() => {
                return Some(room.items[num].clone());
            }
            Ok(num) if num == room.items.len() => {
                return None;
            }
            _ => println!("Invalid choice. Try again.")
        }
    }
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_player() -> Player {
        Player {
            name: String::from("Test Player"),
            health: 20,
            damage: 5,
            inventory: Vec::new(),
        }
    }

    fn create_test_room(room_type: RoomType) -> Room {
        Room::new(
            String::from("Test Room"),
            String::from("A test room"),
            room_type,
            HashMap::new(),
            HashMap::new(),
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

        let consumed = use_item(&mut player, &mut room, potion);

        assert!(consumed);
        assert_eq!(player.health, 30); // 10 + 20 healing
    }

    #[test]
    fn test_use_item_small_potion() {
        let mut player = create_test_player();
        player.health = 5;
        let mut room = create_test_room(RoomType::Normal);
        let potion = items::create_small_health_potion();

        let consumed = use_item(&mut player, &mut room, potion);

        assert!(consumed);
        assert_eq!(player.health, 15); // 5 + 10 healing
    }

    #[test]
    fn test_use_item_poison() {
        let mut player = create_test_player();
        let mut room = create_test_room(RoomType::Normal);
        let poison = items::create_poison_vial();

        let consumed = use_item(&mut player, &mut room, poison);

        assert!(consumed);
        assert_eq!(player.health, 5); // 20 - 15 damage
    }

    #[test]
    fn test_use_item_poison_saturating() {
        let mut player = create_test_player();
        player.health = 5;
        let mut room = create_test_room(RoomType::Normal);
        let poison = items::create_poison_vial(); // 15 damage

        let consumed = use_item(&mut player, &mut room, poison);

        assert!(consumed);
        assert_eq!(player.health, 0); // saturating_sub: 5 - 15 = 0
    }

    #[test]
    fn test_use_item_key_in_treasure_room() {
        let mut player = create_test_player();
        let mut room = create_test_room(RoomType::TreasureRoom);
        let key = items::create_brass_key();

        assert_eq!(room.items.len(), 0);

        let consumed = use_item(&mut player, &mut room, key);

        assert!(!consumed); // Keys are not consumed
        assert_eq!(room.items.len(), 1); // Gold added to room
        assert_eq!(room.items[0].name, "Bag of Gold");
    }

    #[test]
    fn test_use_item_key_in_normal_room() {
        let mut player = create_test_player();
        let mut room = create_test_room(RoomType::Normal);
        let key = items::create_brass_key();

        let consumed = use_item(&mut player, &mut room, key);

        assert!(!consumed);
        assert_eq!(room.items.len(), 0); // No gold added
    }

    #[test]
    fn test_use_item_gold() {
        let mut player = create_test_player();
        let mut room = create_test_room(RoomType::Normal);
        let gold = items::create_gold();

        let consumed = use_item(&mut player, &mut room, gold);

        assert!(!consumed);
        assert_eq!(player.health, 20); // Health unchanged
    }

    #[test]
    fn test_use_item_junk() {
        let mut player = create_test_player();
        let mut room = create_test_room(RoomType::Normal);
        let junk = items::create_broken_cup();

        let consumed = use_item(&mut player, &mut room, junk);

        assert!(!consumed);
        assert_eq!(player.health, 20); // Health unchanged
    }
}
