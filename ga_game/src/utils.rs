pub mod collider {
    use crate::prelude::{Cercle, Position};

    pub fn is_collision(ap: &Position, ac: &Cercle, bp: &Position, bc: &Cercle) -> bool {
        let Position(pos_a) = ap;
        let Position(pos_b) = bp;

        let distance_x = pos_a.x - pos_b.x;
        let distance_y = pos_a.y - pos_b.y;
        let distance_sqr = distance_x * distance_x + distance_y * distance_y;
        let total_radius = ac.0 + bc.0;
        let total_radius_sqr = total_radius * total_radius;

        distance_sqr <= total_radius_sqr
    }
}
