use bracket_lib::color::*;
use bracket_lib::prelude::{BTerm, RandomNumberGenerator, to_cp437};

#[derive(PartialEq, Clone, Copy)]
pub enum TileType {
    Wall,
    Floor,
    Tree,
}

pub fn xy_idx(x: i32, y: i32) -> usize {
    (y as usize * 80) + x as usize
}

pub fn new_map() -> Vec<TileType> {
    let mut map = vec![TileType::Floor; 80 * 50];
    for x in 0..80 {
        map[xy_idx(x, 0)] = TileType::Wall;
        map[xy_idx(x, 49)] = TileType::Wall;
    }

    for y in 0..50 {
        map[xy_idx(0, y)] = TileType::Wall;
        map[xy_idx(79, y)] = TileType::Wall;
    }

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

pub fn draw_map(map: &[TileType], ctx: &mut BTerm) {
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
}
