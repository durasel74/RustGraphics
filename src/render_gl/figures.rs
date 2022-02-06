#![allow(dead_code)]

use std::f32::consts::PI;
use super::Mesh;

pub fn triangle() -> Mesh {
    let vertices = vec![
        -0.5, -0.5, 0.0,   0.8, 0.2, 0.8,  0.0, 0.0,
         0.0,  0.5, 0.0,   0.9, 0.2, 0.8,  0.0, 0.0,
         0.5, -0.5, 0.0,   0.8, 0.2, 1.0,  0.0, 0.0,
    ];
    let indices: Vec<u16> = vec![
        0, 1, 2,
    ];
    Mesh::from_verteices(vertices, indices)
}

pub fn square() -> Mesh {
    let vertices = vec![
        -0.5,  0.5, 0.0,   1.0, 1.0, 1.0,   0.0, 1.0,
         0.5,  0.5, 0.0,   0.9, 0.9, 0.9,   1.0, 1.0,
         0.5, -0.5, 0.0,   0.8, 0.8, 0.8,   1.0, 0.0,
        -0.5, -0.5, 0.0,   0.9, 0.9, 0.9,   0.0, 0.0,
    ];
    let indices: Vec<u16> = vec![
        0, 1, 2,
        2, 3, 0,
    ];
    Mesh::from_verteices(vertices, indices)
}

pub fn create_circle(vertex_count: u16, radius: u32) -> Mesh {
    let normal_radius = (radius as f32) / 100.0;
    let mut x: f32 = 0.0;
    let mut y: f32 = 0.0;
    let mut vertices: Vec<f32> = vec![x, y, 0.0,  0.4, 0.2, 0.7,  x, y];
    let mut indices: Vec<u16> = vec![];
    
    let vertex_count_f32 = vertex_count as f32;
    let mut vertex_index = 1u16;
    for i in 0..vertex_count {
        let angle = (2.0 * PI * i as f32) / vertex_count_f32;
        x = normal_radius * angle.cos();
        y = normal_radius * angle.sin();
        let gradient = ((-x + y) / 2.0) + normal_radius;
        vec_push_range(&mut vertices, vec![x, y, 0.0,  0.4, 0.2, gradient + 0.2,  x, y]);

        let mut next_index = (vertex_index + 1) % vertex_count;
        next_index = next_index + vertex_count * (next_index == 0) as u16;
        vec_push_range(&mut indices, vec![vertex_index, next_index, 0]);
        vertex_index += 1;
    }
    Mesh::from_verteices(vertices, indices)
}

pub fn create_thor(vertex_count: u16, radius: u32, inner_radius: u32) -> Mesh {
    let normal_radius = (radius as f32) / 100.0;
    let normal_inner_radius = (inner_radius as f32) / 100.0;
    let mut x: f32;
    let mut y: f32;
    let mut vertices: Vec<f32> = vec![];
    let mut indices: Vec<u16> = vec! [];

    let vertex_count_f32 = vertex_count as f32;
    let mut vertex_index = 0u16;
    for i in 0..vertex_count {
        let angle = (2.0 * PI * i as f32) / vertex_count_f32;
        x = normal_radius * angle.cos();
        y = normal_radius * angle.sin();
        let gradient = ((-x + y) / 2.0) + normal_radius;
        vec_push_range(&mut vertices, vec![x, y, 0.0,  0.2, gradient + 0.2, 1.0 - gradient / 2.0,  x, y]);
        x = normal_inner_radius * angle.cos();
        y = normal_inner_radius * angle.sin();
        let gradient = ((-x + y) / 2.0) + normal_radius;
        vec_push_range(&mut vertices, vec![x, y, 0.0,  0.1, gradient + 0.1, 0.9 - gradient / 2.0,  x, y]);

        let next_index = (vertex_index + 2) % (vertex_count * 2);
        vec_push_range(&mut indices, vec![vertex_index, next_index, next_index + 1]);
        vec_push_range(&mut indices, vec![vertex_index, vertex_index + 1, next_index + 1]);
        vertex_index += 2;
    }
    Mesh::from_verteices(vertices, indices)
}

pub fn cube() -> Mesh {
    let vertices = vec![
        -1.0,  1.0, -1.0,  0.9, 0.4, 0.7,  0.0, 0.0,
         1.0,  1.0, -1.0,  1.0, 0.4, 0.6,  0.0, 0.0,
         1.0, -1.0, -1.0,  0.9, 0.4, 0.7,  0.0, 0.0,
        -1.0, -1.0, -1.0,  0.8, 0.4, 0.8,  0.0, 0.0,

        -1.0,  1.0,  1.0,  0.7, 0.4, 0.9,  0.0, 0.0,
         1.0,  1.0,  1.0,  0.8, 0.4, 0.8,  0.0, 0.0,
         1.0, -1.0,  1.0,  0.7, 0.4, 0.9,  0.0, 0.0,
        -1.0, -1.0,  1.0,  0.6, 0.4, 1.0,  0.0, 0.0,
    ];
    let indices: Vec<u16> = vec![
        0, 1, 2,
        0, 2, 3,

        2, 1, 5,
        2, 5, 6,

        3, 2, 6,
        3, 6, 7,

        0, 3, 7,
        0, 7, 4,

        1, 0, 4,
        1, 4, 5,

        6, 5, 4,
        6, 4, 7,
    ];
    Mesh::from_verteices(vertices, indices)
}

pub fn tetrahedron() -> Mesh {
    let vertices = vec![
         0.0,  -0.5, -1.0,                0.9, 0.4, 0.7,  0.0, 0.0,
         1.5f32.sqrt(), -0.5, 0.5,        0.7, 0.4, 0.9,  0.0, 0.0,
         -1.5f32.sqrt(), -0.5,  0.5,      0.8, 0.4, 0.8,  0.0, 0.0,
         0.0,  2.0f32.sqrt() - 0.5, 0.0,  1.0, 0.4, 0.6,  0.0, 0.0,
    ];
    let indices: Vec<u16> = vec![
        0, 1, 2,
        0, 3, 1,
        2, 1, 3,
        0, 2, 3,
    ];
    Mesh::from_verteices(vertices, indices)
}

pub fn vec_push_range<T>(vect: &mut Vec<T>, values: Vec<T>) {
    for i in values { vect.push(i); }
}
