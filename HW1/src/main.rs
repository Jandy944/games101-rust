
use glam::Vec3;

mod triangle;
mod rasterizer;
use rasterizer::Rasterizer;

fn main() {

    let mut r = Rasterizer::new(700, 700);

    let eye_pos = Vec3::new(0.0, 0.0, 5.0);
    let pos = vec![
        Vec3::new(2.0, 0.0, -2.0),
        Vec3::new(0.0, 2.0, -1.0),
        Vec3::new(-2.0, 0.0, -2.0)
    ];

    let pos_id = r.load_positions(pos);
}
