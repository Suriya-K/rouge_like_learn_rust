use bracket_lib::color::*;
use bracket_lib::prelude::{BTerm, RandomNumberGenerator, to_cp437};
use std::cmp::{max, min};

use crate::rect::Rect;

#[derive(PartialEq, Clone, Copy)]
pub enum TileType {
    Wall,
    Floor,
    Tree,
}

/// Translate 2D area map into flat vector
/// For exmaple instead of store into map(5)(2) it will use map(156)
/// return usize
pub fn xy_idx(x: i32, y: i32) -> usize {
    (y as usize * 80) + x as usize
}

/// Create a map with solid boundaries and random place tree
pub fn new_map_test() -> Vec<TileType> {
    //declear vector type TileType to create map vector array
    let mut map = vec![TileType::Floor; 80 * 50];

    // for loop to create horizontal wall
    for x in 0..80 {
        map[xy_idx(x, 0)] = TileType::Wall;
        map[xy_idx(x, 49)] = TileType::Wall;
    }

    // for loop to create vertical wall
    for y in 0..50 {
        map[xy_idx(0, y)] = TileType::Wall;
        map[xy_idx(79, y)] = TileType::Wall;
    }

    // using random to create tree inside area map
    let mut rand = RandomNumberGenerator::new();
    for _i in 0..100 {
        let x = rand.roll_dice(1, 78);
        let y = rand.roll_dice(1, 48);
        let idx = xy_idx(x, y);
        if idx != xy_idx(40, 25) {
            map[idx] = TileType::Tree;
        }
    }

    map
}

fn apply_room_to_map(room: &Rect, map: &mut [TileType]) {
    for y in room.y1 + 1..=room.y2 {
        for x in room.x1 + 1..=room.x2 {
            map[xy_idx(x, y)] = TileType::Floor;
        }
    }
}

pub fn new_map_rooms_and_corridors() -> (Vec<TileType>, Vec<Rect>) {
    let mut map = vec![TileType::Wall; 80 * 50];
    let mut rooms: Vec<Rect> = Vec::new();
    const MAX_ROOMS: i32 = 20;
    const MIN_ROOM_SIZE: i32 = 6;
    const MAX_ROOM_SIZE: i32 = 12;
    generate_room(
        MIN_ROOM_SIZE,
        MAX_ROOM_SIZE,
        MAX_ROOMS,
        &mut rooms,
        &mut map,
    );

    (map, rooms)
}

fn generate_room(min: i32, max: i32, limit: i32, rooms: &mut Vec<Rect>, map: &mut [TileType]) {
    let mut rng = RandomNumberGenerator::new();
    for _ in 0..limit {
        let width = rng.range(min, max);
        let hight = rng.range(min, max);
        let pos_x = rng.roll_dice(1, 80 - width - 1) - 1;
        let pos_y = rng.roll_dice(1, 50 - hight - 1) - 1;
        let new_room = Rect::new(pos_x, pos_y, width, hight);
        let mut can_place_room = true;
        for other_room in rooms.iter() {
            if new_room.intersect(other_room) {
                can_place_room = false
            }
        }
        if can_place_room {
            apply_room_to_map(&new_room, map);
            if !rooms.is_empty() {
                let (new_x, new_y) = new_room.center();
                let (prev_x, prev_y) = rooms[rooms.len() - 1].center();
                if rng.range(0, 2) == 1 {
                    apply_horizontal_tunnel(map, prev_x, new_x, prev_y);
                    apply_vertical_tunnel(map, prev_y, new_y, new_x);
                } else {
                    apply_horizontal_tunnel(map, prev_x, new_x, new_y);
                    apply_vertical_tunnel(map, prev_y, new_y, prev_x);
                }
            }
            rooms.push(new_room);
        }
    }
}

fn apply_horizontal_tunnel(map: &mut [TileType], x1: i32, x2: i32, y: i32) {
    for x in min(x1, x2)..=max(x1, x2) {
        let idx = xy_idx(x, y);
        if idx > 0 && idx < 80 * 50 {
            map[idx] = TileType::Floor;
        }
    }
}

fn apply_vertical_tunnel(map: &mut [TileType], y1: i32, y2: i32, x: i32) {
    for y in min(y1, y2)..=max(y1, y2) {
        let idx = xy_idx(x, y);
        if idx > 0 && idx < 80 * 50 {
            map[idx] = TileType::Floor;
        }
    }
}

pub fn draw_room_number(number: i32, ctx: &mut BTerm, x: i32, y: i32) {
    ctx.print_color_right(x, y, GRAY100, BLACK, number);
}

/// Draw a map base on TileType
pub fn draw_map(map: &[TileType], rooms: &[Rect], ctx: &mut BTerm) {
    let mut x = 0;
    let mut y = 0;

    for tile in map.iter() {
        match tile {
            TileType::Floor => {
                ctx.set(x, y, BLACK, BLACK, to_cp437(' '));
            }
            TileType::Wall => {
                ctx.set(x, y, GRAY, BLACK, to_cp437('#'));
            }
            TileType::Tree => {
                ctx.set(x, y, GREEN4, BLACK, to_cp437('|'));
            }
        }
        x += 1;
        if x > 79 {
            x = 0;
            y += 1;
        }
    }

    for (i, room) in rooms.iter().enumerate() {
        let (x, y) = room.center();
        ctx.print_color(x, y, GRAY100, BLACK, format!("{}", i));
    }
}
