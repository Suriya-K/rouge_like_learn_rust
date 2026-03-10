use super::{Rect, World};
use bracket_lib::color::*;
use bracket_lib::prelude::{Algorithm2D, BTerm, BaseMap, Point, RandomNumberGenerator, to_cp437};
use std::cmp::{max, min};

#[derive(PartialEq, Clone, Copy)]
pub enum TileType {
    Wall,
    Floor,
    Tree,
}

pub struct Map {
    pub tiles: Vec<TileType>,
    pub rooms: Vec<Rect>,
    pub width: i32,
    pub height: i32,
    pub revealed_tiles: Vec<bool>,
    pub visible_tiles: Vec<bool>,
}

impl Map {
    /// Translate 2D area map into flat vector
    /// For exmaple instead of store into map(5)(2) it will use map(156)
    /// return usize
    pub fn xy_idx(&self, x: i32, y: i32) -> usize {
        (y as usize * 80) + x as usize
    }

    fn apply_room_to_map(&mut self, room: &Rect) {
        for y in room.y1 + 1..=room.y2 {
            for x in room.x1 + 1..=room.x2 {
                let idx = self.xy_idx(x, y);
                self.tiles[idx] = TileType::Floor;
            }
        }
    }

    fn apply_horizontal_tunnel(&mut self, x1: i32, x2: i32, y: i32) {
        for x in min(x1, x2)..=max(x1, x2) {
            let idx = self.xy_idx(x, y);
            if idx > 0 && idx < 80 * 50 {
                self.tiles[idx] = TileType::Floor;
            }
        }
    }

    fn apply_vertical_tunnel(&mut self, y1: i32, y2: i32, x: i32) {
        for y in min(y1, y2)..=max(y1, y2) {
            let idx = self.xy_idx(x, y);
            if idx > 0 && idx < 80 * 50 {
                self.tiles[idx] = TileType::Floor;
            }
        }
    }

    pub fn new_map_rooms_and_corridors(&mut self) {
        const MAX_ROOMS: i32 = 20;
        const MIN_ROOM_SIZE: i32 = 6;
        const MAX_ROOM_SIZE: i32 = 12;
        self.generate_room(MIN_ROOM_SIZE, MAX_ROOM_SIZE, MAX_ROOMS);
    }

    pub fn generate_room(&mut self, min: i32, max: i32, limit: i32) {
        let mut rng = RandomNumberGenerator::new();
        for _ in 0..limit {
            let width = rng.range(min, max);
            let hight = rng.range(min, max);
            let pos_x = rng.roll_dice(1, 80 - width - 1) - 1;
            let pos_y = rng.roll_dice(1, 50 - hight - 1) - 1;
            let new_room = Rect::new(pos_x, pos_y, width, hight);
            let mut can_place_room = true;
            for other_room in self.rooms.iter() {
                if new_room.intersect(other_room) {
                    can_place_room = false
                }
            }
            if can_place_room {
                self.apply_room_to_map(&new_room);
                if !self.rooms.is_empty() {
                    let (new_x, new_y) = new_room.center();
                    let (prev_x, prev_y) = self.rooms[self.rooms.len() - 1].center();
                    if rng.range(0, 2) == 1 {
                        self.apply_horizontal_tunnel(prev_x, new_x, prev_y);
                        self.apply_vertical_tunnel(prev_y, new_y, new_x);
                    } else {
                        self.apply_horizontal_tunnel(prev_x, new_x, new_y);
                        self.apply_vertical_tunnel(prev_y, new_y, prev_x);
                    }
                }
                self.rooms.push(new_room);
            }
        }
    }

    /// Create a map with solid boundaries and random place tree
    pub fn new_map_test(&self) -> Vec<TileType> {
        //declear vector type TileType to create map vector array
        let mut map = vec![TileType::Floor; 80 * 50];

        // for loop to create horizontal wall
        for x in 0..80 {
            map[self.xy_idx(x, 0)] = TileType::Wall;
            map[self.xy_idx(x, 49)] = TileType::Wall;
        }

        // for loop to create vertical wall
        for y in 0..50 {
            map[self.xy_idx(0, y)] = TileType::Wall;
            map[self.xy_idx(79, y)] = TileType::Wall;
        }

        // using random to create tree inside area map
        let mut rand = RandomNumberGenerator::new();
        for _i in 0..100 {
            let x = rand.roll_dice(1, 78);
            let y = rand.roll_dice(1, 48);
            let idx = self.xy_idx(x, y);
            if idx != self.xy_idx(40, 25) {
                map[idx] = TileType::Tree;
            }
        }

        map
    }
}

impl Algorithm2D for Map {
    fn dimensions(&self) -> Point {
        Point::new(self.width, self.height)
    }
}

impl BaseMap for Map {
    fn is_opaque(&self, idx: usize) -> bool {
        self.tiles[idx] == TileType::Wall
    }
}

pub fn draw_room_number(number: i32, ctx: &mut BTerm, x: i32, y: i32) {
    ctx.print_color_right(x, y, GRAY100, BLACK, number);
}

/// Draw a map base on TileType
pub fn draw_map(ecs: &World, ctx: &mut BTerm) {
    let map = ecs.fetch::<Map>();

    let mut x = 0;
    let mut y = 0;

    for (idx, tile) in map.tiles.iter().enumerate() {
        if map.revealed_tiles[idx] {
            let glyph;
            match tile {
                TileType::Floor => {
                    glyph = to_cp437('.');
                    ctx.set(x, y, WHITE, BLACK, glyph);
                }
                TileType::Wall => {
                    glyph = to_cp437('#');
                    ctx.set(x, y, GRAY, BLACK, glyph);
                }
                TileType::Tree => {
                    glyph = to_cp437('|');
                    ctx.set(x, y, GREEN4, BLACK, glyph);
                }
            }

            if !map.visible_tiles[idx] {
                ctx.set(x, y, GRAY10, BLACK, glyph);
            }
        }

        x += 1;
        if x > 79 {
            x = 0;
            y += 1;
        }
    }

    for (i, room) in map.rooms.iter().enumerate() {
        let (x, y) = room.center();
        ctx.print_color(x, y, GRAY100, BLACK, format!("{}", i));
    }
}
