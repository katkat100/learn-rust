use colored::Colorize;

use std::collections::{HashMap, HashSet};
use std::io::{self};

mod items;
use items::{Item, ItemType};

mod rooms;
use rooms::{Direction, Room, RoomType};

mod npc;
use npc::NPCType;
mod player;
use player::Player;

mod map;
use map::display_map;

const ACTIONS_STR: &str = "(look/move/take/inventory/stats/talk/map/help/quit)";

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("{}", "Welcome to the adventure!".bright_yellow());

    println!("What is your name adventurer?");
    let mut name_str = String::new();
    io::stdin().read_line(&mut name_str)?;

    let mut player = Player::new(
        String::from(name_str.trim()),
        20,
        20,
        5,
        0,
        vec![items::create_apple()],
        0,
        player::EquippedItems {
            head: None,
            body: None,
            left_hand: None,
            right_hand: None,
            feet: None,
        },
        Vec::new(),
    );

    println!("\nHello {}!", player.name);

    let mut world_map: HashMap<String, Room> = HashMap::new();
    let mut visited_rooms: HashSet<String> = HashSet::new();

    // === Row 8 (bottom) ===

    // cave_entrance (H8) - Starting room
    let mut exits = HashMap::new();
    exits.insert(Direction::West, String::from("norm_g8"));
    exits.insert(Direction::North, String::from("norm_h7"));
    exits.insert(Direction::East, String::from("cleaners_room"));
    world_map.insert(String::from("cave_entrance"), Room::new_with_items(
        String::from("Start"),
        String::from("You find yourself in a dark, damp cave. Water drips from the stalactites above, echoing through the chamber. The air is cold and musty. You can barely make out the rough stone walls in the dim light filtering from somewhere above."),
        RoomType::Normal,
        vec![items::create_broken_cup(), items::create_rusty_spoon(), items::create_poison_vial(), items::create_hat()],
        exits,
        HashMap::new(),
        HashMap::new(),
        None,
        None,
    ));

    // cleaners_room (I8) - Cleaner NPC
    let mut exits = HashMap::new();
    exits.insert(Direction::West, String::from("cave_entrance"));
    world_map.insert(
        String::from("cleaners_room"),
        Room::new(
            String::from("Cleaners Room"),
            String::from("You enter a bare room with a table and a single occupied chair."),
            RoomType::Normal,
            exits,
            HashMap::new(),
            HashMap::new(),
            None,
            Some(npc::create_npc_cleaner()),
        ),
    );

    // norm_g8 (G8)
    let mut exits = HashMap::new();
    exits.insert(Direction::West, String::from("norm_f8"));
    exits.insert(Direction::East, String::from("cave_entrance"));
    world_map.insert(
        String::from("norm_g8"),
        Room::new_with_items(
            String::from("Slime Tunnel"),
            String::from("A narrow tunnel coated in a thin layer of translucent goo. The walls glisten in the dim light, and the air carries a faintly sweet, rotten smell."),
            RoomType::Normal,
            vec![items::create_small_health_potion()],
            exits,
            HashMap::new(),
            HashMap::new(),
            Some(rooms::create_slime()),
            None,
        ),
    );

    // norm_f8 (F8)
    let mut exits = HashMap::new();
    exits.insert(Direction::West, String::from("norm_e8"));
    exits.insert(Direction::East, String::from("norm_g8"));
    world_map.insert(
        String::from("norm_f8"),
        Room::new_with_items(
            String::from("Kobold Den"),
            String::from("Crude scratches cover the walls like tally marks. Small bones and scraps of leather litter the ground. Something has been living here."),
            RoomType::Normal,
            vec![items::create_rusty_sword()],
            exits,
            HashMap::new(),
            HashMap::new(),
            Some(rooms::create_kobold()),
            None,
        ),
    );

    // norm_e8 (E8)
    let mut exits = HashMap::new();
    exits.insert(Direction::North, String::from("norm_e7"));
    exits.insert(Direction::East, String::from("norm_f8"));
    world_map.insert(
        String::from("norm_e8"),
        Room::new_with_items(
            String::from("Bone Corridor"),
            String::from("Fragments of old armor and scattered bones crunch underfoot. The remains of past adventurers line the walls like a grim warning."),
            RoomType::Normal,
            vec![items::create_wooden_shield()],
            exits,
            HashMap::new(),
            HashMap::new(),
            Some(rooms::create_skeleton()),
            None,
        ),
    );

    // === Row 7 ===

    // norm_e7 (E7)
    let mut exits = HashMap::new();
    exits.insert(Direction::North, String::from("norm_e6"));
    exits.insert(Direction::South, String::from("norm_e8"));
    world_map.insert(
        String::from("norm_e7"),
        Room::new_with_items(
            String::from("Dripping Passage"),
            String::from("Water trickles steadily down the walls, pooling in shallow puddles on the uneven stone floor. The constant dripping echoes through the narrow passage."),
            RoomType::Normal,
            vec![items::create_bread()],
            exits,
            HashMap::new(),
            HashMap::new(),
            Some(rooms::create_slime()),
            None,
        ),
    );

    // norm_h7 (H7)
    let mut exits = HashMap::new();
    exits.insert(Direction::North, String::from("norm_h6"));
    exits.insert(Direction::South, String::from("cave_entrance"));
    world_map.insert(
        String::from("norm_h7"),
        Room::new_with_items(
            String::from("Goblin's Crossing"),
            String::from("Crude drawings of stick figures and arrows are smeared across the walls in what looks like charcoal. A foul stench hangs in the air, and a half-eaten rat sits on a flat rock."),
            RoomType::Normal,
            vec![items::create_one_gold(), items::create_jerky()],
            exits,
            HashMap::new(),
            HashMap::new(),
            Some(rooms::create_goblin()),
            None,
        ),
    );

    // boss_room (C7)
    let mut exits = HashMap::new();
    exits.insert(Direction::North, String::from("dungeon_2"));
    exits.insert(Direction::South, String::from("end"));
    world_map.insert(String::from("boss_room"), Room::new(
        String::from("Dragon's Lair"),
        String::from("You enter a vast cavern. The ceiling disappears into darkness high above. Piles of gold and jewels are scattered everywhere, but your eyes are drawn to the massive dragon sleeping atop the largest treasure heap. Its scales shimmer even in the dim light."),
        RoomType::BossRoom,
        exits,
        HashMap::new(),
        HashMap::new(),
        Some(rooms::create_dragon()),
        None,
    ));

    // === Row 6 ===

    // dungeon_2 (C6)
    let mut exits = HashMap::new();
    exits.insert(Direction::East, String::from("norm_d6"));
    exits.insert(Direction::South, String::from("boss_room"));
    world_map.insert(String::from("dungeon_2"), Room::new_with_items(
        String::from("Dungeon Cell"),
        String::from("The walls are made of cold, rough stone. Rusted chains hang from the walls, and the floor is covered in filthy straw. A small barred window near the ceiling lets in a sliver of pale moonlight."),
        RoomType::Dungeon,
        vec![items::create_chest_plate(), items::create_tough_potion()],
        exits,
        HashMap::new(),
        HashMap::new(),
        Some(rooms::create_skeleton()),
        None,
    ));

    // norm_d6 (D6) - locked west exit to dungeon_2
    let mut exits = HashMap::new();
    exits.insert(Direction::West, String::from("dungeon_2"));
    exits.insert(Direction::East, String::from("norm_e6"));
    let mut locked_exits = HashMap::new();
    locked_exits.insert(Direction::West, String::from("Iron Key"));
    world_map.insert(
        String::from("norm_d6"),
        Room::new_with_items(
            String::from("Locked Chamber"),
            String::from("A heavy iron door dominates the western wall, its surface pitted with rust and age. Deep gouges in the stone floor suggest something was once dragged through here."),
            RoomType::Normal,
            vec![items::create_iron_sword(), items::create_medium_health_potion()],
            exits,
            locked_exits,
            HashMap::new(),
            Some(rooms::create_slime()),
            None,
        ),
    );

    // norm_e6 (E6) - Merchant NPC
    let mut exits = HashMap::new();
    exits.insert(Direction::West, String::from("norm_d6"));
    exits.insert(Direction::East, String::from("norm_f6"));
    exits.insert(Direction::South, String::from("norm_e7"));
    world_map.insert(
        String::from("norm_e6"),
        Room::new(
            String::from("Underground Market"),
            String::from("A wider cavern with a makeshift wooden stall propped against one wall. Lanterns hang from rusty hooks, casting a warm glow over scattered wares and trinkets."),
            RoomType::Normal,
            exits,
            HashMap::new(),
            HashMap::new(),
            Some(rooms::create_skeleton()),
            Some(npc::create_npc_merchant()),
        ),
    );

    // norm_f6 (F6) - Brass Key here
    let mut exits = HashMap::new();
    exits.insert(Direction::West, String::from("norm_e6"));
    world_map.insert(
        String::from("norm_f6"),
        Room::new_with_items(
            String::from("Snake's Nest"),
            String::from("The air is thick and humid. Shed scales crunch beneath your feet and the faint sound of hissing echoes from the shadows. A warm draft rises from cracks in the floor."),
            RoomType::Normal,
            vec![items::create_brass_key()],
            exits,
            HashMap::new(),
            HashMap::new(),
            Some(rooms::create_two_headed_snake()),
            None,
        ),
    );

    // norm_h6 (H6)
    let mut exits = HashMap::new();
    exits.insert(Direction::North, String::from("dungeon_1"));
    exits.insert(Direction::South, String::from("norm_h7"));
    world_map.insert(
        String::from("norm_h6"),
        Room::new_with_items(
            String::from("Crumbling Hallway"),
            String::from("The ceiling sags dangerously low in places and loose stones litter the path. A worn trail in the dust leads north toward what looks like iron bars."),
            RoomType::Normal,
            vec![items::create_hard_cheese()],
            exits,
            HashMap::new(),
            HashMap::new(),
            Some(rooms::create_skeleton()),
            None,
        ),
    );

    // treasure_room (J6)
    let mut exits = HashMap::new();
    exits.insert(Direction::North, String::from("norm_j5"));
    world_map.insert(String::from("treasure_room"), Room::new_with_items(
        String::from("Treasure Room"),
        String::from("A room that looks to have been filled with treasure, maybe you can find something useful."),
        RoomType::TreasureRoom,
        vec![items::create_health_potion(), items::create_ring(), items::create_enchanted_blade()],
        exits,
        HashMap::new(),
        HashMap::new(),
        Some(rooms::create_giant_spider()),
        None,
    ));

    // === Row 5 (top) ===

    // dungeon_1 (H5) - Iron Key + QuestGiver NPC (Jacob)
    let mut exits = HashMap::new();
    exits.insert(Direction::East, String::from("norm_i5"));
    exits.insert(Direction::South, String::from("norm_h6"));
    world_map.insert(String::from("dungeon_1"), Room::new_with_items(
        String::from("Dungeon Cell"),
        String::from("The walls are made of cold, rough stone. Rusted chains hang from the walls, and the floor is covered in filthy straw. A small barred window near the ceiling lets in a sliver of pale moonlight."),
        RoomType::Dungeon,
        vec![items::create_torn_page(), items::create_iron_key()],
        exits,
        HashMap::new(),
        HashMap::new(),
        Some(rooms::create_skeleton()),
        Some(npc::create_npc_regular_guy()),
    ));

    // norm_i5 (I5)
    let mut exits = HashMap::new();
    exits.insert(Direction::West, String::from("dungeon_1"));
    exits.insert(Direction::East, String::from("norm_j5"));
    world_map.insert(
        String::from("norm_i5"),
        Room::new_with_items(
            String::from("Glowing Cavern"),
            String::from("Veins of luminescent minerals streak through the rock, casting an eerie blue-green glow across the cavern. Patches of thick slime cling to the lower walls."),
            RoomType::Normal,
            vec![items::create_shiny_gem()],
            exits,
            HashMap::new(),
            HashMap::new(),
            Some(rooms::create_giant_slime()),
            None,
        ),
    );

    // norm_j5 (J5) - locked south exit to treasure_room
    let mut exits = HashMap::new();
    exits.insert(Direction::West, String::from("norm_i5"));
    // exits.insert(Direction::South, String::from("treasure_room"));
    let locked_exits = HashMap::new();
    // locked_exits.insert(Direction::South, String::from("Brass Key"));
    let mut secret_exits = HashMap::new();
    secret_exits.insert(
        Direction::South,
        (String::from("treasure_room"), String::from("Shiny Gem")),
    );
    world_map.insert(
        String::from("norm_j5"),
        Room::new_with_items(
            String::from("Whispering Chamber"),
            String::from("The walls seem to hum with a faint vibration. A cool breeze drifts from somewhere to the south, carrying with it the faintest glimmer of light."),
            RoomType::Normal,
            vec![items::create_buff_potion()],
            exits,
            locked_exits,
            secret_exits,
            Some(rooms::create_slime()),
            None,
        ),
    );

    // === End room (C8) ===

    let mut exits = HashMap::new();
    exits.insert(Direction::North, String::from("boss_room"));
    world_map.insert(String::from("end"), Room::new(
        String::from("Freedom!"),
        String::from("You emerge from the dark dungeon into brilliant sunlight. The fresh air fills your lungs as you stand at the entrance, the dragon's lair far behind you. Against all odds, you survived. The adventure is over, but the memories will last forever."),
        RoomType::Normal,
        exits,
        HashMap::new(),
        HashMap::new(),
        None,
        None,
    ));
    let mut current_room_id = String::from("cave_entrance");
    visited_rooms.insert(current_room_id.clone());
    let mut previous_room_id = String::from("cave_entrance");

    println!("\n");
    println!("---------");

    println!("\n{}", world_map[&current_room_id].describe());

    loop {
        // --- SECRET ROOMS ---
        // Check if current room has a secret exit
        let mut revealed: Vec<(Direction, String, String)> = Vec::new();
        if let Some(room) = world_map.get_mut(&current_room_id) {
            for (direction, (room_name, required_item)) in &room.secret_exits {
                if player
                    .inventory
                    .iter()
                    .any(|item| item.name == *required_item)
                {
                    revealed.push((direction.clone(), room_name.clone(), required_item.clone()));
                }
            }
        }

        for (_, _, item_name) in &revealed {
            if let Some(index) = player
                .inventory
                .iter()
                .position(|item| item.name == *item_name)
            {
                player.inventory.remove(index);
            }
        }

        if let Some(room) = world_map.get_mut(&current_room_id) {
            for (dir, room_name, item_name) in revealed {
                room.secret_exits.remove(&dir);
                room.exits.insert(dir, room_name);
                println!(
                    "The {} started glowing and flew to the {} wall!",
                    item_name,
                    dir.as_str()
                );
                println!("You have revealed a secret exit!");
            }
        }
        // --- SECRET ROOMS ---

        // ---MONSTER FIGHTS ---
        let monster_info = world_map
            .get(&current_room_id)
            .and_then(|room| room.monster.as_ref())
            .map(|m| {
                (
                    m.name.clone(),
                    m.health,
                    m.description.clone(),
                    m.drop.clone(),
                )
            });

        if let Some((monster_name, monster_health, monster_description, monster_drop)) =
            monster_info
        {
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

            println!(
                "\nThere's a {} in this room!\n{}\nWhat will you do?",
                monster_name.to_string().magenta(),
                monster_description
            );
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
                        && let Some(monster) = &mut room.monster
                    {
                        let damage = player.total_damage();
                        let defense = player.total_defense();

                        monster.health = monster.health.saturating_sub(damage);
                        println!(
                            "\nYou hit the {} for {} damage!\n",
                            monster.name.to_string().magenta(),
                            damage.to_string().red()
                        );
                        player.wear_damage_equipment();

                        if monster.health > 0 {
                            player.health = player
                                .health
                                .saturating_sub(monster.damage.saturating_sub(defense));
                            println!(
                                "The {} hits you for {} damage! Health: {}",
                                monster.name.to_string().magenta(),
                                monster.damage.to_string().red(),
                                player.color_health()
                            );
                            player.wear_defense_equipment();

                            if player.health == 0 {
                                println!("{}", "\n╔════════════════════════════════════╗".red());
                                println!("{}", "║         💀 GAME OVER 💀            ║".red());
                                println!("{}", "╚════════════════════════════════════╝".red());
                                println!("You were defeated by the {}...", monster.name.magenta());
                                return Ok(());
                            }
                        }
                        player.tick_buffs();
                    }
                }
                Ok(1) => {
                    println!("Inventory check!");
                    loop {
                        println!("You have multiple usable items");
                        let useable_items: Vec<Item> = player
                            .inventory
                            .iter()
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
                                    let (item_used, line) = player.use_item(room, item.clone());
                                    if line.is_some() {
                                        println!("{}", line.unwrap());
                                    }
                                    if item_used {
                                        let item_name = item.name.clone();
                                        if let Some(index) = player
                                            .inventory
                                            .iter()
                                            .position(|inv_item| inv_item.name == item_name)
                                        {
                                            player.inventory.remove(index);
                                        }
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
                Ok(2) => {
                    if let Some(room) = world_map.get_mut(&current_room_id)
                        && let Some(monster) = &room.monster
                    {
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

        let parts: Vec<&str> = action.trim().splitn(2, ' ').collect();

        match parts[0] {
            "look" => {
                println!("\n{}", world_map[&current_room_id].describe());
                continue;
            }
            "go" | "move" => {
                let callback = handle_move(
                    parts,
                    &mut world_map,
                    &mut current_room_id,
                    &mut player,
                    &mut previous_room_id,
                    &mut visited_rooms,
                );

                if !callback {
                    break;
                }

                continue;
            }
            "examine" => {
                let callback = handle_examine(parts, &mut world_map, &mut current_room_id);

                if !callback {
                    break;
                }

                continue;
            }
            "talk" => {
                let callback = handle_talk(&mut world_map, &mut current_room_id, &mut player);

                if !callback {
                    break;
                }

                continue;
            }
            "use" => {
                handle_use(parts, &mut player, &mut world_map, &mut current_room_id);
            }
            "take" => {
                handle_take(parts, &mut world_map, &mut current_room_id, &mut player);
            }
            "stats" => {
                handle_stats(&player);
            }
            "inventory" | "inv" | "i" => {
                handle_inv(&mut player, &mut world_map, &mut current_room_id);
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

fn handle_move(
    parts: Vec<&str>,
    world_map: &mut HashMap<String, Room>,
    current_room_id: &mut String,
    player: &mut Player,
    previous_room_id: &mut String,
    visited_rooms: &mut HashSet<String>,
) -> bool {
    let mut direction = None;
    if let Some(dir_str) = parts.get(1) {
        match *dir_str {
            "north" => direction = Some(Direction::North),
            "south" => direction = Some(Direction::South),
            "east" => direction = Some(Direction::East),
            "west" => direction = Some(Direction::West),
            _ => println!("Invalid direction!"),
        }
    } else {
        direction = {
            let room = &world_map[current_room_id];
            room.get_direction_choice()
        };
    }
    if let Some(direction) = direction {
        if let Some(room) = world_map.get_mut(current_room_id)
            && let Some(required_key) = room.locked_exits.get(&direction)
        {
            if player
                .inventory
                .iter()
                .any(|item| &item.name == required_key)
            {
                println!("You use the {} to unlock the door!", required_key);
                room.locked_exits.remove(&direction);
            } else {
                println!("This door is locked. You need a {}.", required_key);
                return true;
            }
        }
        let next_room_id = world_map[current_room_id].get_exit(&direction).cloned();
        if let Some(new_room_id) = next_room_id {
            *previous_room_id = current_room_id.clone();
            *current_room_id = new_room_id;
            visited_rooms.insert(current_room_id.clone());
            println!("\nYou go {}...\n", direction.as_str());

            // Check if reached exit
            if current_room_id == "end" {
                println!("{}", world_map[current_room_id].describe());
                println!("{}", "\n╔════════════════════════════════════╗".yellow());
                println!("{}", "║      🏆 VICTORY ACHIEVED! 🏆       ║".yellow());
                println!("{}", "╚════════════════════════════════════╝".yellow());
                println!();
                println!("Your adventure statistics:");
                println!("  - Health remaining: {} HP", player.health);
                println!("  - Items collected: {}", player.inventory.len());
                println!();
                println!("Thank you for playing, {}!", player.name);
                println!("The dragon's treasure is yours...");
                return false;
            }

            player.on_move();

            println!("{}", world_map[current_room_id].describe());
        } else {
            println!("You can't go that way!");
        }
    } else {
        println!("You decide to stay put.");
    }
    return true;
}

fn handle_examine(
    parts: Vec<&str>,
    world_map: &mut HashMap<String, Room>,
    current_room_id: &mut String,
) -> bool {
    loop {
        if let Some(item) = parts.get(1) {
            if let Some(item) = world_map[current_room_id]
                .items
                .iter()
                .find(|i| i.name == *item)
            {
                println!("\n==============");
                println!("Examining 🔎...\n");
                println!("\n{}: {}\n", item.name_colored, item.description);
                println!("\n==============");
            } else {
                println!("There is no such item in the room.");
            }
        } else {
            println!("Which item in the room do you want to examine?");
            for (index, item) in world_map[current_room_id].items.iter().enumerate() {
                println!("  [{}] {}", index, item.describe());
            }
            println!("  [{}] Back", world_map[current_room_id].items.len());

            let mut input = String::new();
            io::stdin()
                .read_line(&mut input)
                .expect("Failed to read line");
            match input.trim().parse::<usize>() {
                Ok(num) if num < world_map[current_room_id].items.len() => {
                    let item = &world_map[current_room_id].items[num];
                    println!("\n==============");
                    println!("\nExamining 🔎...");
                    println!("\n{}: {}\n", item.name_colored, item.description);
                    println!("\n==============\n");
                }
                Ok(num) if num == world_map[current_room_id].items.len() => {
                    println!("Going back...");
                    return true;
                }
                _ => println!("Invalid choice. Try again."),
            }
        }
        true;
    }
}

fn handle_stats(player: &Player) -> bool {
    println!(
        "\n{}\n",
        String::from("PLAYER STATS").bold().bright_purple()
    );

    // name
    println!("Name: {}", player.name);
    // health
    println!("Health: {}/{}", player.color_health(), player.max_health);
    // hunger
    println!("Hunger: {}/{}", player.hunger, player.max_hunger);
    // strength
    println!("Strength: {}", player.total_damage());
    // defense
    println!("Defense: {}", player.total_defense());
    // equipment
    let mut equippment = String::new();

    if let Some(item) = player.get_equipped_slot("Head") {
        if let ItemType::Equipment { durability, .. } = &item.item_type {
            equippment += &format!("\nHead: {} (durability: {})", item.name, durability);
        }
    }
    if let Some(item) = player.get_equipped_slot("Body") {
        if let ItemType::Equipment { durability, .. } = &item.item_type {
            equippment += &format!("\nBody: {} (durability: {})", item.name, durability);
        }
    }
    if let Some(item) = player.get_equipped_slot("Feet") {
        if let ItemType::Equipment { durability, .. } = &item.item_type {
            equippment += &format!("\nFeet: {} (durability: {})", item.name, durability);
        }
    }
    let (left, right) = player.get_equipped_hands();
    if let Some(item) = left {
        if let ItemType::Equipment { durability, .. } = &item.item_type {
            equippment += &format!("\nLeft Hand: {} (durability: {})", item.name, durability);
        }
    }

    if let Some(item) = right {
        if let ItemType::Equipment { durability, .. } = &item.item_type {
            equippment += &format!("\nRight Hand: {} (durability: {})", item.name, durability);
        }
    }

    println!("{}", equippment);
    true
}

fn handle_talk(
    world_map: &mut HashMap<String, Room>,
    current_room_id: &mut String,
    player: &mut Player,
) -> bool {
    if let Some(room) = world_map.get_mut(current_room_id) {
        if let Some(npc) = &mut room.npc {
            println!("{}", &npc.talk());

            let actions = npc.available_actions();
            for (i, action) in actions.iter().enumerate() {
                println!("  [{}] {}", i, action);
            }
            let mut input = String::new();
            io::stdin()
                .read_line(&mut input)
                .expect("Failed to read line");

            match input.trim().parse::<usize>() {
                Ok(num) if num < actions.len() => match actions[num] {
                    "Trade junk" => {
                        let mut variation = 0;
                        loop {
                            let players_junk: Vec<Item> = player
                                .inventory
                                .iter()
                                .filter(|i| i.item_type == ItemType::Junk)
                                .cloned()
                                .collect();
                            if variation == 0 {
                                println!(
                                    "\n{}: \"Lets see what you've picked up.\"",
                                    npc.name.green()
                                );
                            } else {
                                println!("\n{}: \"Anything else?\"", npc.name.green());
                            }

                            variation += 1;

                            println!("\n=== Your Inventory ===");
                            for (index, item) in players_junk.iter().enumerate() {
                                println!("  [{}] {}", index, item.describe());
                            }
                            if players_junk.is_empty() {
                                println!("----    [EMPTY]    ----");
                            }
                            println!("  [{}] Close Inventory", player.inventory.len());

                            let mut input = String::new();
                            io::stdin()
                                .read_line(&mut input)
                                .expect("Failed to read line");

                            match input.trim().parse::<usize>() {
                                Ok(index) if index < player.inventory.len() => {
                                    let junk_item = players_junk[index].clone();
                                    if let Some(inv_index) = player
                                        .inventory
                                        .iter()
                                        .position(|i| i.name == junk_item.name)
                                    {
                                        let item = player.inventory.remove(inv_index);
                                        player.gold += 1;
                                        println!("You traded {} for 1 gold", item.name_colored);
                                    }
                                }
                                Ok(index) if index == player.inventory.len() => {
                                    println!(
                                        "\n{}: \"If you find any junk, bring it to me.\"",
                                        npc.name.green()
                                    );
                                    break;
                                }
                                _ => println!("Invalid input"),
                            }
                        }
                    }
                    "Buy" => {
                        let shop_items = &mut npc.items;
                        if shop_items.iter().len() == 0 {
                            println!("{}: \"I'm sorry, but I'm out of stock.\"", npc.name.green());
                            return true;
                        } else {
                            loop {
                                println!("{}: \"Come take a look at my wares.\"", npc.name.green());
                                for (index, item) in shop_items.iter().enumerate() {
                                    println!(
                                        "  [{}]: {} - {} gold",
                                        index,
                                        item.name,
                                        (item.cost + 1).to_string().bold()
                                    );
                                }
                                println!("  [{}]: {}", shop_items.len(), "Leave");

                                let mut shop_input = String::new();
                                io::stdin()
                                    .read_line(&mut shop_input)
                                    .expect("Failed to read line");

                                match shop_input.trim().parse::<usize>() {
                                    Ok(index) if index < shop_items.len() => {
                                        let item = shop_items[index].clone();
                                        if player.gold >= item.cost {
                                            shop_items.remove(index);
                                            player.gold -= item.cost;
                                            player.inventory.push(item.clone());
                                            println!(
                                                "{}: \"Thank you for your purchase!\"",
                                                npc.name.green()
                                            );
                                        } else {
                                            println!(
                                                "\n{}: \"I don't think you can afford that.\"\n",
                                                npc.name.green()
                                            );
                                        }
                                    }
                                    Ok(index) if index == shop_items.len() => {
                                        println!("{}: \"Come back anytime!\"", npc.name.green());
                                        break;
                                    }
                                    _ => println!(
                                        "\n{}: \"I don't think I have that.\"\n",
                                        npc.name.green()
                                    ),
                                }
                            }
                        }
                    }
                    "Sell" => {
                        if !player
                            .inventory
                            .iter()
                            .any(|item| item.item_type != ItemType::Junk)
                        {
                            println!("{}: \"You don't have anything to sell!\"", npc.name.green());
                        } else {
                            loop {
                                let player_items: Vec<Item> = player
                                    .inventory
                                    .iter()
                                    .filter(|item| item.item_type != ItemType::Junk)
                                    .cloned()
                                    .collect();
                                println!("\n=== Your Inventory ===");
                                for (index, item) in player_items.iter().enumerate() {
                                    println!("  [{}] {}", index, item.describe());
                                }
                                println!("  [{}] Close Inventory", player_items.len());

                                let mut input = String::new();
                                io::stdin()
                                    .read_line(&mut input)
                                    .expect("Failed to read line");

                                match input.trim().parse::<usize>() {
                                    Ok(index) if index < player_items.len() => {
                                        println!(
                                            "{}: I'll buy that {} for {} gold.",
                                            npc.name.green(),
                                            player_items[index].name_colored,
                                            (player_items[index].cost - 1).to_string().bold()
                                        );

                                        println!("\n  [0] No");
                                        println!("  [1] Yes");

                                        let mut deal_input = String::new();
                                        io::stdin()
                                            .read_line(&mut deal_input)
                                            .expect("Failed to read line");

                                        if deal_input.trim() == "1" {
                                            let play_item = player_items[index].clone();
                                            if let Some(inv_index) = player
                                                .inventory
                                                .iter()
                                                .position(|i| i.name == play_item.name)
                                            {
                                                let item = player.inventory.remove(inv_index);
                                                player.gold += item.cost - 1;
                                                println!(
                                                    "You sold the {} for {} gold",
                                                    item.name_colored,
                                                    (item.cost - 1).to_string().bold()
                                                );
                                                npc.items.push(item);
                                            }
                                        } else if deal_input.trim() == "0" {
                                            println!("You decide not to sell the item.");
                                        } else {
                                            println!("Invalid input");
                                        }
                                    }
                                    Ok(index) if index == player_items.len() => {
                                        println!("\n{}: \"Maybe next time.\"", npc.name.green());
                                        break;
                                    }
                                    _ => println!("Invalid input"),
                                }
                            }
                        }
                    }
                    "Deliver Item" => {
                        if let NPCType::QuestGiver { quest, quest_taken } = &mut npc.npc_type {
                            let quest_item: Vec<Item> = player
                                .inventory
                                .iter()
                                .filter(|i| i.name == *quest)
                                .cloned()
                                .collect();
                            if !quest_item.is_empty() {
                                if let Some(item) =
                                    player.inventory.iter().position(|i| i.name == *quest)
                                {
                                    player.inventory.remove(item);
                                }
                                *quest_taken = true;
                                println!(
                                    "You gave the {} to the {}",
                                    quest_item[0].name,
                                    npc.name.green()
                                );
                            } else {
                                println!("{}", "You don't have the required item".red());
                            }
                        }
                    }
                    "Leave" => return true,
                    _ => {}
                },
                _ => println!("Invalid choice."),
            }
        } else {
            println!("There is no one to talk to.");
        }
    }

    true
}

fn handle_use(
    parts: Vec<&str>,
    player: &mut Player,
    world_map: &mut HashMap<String, Room>,
    current_room_id: &mut String,
) {
    if let Some(item) = parts.get(1) {
        let found_item = player
            .inventory
            .iter()
            .find(|i| i.name.to_lowercase() == item.to_lowercase())
            .cloned();
        if let Some(item) = found_item {
            if let Some(room) = world_map.get_mut(current_room_id) {
                let (item_used, line) = player.use_item(room, item.clone());
                if line.is_some() {
                    print!("{}", line.unwrap())
                }
                if item_used {
                    if let Some(index) = player
                        .inventory
                        .iter()
                        .position(|inv_item| inv_item.name == item.name)
                    {
                        player.inventory.remove(index);
                    }
                }
            }
        } else {
            println!("You do not have that item.");
        }
    } else {
        loop {
            println!("You have multiple usable items");
            let useable_items: Vec<Item> = player
                .inventory
                .iter()
                .filter(|x| {
                    x.item_type != ItemType::Junk
                        && !matches!(x.item_type, ItemType::Gold { gold: _ })
                })
                .cloned()
                .collect();
            for (index, item) in useable_items.iter().enumerate() {
                println!("[{}] {}", index, item.name);
            }
            println!("[{}] Cancel", useable_items.len());

            let mut input = String::new();
            io::stdin()
                .read_line(&mut input)
                .expect("Failed to read input");

            match input.trim().parse::<usize>() {
                Ok(num) if num < useable_items.len() => {
                    let item = useable_items[num].clone();
                    if let Some(room) = world_map.get_mut(current_room_id) {
                        let (item_used, line) = player.use_item(room, item.clone());
                        if line.is_some() {
                            println!("{}", line.unwrap());
                        }
                        if item_used {
                            let item_name = item.name.clone();
                            if let Some(index) = player
                                .inventory
                                .iter()
                                .position(|inv_item| inv_item.name == item_name)
                            {
                                player.inventory.remove(index);
                            }
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
}

fn handle_take(
    parts: Vec<&str>,
    world_map: &mut HashMap<String, Room>,
    current_room_id: &mut String,
    player: &mut Player,
) {
    // First, figure out what item to take (immutable borrow)
    let item_name = if let Some(name) = parts.get(1) {
        let current_room = &world_map[current_room_id];
        let found = current_room
            .items
            .iter()
            .find(|i| i.name.to_lowercase() == name.to_lowercase())
            .map(|i| i.name.clone());
        if found.is_none() {
            println!("There is no '{}' in this room.", name);
        }
        found
    } else {
        let current_room = &world_map[current_room_id];
        if let Some(item) = current_room.get_item_choice() {
            Some(item.name.clone())
        } else {
            println!("No items to take.");
            None
        }
    }; // Immutable borrow ends here

    // Then take it (mutable borrow)
    if let Some(name) = item_name
        && let Some(room) = world_map.get_mut(current_room_id)
        && let Some(taken_item) = room.take_item(&name)
    {
        println!("\nAdded {} to your inventory", &taken_item.name);
        player.inventory.push(taken_item);
    }
}

fn handle_inv(
    player: &mut Player,
    world_map: &mut HashMap<String, Room>,
    current_room_id: &mut String,
) {
    println!("\n=== Your Wallet ===");
    println!(
        "You have {} gold\n",
        player.gold.to_string().bright_yellow()
    );

    if player.inventory.is_empty() {
        println!("Your inventory is empty.");
    } else {
        loop {
            println!("\n=== Your Inventory ===");
            for (index, item) in player.inventory.iter().enumerate() {
                println!("  [{}] {}", index, item.describe());
            }

            println!("  [{}] Nothing", player.inventory.len());

            let mut input = String::new();
            io::stdin()
                .read_line(&mut input)
                .expect("Failed to read line");

            match input.trim().parse::<usize>() {
                Ok(num) if num < player.inventory.len() => {
                    let (item_display, actions) = {
                        let item = &player.inventory[num];
                        let mut item_actions = item.available_actions();
                        item_actions.push("Drop");
                        item_actions.push("Examine");
                        item_actions.push("Nothing");
                        (
                            item.to_string(),
                            // item.item_type == ItemType::Junk,
                            item_actions,
                        )
                    };

                    println!("What would you like to do with the {}", item_display);

                    for (i, action) in actions.iter().enumerate() {
                        println!("  [{}] {}", i, action);
                    }

                    let mut inv_input = String::new();
                    io::stdin()
                        .read_line(&mut inv_input)
                        .expect("Failed to read input");

                    match inv_input.trim().parse::<usize>() {
                        Ok(inv_num) if inv_num < actions.len() => match actions[inv_num] {
                            "Drink" | "Unlock" | "Save" | "Equip" | "Eat" => {
                                let item = player.inventory[num].clone();
                                if let Some(room) = world_map.get_mut(current_room_id) {
                                    let (item_used, line) = player.use_item(room, item.clone());
                                    if line.is_some() {
                                        println!("{}", line.unwrap());
                                    }
                                    if item_used {
                                        let item_name = item.name.clone();
                                        if let Some(index) = player
                                            .inventory
                                            .iter()
                                            .position(|inv_item| inv_item.name == item_name)
                                        {
                                            player.inventory.remove(index);
                                        }
                                    }
                                }
                                break;
                            }
                            "Throw" => {
                                println!("There are no enemies to throw this item at in this room.")
                            }
                            "Drop" => {
                                let item = player.inventory.remove(num);
                                println!("Dropped {} in the room.", &item.name);
                                if let Some(room) = world_map.get_mut(current_room_id) {
                                    room.add_item(item);
                                }
                                break;
                            }
                            "Examine" => {
                                println!(
                                    "\n{}: {}",
                                    player.inventory[num].name, player.inventory[num].description
                                );
                            }
                            "Nothing" => {}
                            _ => {
                                println!("Invalid input");
                            }
                        },
                        _ => {
                            println!("Invalid input");
                        }
                    }
                    // match inv_input.trim().parse::<usize>() {
                    //     Ok(0) => {
                    //         let item = player.inventory.remove(num);
                    //         println!("Dropped {} in the room.", &item.name);
                    //         if let Some(room) = world_map.get_mut(current_room_id) {
                    //             room.add_item(item);
                    //         }
                    //         break;
                    //     }
                    //     Ok(1) => {
                    //         println!(
                    //             "\n{}: {}",
                    //             player.inventory[num].name, player.inventory[num].description
                    //         );
                    //     }
                    //     Ok(2) => {
                    //         if is_junk {
                    //             break;
                    //         } else {
                    //             println!("using item...");
                    //             let item = player.inventory.remove(num);
                    //             if let Some(room) = world_map.get_mut(current_room_id) {
                    //                 let item_used = player.use_item(room, item.clone());
                    //                 if !item_used {
                    //                     player.inventory.push(item);
                    //                 }
                    //                 break;
                    //             }
                    //         }
                    //     }
                    //     Ok(3) => {
                    //         break;
                    //     }
                    //     _ => println!("Invalid choice. Try again."),
                    // }
                }
                Ok(num) if num == player.inventory.len() => {
                    break;
                }
                _ => println!("Invalid choice. Try again."),
            }
        }
    }
}
