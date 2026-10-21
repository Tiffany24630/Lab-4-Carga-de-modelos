use crate::{color::Color, framebuffer::Framebuffer};

#[derive(Clone, Copy, Debug)]
pub struct Point {
    pub x: i32,
    pub y: i32,
    pub z: f32,
}

pub fn triangle(
    framebuffer: &mut Framebuffer,
    p0: Point,
    p1: Point,
    p2: Point,
    color: Color,
) {
    let min_x = p0.x.min(p1.x).min(p2.x).max(0);
    let max_x = p0.x.max(p1.x).max(p2.x).min(framebuffer.width as i32 - 1);
    let min_y = p0.y.min(p1.y).min(p2.y).max(0);
    let max_y = p0.y.max(p1.y).max(p2.y).min(framebuffer.height as i32 - 1);

    let area = edge(p0, p1, p2);

    if area == 0 {
        return;
    }

    for y in min_y..=max_y {
        for x in min_x..=max_x {
            let p = Point { x, y, z: 0.0 };

            let w0 = edge(p1, p2, p);
            let w1 = edge(p2, p0, p);
            let w2 = edge(p0, p1, p);

            if (w0 >= 0 && w1 >= 0 && w2 >= 0)
                || (w0 <= 0 && w1 <= 0 && w2 <= 0)
            {
                let depth = (w0 as f32 * p0.z + w1 as f32 * p1.z + w2 as f32 * p2.z)
                    / area as f32;
                framebuffer.set_pixel(x, y, depth, color);
            }
        }
    }
}

fn edge(a: Point, b: Point, c: Point) -> i64 {
    let ab_x = (b.x - a.x) as i64;
    let ab_y = (b.y - a.y) as i64;
    let ac_x = (c.x - a.x) as i64;
    let ac_y = (c.y - a.y) as i64;

    ab_x * ac_y - ab_y * ac_x
}