use colored::Colorize;

use std::collections::{HashMap, HashSet, VecDeque};

use crate::rooms::{Direction, Room};

const CELL_WIDTH: usize = 10;

/// Pad or truncate a string to exactly CELL_WIDTH characters, centered.
fn pad_name(name: &str) -> String {
    if name.len() >= CELL_WIDTH {
        name[..CELL_WIDTH].to_string()
    } else {
        let total_pad = CELL_WIDTH - name.len();
        let left = total_pad / 2;
        let right = total_pad - left;
        format!("{}{}{}", " ".repeat(left), name, " ".repeat(right))
    }
}

/// Pad a colored string to CELL_WIDTH by adding spaces around it.
/// Colored strings have hidden ANSI codes, so we pad based on the original text length.
fn pad_colored(name: &str, colored: colored::ColoredString) -> String {
    let total_pad = CELL_WIDTH.saturating_sub(name.len());
    let left = total_pad / 2;
    let right = total_pad - left;
    format!("{}{}{}", " ".repeat(left), colored, " ".repeat(right))
}

fn empty_cell() -> String {
    " ".repeat(CELL_WIDTH)
}

fn connector_vertical() -> String {
    let left = CELL_WIDTH / 2;
    let right = CELL_WIDTH - left - 1;
    format!("{}|{}", " ".repeat(left), " ".repeat(right))
}

fn connector_horizontal() -> String {
    let dashes = 4;
    let pad = (CELL_WIDTH - dashes) / 2;
    format!("{}{}{}", " ".repeat(pad), "-".repeat(dashes), " ".repeat(CELL_WIDTH - pad - dashes))
}

pub fn display_map(
    world_map: &HashMap<String, Room>,
    visited_rooms: &HashSet<String>,
    current_room_id: &str,
) {
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
                        Direction::East => (cx + 1, cy),
                        Direction::West => (cx - 1, cy),
                    };
                    positions.insert(neighbor_id.clone(), (nx, ny));
                }
            }
        }
    }

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

    // Build render grid
    let mut grid =
        vec![vec![empty_cell(); (width * 2 + 1) as usize]; (height * 2 + 1) as usize];
    for (id, (x, y)) in &normalized_positions {
        let room = world_map.get(id).unwrap();

        // add room to grid
        if current_room_id == id {
            grid[*y as usize * 2][*x as usize * 2] = pad_colored(&room.name, room.name.magenta());
        } else if visited_rooms.contains(id) {
            grid[*y as usize * 2][*x as usize * 2] = pad_name(&room.name);
        } else {
            grid[*y as usize * 2][*x as usize * 2] = pad_name("???");
        }

        // add room exits to grid
        for (direction, _) in &room.exits {
            let gx = x * 2;
            let gy = y * 2;
            let (cx, cy) = match direction {
                Direction::North => (gx, gy - 1),
                Direction::South => (gx, gy + 1),
                Direction::East => (gx + 1, gy),
                Direction::West => (gx - 1, gy),
            };
            if cx >= 0 && cy >= 0 && (cy as usize) < grid.len() && (cx as usize) < grid[0].len() {
                grid[cy as usize][cx as usize] = match direction {
                    Direction::North | Direction::South => connector_vertical(),
                    Direction::East | Direction::West => connector_horizontal(),
                };
            }
        }
    }

    for row in &grid {
        println!("{}", row.join(""));
    }
}
