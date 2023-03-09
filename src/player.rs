use crate::prelude::*;

pub struct Player {
    position: Point,
}

impl Player {
    pub fn new(position: Point) -> Self {
        Self { position }
    }

    pub fn render(&mut self, ctx: &mut BTerm) {
        ctx.set(
            self.position.x,
            self.position.y,
            WHITE,
            BLACK,
            to_cp437('@'),
        )
    }

    pub fn update(&mut self, ctx: &mut BTerm, map: &Map) {
        if let Some(key) = ctx.key {
            let delta = match key {
                VirtualKeyCode::Left | VirtualKeyCode::H => Point::new(-1, 0),
                VirtualKeyCode::Right | VirtualKeyCode::L => Point::new(1, 0),
                VirtualKeyCode::Down | VirtualKeyCode::J => Point::new(0, 1),
                VirtualKeyCode::Up | VirtualKeyCode::K => Point::new(0, -1),
                _ => {
                    println!("{:#?}", key);
                    Point::new(0, 0)
                }
            };
            let new_position = self.position + delta;
            if map.can_enter_tile(new_position) {
                self.position = new_position
            }
        }
    }
}
