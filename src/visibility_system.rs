use super::{FieldOfView, Map, Position};
use bracket_lib::prelude::{Point, field_of_view};
use specs::prelude::*;

pub struct VisibilitySystem {}

impl<'a> System<'a> for VisibilitySystem {
    type SystemData = (
        ReadExpect<'a, Map>,
        WriteStorage<'a, FieldOfView>,
        WriteStorage<'a, Position>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (map, mut viewshed, pos) = data;
        for (viewshed, pos) in (&mut viewshed, &pos).join() {
            viewshed.visuble_tiles.clear();
            viewshed.visuble_tiles = field_of_view(Point::new(pos.x, pos.y), viewshed.range, &*map);
            viewshed
                .visuble_tiles
                .retain(|p| p.x >= 0 && p.x < map.width && p.y >= 0 && p.y < map.height);
        }
    }
}
