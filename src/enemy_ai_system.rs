use std::fmt::format;

use bracket_lib::prelude::{Point, console};
use specs::{Join, Read, ReadExpect, ReadStorage, System};

use crate::{EntityName, FieldOfView, Monster, Position};

pub struct MonsterAI {}

impl<'a> System<'a> for MonsterAI {
    type SystemData = (
        ReadStorage<'a, FieldOfView>,
        ReadStorage<'a, Position>,
        ReadStorage<'a, Monster>,
        ReadExpect<'a, Point>,
        ReadStorage<'a, EntityName>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (fovs, pos, mon, player_pos, name) = data;
        for (fov, p, m, entity_name) in (&fovs, &pos, &mon, &name).join() {
            if fov.visuble_tiles.contains(&*player_pos) {
                console::log(&format!("[SYS] {} See you!", entity_name.name));
            }
        }
    }
}
