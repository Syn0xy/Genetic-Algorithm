use ga_ecs::prelude::EntityManager;

use crate::prelude::Position;

pub(crate) fn draw(manager: EntityManager) {
    let height = 50i32;
    let width = height;

    let mut buffer = vec!['.'; (width * height) as usize];

    for (_, position) in manager.components::<Position>() {
        let px = (position.0.x + width as f32 / 2.0) as i32;
        let py = (position.0.y + height as f32 / 2.0) as i32;
        let index = (py * width + px) as usize;

        if index < buffer.len() && px >= 0 && px < width && py >= 0 && py < height {
            buffer[index] = 'X';
        }
    }

    for y in 0..height {
        for x in 0..width {
            print!("{} ", buffer[(y * width + x) as usize]);
        }
        println!();
    }
}
