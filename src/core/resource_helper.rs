use std::{fs, path::Path};

use crate::{math::numerics::{float2::Float2, float3::Float3, float4::Float4}, types::{mesh::Mesh, texture::Texture}};

pub fn load_mesh(path: &str) -> Mesh {
    let obj_string =
        fs::read_to_string(path).expect("Failed to read obj file");

    load_obj(&obj_string)
}

pub fn load_texture(path: &str) -> Texture {
    load_texture_png(path)
}

fn load_obj(model_string: &str) -> Mesh {
    let lines = split_by_line(model_string, true);

    let mut vertex_positions = Vec::<Float3>::new();
    let mut normals_src = Vec::<Float3>::new();
    let mut uvs_src = Vec::<Float2>::new();

    let mut positions = Vec::<Float3>::new();
    let mut normals = Vec::<Float3>::new();
    let mut uvs = Vec::<Float2>::new();
    let mut indices = Vec::<u32>::new();

    for line in lines {
        if line.starts_with("v ") {
            let p: Vec<&str> = line[2..].split_whitespace().collect();
            vertex_positions.push(Float3 {
                x: p[0].parse().unwrap(),
                y: p[1].parse().unwrap(),
                z: p[2].parse().unwrap(),
            });
        }
        else if line.starts_with("vn ") {
            let p: Vec<&str> = line[3..].split_whitespace().collect();
            normals_src.push(Float3 {
                x: p[0].parse().unwrap(),
                y: p[1].parse().unwrap(),
                z: p[2].parse().unwrap(),
            });
        }
        else if line.starts_with("vt ") {
            let p: Vec<&str> = line[3..].split_whitespace().collect();
            uvs_src.push(Float2 {
                x: p[0].parse().unwrap(),
                y: p[1].parse().unwrap(),
            });
        }
        else if line.starts_with("f ") {
            let groups: Vec<&str> =
                line[2..].split_whitespace().collect();

            let mut face_indices = Vec::<u32>::new();

            for g in groups {
                let e: Vec<&str> = g.split('/').collect();

                let v = e.get(0).and_then(|s| s.parse::<usize>().ok());
                let t = e.get(1).and_then(|s| s.parse::<usize>().ok());
                let n = e.get(2).and_then(|s| s.parse::<usize>().ok());

                let pos = v.map(|i| vertex_positions[i - 1]).unwrap_or_default();
                let norm = n.map(|i| normals_src[i - 1]).unwrap_or_default();
                let uv = t.map(|i| uvs_src[i - 1]).unwrap_or_default();

                let index = positions.len() as u32;

                positions.push(pos);
                normals.push(norm);
                uvs.push(uv);

                face_indices.push(index);
            }

            // Fan triangulation
            for i in 2..face_indices.len() {
                indices.push(face_indices[0]);
                indices.push(face_indices[i - 1]);
                indices.push(face_indices[i]);
            }
        }
    }

    Mesh::new(positions, indices, normals, uvs)
}

fn load_texture_png<P: AsRef<Path>>(path: P) -> Texture {
    let img = image::open(path).expect("Failed to load texture");
    let rgba = img.to_rgba8();
    let (width, height) = rgba.dimensions();

    let mut image_data = vec![vec![Float4::ZERO; height as usize]; width as usize];

    for y in 0..height {
        for x in 0..width {
            let [r, g, b, a] = rgba.get_pixel(x, y).0;

            image_data[x as usize][y as usize] = Float4 {
                x: r as f32 / 255.0,
                y: g as f32 / 255.0,
                z: b as f32 / 255.0,
                w: a as f32 / 255.0,
            };
        }
    }

    Texture::new(image_data)
}

pub fn split_by_line(text: &str, remove_empty: bool) -> Vec<&str> {
    text.lines().filter(|l| !remove_empty || !l.is_empty()).collect()
}
