use crate::SystemState;

use super::{FieldOfView, Map, Player, Position, State, TileType};
use bracket_lib::prelude::{BTerm, Point, VirtualKeyCode};
use specs::prelude::*;
use std::cmp::{max, min};

pub fn move_player(delta_x: i32, delta_y: i32, ecs: &mut World) {
    let mut position = ecs.write_storage::<Position>();
    let mut player = ecs.write_storage::<Player>();
    let mut fovs = ecs.write_storage::<FieldOfView>();
    let map = ecs.fetch::<Map>();
    let mut player_pos = ecs.write_resource::<Point>();

    for (_player, pos, fov) in (&mut player, &mut position, &mut fovs).join() {
        let des_idx = map.xy_idx(pos.x + delta_x, pos.y + delta_y);
        if map.tiles[des_idx] != TileType::Wall && map.tiles[des_idx] != TileType::Tree {
            pos.x = min(79, max(0, pos.x + delta_x));
            //pos.x = pos.x.clamp(79, max(0, pos.x + delta_x));
            //pos.y = pos.y.clamp(49, max(0, pos.y + delta_y));
            pos.y = min(49, max(0, pos.y + delta_y));
            player_pos.x = pos.x;
            player_pos.y = pos.y;
            fov.dirty = true;
        }
    }
}

pub fn player_input(gs: &mut State, ctx: &mut BTerm) -> SystemState {
    match ctx.key {
        None => return SystemState::Paused,
        Some(key) => match key {
            VirtualKeyCode::Left | VirtualKeyCode::Numpad4 | VirtualKeyCode::H => {
                move_player(-1, 0, &mut gs.ecs);
            }
            VirtualKeyCode::Right | VirtualKeyCode::Numpad6 | VirtualKeyCode::L => {
                move_player(1, 0, &mut gs.ecs)
            }
            VirtualKeyCode::Up | VirtualKeyCode::Numpad8 | VirtualKeyCode::K => {
                move_player(0, -1, &mut gs.ecs)
            }
            VirtualKeyCode::Down | VirtualKeyCode::Numpad2 | VirtualKeyCode::J => {
                move_player(0, 1, &mut gs.ecs)
            }
            _ => return SystemState::Paused,
        },
    }
    SystemState::Running
}
