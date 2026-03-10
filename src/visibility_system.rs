use super::{FieldOfView, Map, Player, Position};
use bracket_lib::prelude::{Point, field_of_view};
use specs::prelude::*;

pub struct VisibilitySystem {}

impl<'a> System<'a> for VisibilitySystem {
    type SystemData = (
        WriteExpect<'a, Map>,
        WriteStorage<'a, FieldOfView>,
        WriteStorage<'a, Position>,
        ReadStorage<'a, Player>,
        Entities<'a>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (mut map, mut fovs, pos, player, entities) = data;
        for (fov, pos, ent) in (&mut fovs, &pos, &entities).join() {
            if fov.dirty {
                println!("[SYS] fov calculated for entitiy {:?}", ent);
                fov.dirty = false;
                fov.visuble_tiles.clear();
                fov.visuble_tiles = field_of_view(Point::new(pos.x, pos.y), fov.range, &*map);
                fov.visuble_tiles
                    .retain(|p| p.x >= 0 && p.x < map.width && p.y >= 0 && p.y < map.height);

                let _p = player.get(ent);
                if let Some(_p) = _p {
                    for t in map.visible_tiles.iter_mut() {
                        *t = false
                    }
                    for view in fov.visuble_tiles.iter() {
                        let idx = map.xy_idx(view.x, view.y);
                        map.revealed_tiles[idx] = true;
                        map.visible_tiles[idx] = true;
                    }
                }
            }
        }
    }
}
