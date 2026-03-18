use std::fmt::format;

use bracket_lib::prelude::{DistanceAlg, Point, a_star_search, console};
use specs::{Join, ReadExpect, ReadStorage, System, WriteExpect, WriteStorage};

use crate::{EntityName, FieldOfView, Map, Monster, Position};

pub struct MonsterAI {}

impl<'a> System<'a> for MonsterAI {
    #[allow(clippy::type_complexity)]
    type SystemData = (
        WriteExpect<'a, Map>,
        WriteStorage<'a, FieldOfView>,
        WriteStorage<'a, Position>,
        ReadStorage<'a, Monster>,
        ReadExpect<'a, Point>,
        ReadStorage<'a, EntityName>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (mut map, mut field_of_view, mut position, monster, player_point, entity_name) = data;

        for (fov, mon, e_name, p) in
            (&mut field_of_view, &monster, &entity_name, &mut position).join()
        {
            let point = Point::new(p.x, p.y);
            let distance = DistanceAlg::Pythagoras.distance2d(point, *player_point);
            if distance < 1.5 {
                console::log(&format!("[SYS] {} Close to you!", e_name.name));
            } else if fov.visuble_tiles.contains(&*player_point) {
                let path = a_star_search(
                    map.xy_idx(p.x, p.y) as i32,
                    map.xy_idx(player_point.x, player_point.y) as i32,
                    &*map,
                );
                if path.success && path.steps.len() > 1 {
                    let mut idx = map.xy_idx(p.x, p.y);
                    map.blocked_tiles[idx] = false;
                    let (new_x, new_y) = map.idx_xy(path.steps[1]);
                    p.x = new_x;
                    p.y = new_y;
                    idx = map.xy_idx(p.x, p.y);
                    map.blocked_tiles[idx] = true;
                    fov.dirty = true;
                } else {
                    println!("I see you but i can't find a way");
                }
            }
        }
    }
}
