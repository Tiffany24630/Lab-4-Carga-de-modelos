use std::{fs, io, path::Path};

#[derive(Clone, Copy, Debug)]
pub struct Vertex {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

#[derive(Clone, Copy, Debug)]
pub struct Face {
    pub indices: [usize; 3],
}

#[derive(Debug)]
pub struct Model {
    pub vertices: Vec<Vertex>,
    pub faces: Vec<Face>,
}

impl Model {
    pub fn load<P: AsRef<Path>>(path: P) -> io::Result<Self> {
        let contents = fs::read_to_string(path)?;
        let mut vertices = Vec::new();
        let mut faces = Vec::new();

        for line in contents.lines() {
            let line = line.trim();

            if line.is_empty() || line.starts_with('#') {
                continue;
            }

            let mut parts = line.split_whitespace();
            let Some(kind) = parts.next() else {
                continue;
            };

            match kind {
                "v" => {
                    let x: f32 = parts.next().unwrap_or("0").parse().unwrap_or(0.0);
                    let y: f32 = parts.next().unwrap_or("0").parse().unwrap_or(0.0);
                    let z: f32 = parts.next().unwrap_or("0").parse().unwrap_or(0.0);

                    vertices.push(Vertex { x, y, z });
                }

                "f" => {
                    let tokens: Vec<&str> = parts.collect();

                    if tokens.len() < 3 {
                        continue;
                    }

                    let first = parse_index(tokens[0], vertices.len());
                    for i in 1..tokens.len() - 1 {
                        let second = parse_index(tokens[i], vertices.len());
                        let third = parse_index(tokens[i + 1], vertices.len());

                        if let (Some(a), Some(b), Some(c)) = (first, second, third) {
                            faces.push(Face {
                                indices: [a, b, c],
                            });
                        }
                    }
                }
                _ => {}
            }
        }
        Ok(Self { vertices, faces })
    }
}

fn parse_index(token: &str, vertex_count: usize) -> Option<usize> {
    let index_text = token.split('/').next()?;
    let index: isize = index_text.parse().ok()?;

    if index > 0 {
        let index = index as usize - 1;
        (index < vertex_count).then_some(index)

    } else if index < 0 {
        let index = vertex_count as isize + index;
        (index >= 0 && (index as usize) < vertex_count).then_some(index as usize)
        
    } else {
        None
    }
}