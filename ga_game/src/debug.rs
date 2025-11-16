use ga_ecs::prelude::EntityManager;

use crate::{Position, Renderable};

const HEIGHT: usize = 40;
const WIDTH: usize = 80;
const LENGTH: usize = WIDTH * HEIGHT;

pub(crate) fn draw(manager: &EntityManager) {
    let mut buffer = vec![None; LENGTH];
    let mut output = String::with_capacity((LENGTH + HEIGHT) * 2);

    for (id, position) in manager.components::<Position>() {
        let Some(renderable) = manager.get_first::<Renderable>(id) else {
            continue;
        };

        let px = (position.0.x + WIDTH as f32 / 2.0).round() as i32;
        let py = (position.0.y + HEIGHT as f32 / 2.0).round() as i32;

        if px < 0 || py < 0 || px >= WIDTH as i32 || py >= HEIGHT as i32 {
            continue;
        }

        if let Some(character) = buffer.get_mut((py * WIDTH as i32 + px) as usize) {
            *character = Some(renderable.0);
        }
    }

    for y in 0..HEIGHT {
        for c in &buffer[y * WIDTH..(y + 1) * WIDTH] {
            match c {
                Some(character) => output.push(*character),
                None => {
                    output.push_str("  ");
                }
            }
        }

        output.push('\n');
    }

    print!("{}", output);
}
