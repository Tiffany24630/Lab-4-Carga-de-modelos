mod color;
mod framebuffer;
mod obj;
mod triangle;

use color::Color;
use framebuffer::{Framebuffer, SCREEN_HEIGHT, SCREEN_WIDTH};
use obj::{Model, Vertex};
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::time::Duration;
use triangle::{triangle, Point};

const MODEL_PATH: &str = "assets/nave_espacial.obj";

fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video = sdl_context.video()?;

    let window = video
        .window(
            "Rust Software OBJ Renderer",
            SCREEN_WIDTH as u32,
            SCREEN_HEIGHT as u32,
        )
        .position_centered()
        .build()
        .map_err(|e| e.to_string())?;

    let mut canvas = window
        .into_canvas()
        .build()
        .map_err(|e| e.to_string())?;

    let mut event_pump = sdl_context.event_pump()?;

    let model = Model::load(MODEL_PATH)
        .map_err(|e| format!("No se pudo cargar '{}': {}", MODEL_PATH, e))?;

    println!("Modelo cargado correctamente.");
    println!("Vertices: {}", model.vertices.len());
    println!("Triangulos: {}", model.faces.len());

    let (center_x, center_y, model_center, scale) = calculate_transform(&model);
    let mut rotation_x = 0.0_f32;
    let mut rotation_y = 0.0_f32;
    let mut zoom = 1.0_f32;

    let mut framebuffer = Framebuffer::new(SCREEN_WIDTH, SCREEN_HEIGHT);

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,

                Event::KeyDown {
                    keycode: Some(keycode),
                    repeat: false,
                    ..
                } => match keycode {
                    Keycode::Left => rotation_y -= 0.12,
                    Keycode::Right => rotation_y += 0.12,
                    Keycode::Up | Keycode::W => rotation_x -= 0.12,
                    Keycode::Down | Keycode::S => rotation_x += 0.12,
                    Keycode::Equals | Keycode::Plus => zoom = (zoom * 1.15).min(5.0),
                    Keycode::Minus => zoom = (zoom / 1.15).max(0.2),
                    Keycode::R => {
                        rotation_x = 0.0;
                        rotation_y = 0.0;
                        zoom = 1.0;
                    }
                    _ => {}
                },

                _ => {}
            }
        }

        framebuffer.clear(Color::new(24, 28, 36));

        render(
            &mut framebuffer,
            &model,
            center_x,
            center_y,
            model_center,
            scale,
            rotation_x,
            rotation_y,
            zoom,
            Color::new(156, 177, 194),
        );

        framebuffer.present(&mut canvas)?;

        std::thread::sleep(Duration::from_millis(16));
    }

    Ok(())
}

fn render(
    framebuffer: &mut Framebuffer,
    model: &Model,
    center_x: f32,
    center_y: f32,
    model_center: Vertex,
    scale: f32,
    rotation_x: f32,
    rotation_y: f32,
    zoom: f32,
    color: Color,
) {
    for face in &model.faces {
        let [i0, i1, i2] = face.indices;
        let v0 = model.vertices[i0];
        let v1 = model.vertices[i1];
        let v2 = model.vertices[i2];

        let p0 = world_to_screen(v0, model_center, center_x, center_y, scale, rotation_x, rotation_y, zoom);
        let p1 = world_to_screen(v1, model_center, center_x, center_y, scale, rotation_x, rotation_y, zoom);
        let p2 = world_to_screen(v2, model_center, center_x, center_y, scale, rotation_x, rotation_y, zoom);

        let face_color = shade_face(v0, v1, v2, color);
        triangle(framebuffer, p0, p1, p2, face_color);
    }
}

fn world_to_screen(
    vertex: Vertex,
    model_center: Vertex,
    center_x: f32,
    center_y: f32,
    scale: f32,
    rotation_x: f32,
    rotation_y: f32,
    zoom: f32,
) -> Point {
    let x = vertex.x - model_center.x;
    let y = vertex.y - model_center.y;
    let z = vertex.z - model_center.z;

    let (sin_y, cos_y) = rotation_y.sin_cos();
    let rotated_x = x * cos_y + z * sin_y;
    let rotated_z = -x * sin_y + z * cos_y;

    let (sin_x, cos_x) = rotation_x.sin_cos();
    let rotated_y = y * cos_x - rotated_z * sin_x;
    let rotated_z = y * sin_x + rotated_z * cos_x;

    Point {
        x: (center_x + rotated_x * scale * zoom).round() as i32,
        y: (center_y - rotated_y * scale * zoom).round() as i32,
        z: -rotated_z,
    }
}

fn shade_face(v0: Vertex, v1: Vertex, v2: Vertex, base: Color) -> Color {
    let ab = (v1.x - v0.x, v1.y - v0.y, v1.z - v0.z);
    let ac = (v2.x - v0.x, v2.y - v0.y, v2.z - v0.z);
    let normal = (
        ab.1 * ac.2 - ab.2 * ac.1,
        ab.2 * ac.0 - ab.0 * ac.2,
        ab.0 * ac.1 - ab.1 * ac.0,
    );

    let length = (normal.0 * normal.0 + normal.1 * normal.1 + normal.2 * normal.2).sqrt();
    if length == 0.0 {
        return base;
    }

    let light = (-0.4_f32, 0.6_f32, 1.0_f32);
    let light_length = (light.0 * light.0 + light.1 * light.1 + light.2 * light.2).sqrt();
    let diffuse = ((normal.0 * light.0 + normal.1 * light.1 + normal.2 * light.2)
        / (length * light_length))
        .abs();
    let intensity = 0.25 + 0.75 * diffuse;

    Color::new(
        (base.r as f32 * intensity) as u8,
        (base.g as f32 * intensity) as u8,
        (base.b as f32 * intensity) as u8,
    )
}

fn calculate_transform(model: &Model) -> (f32, f32, Vertex, f32) {
    if model.vertices.is_empty() {
        return (
            SCREEN_WIDTH as f32 / 2.0,
            SCREEN_HEIGHT as f32 / 2.0,
            Vertex { x: 0.0, y: 0.0, z: 0.0 },
            1.0,
        );
    }

    let mut min_x = f32::INFINITY;
    let mut max_x = f32::NEG_INFINITY;
    let mut min_y = f32::INFINITY;
    let mut max_y = f32::NEG_INFINITY;
    let mut min_z = f32::INFINITY;
    let mut max_z = f32::NEG_INFINITY;

    for vertex in &model.vertices {
        min_x = min_x.min(vertex.x);
        max_x = max_x.max(vertex.x);
        min_y = min_y.min(vertex.y);
        max_y = max_y.max(vertex.y);
        min_z = min_z.min(vertex.z);
        max_z = max_z.max(vertex.z);
    }

    let model_center_x = (min_x + max_x) / 2.0;
    let model_center_y = (min_y + max_y) / 2.0;
    let model_center_z = (min_z + max_z) / 2.0;

    let radius = model
        .vertices
        .iter()
        .map(|vertex| {
            let dx = vertex.x - model_center_x;
            let dy = vertex.y - model_center_y;
            let dz = vertex.z - model_center_z;
            (dx * dx + dy * dy + dz * dz).sqrt()
        })
        .fold(0.0001_f32, f32::max);

    // Se usa una esfera envolvente para que el modelo también quepa al rotarlo.
    let available_size = (SCREEN_WIDTH.min(SCREEN_HEIGHT) as f32) * 0.72;
    let scale = available_size / (2.0 * radius);

    (
        SCREEN_WIDTH as f32 / 2.0,
        SCREEN_HEIGHT as f32 / 2.0,
        Vertex {
            x: model_center_x,
            y: model_center_y,
            z: model_center_z,
        },
        scale,
    )
}
