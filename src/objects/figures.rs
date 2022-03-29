#![allow(dead_code)]

use std::f32::consts::PI;
use super::Mesh;

// pub fn triangle() -> Mesh {
//     let vertices = vec![
//         -1.0, -1.0, 0.0,   0.8, 0.2, 0.8,  0.0, 0.0,
//          0.0,  1.0, 0.0,   0.9, 0.2, 0.8,  0.5, 1.0,
//          1.0, -1.0, 0.0,   0.8, 0.2, 1.0,  1.0, 0.0,
//     ];
//     let indices: Vec<u16> = vec![
//         0, 1, 2,
//     ];
//     Mesh::from_verteices(vertices, indices)
// }

// pub fn square() -> Mesh {
//     let vertices = vec![
//         -1.0,  1.0, 0.0,   1.0, 1.0, 1.0,   0.0, 1.0,
//          1.0,  1.0, 0.0,   0.9, 0.9, 0.9,   1.0, 1.0,
//          1.0, -1.0, 0.0,   0.8, 0.8, 0.8,   1.0, 0.0,
//         -1.0, -1.0, 0.0,   0.9, 0.9, 0.9,   0.0, 0.0,
//     ];
//     let indices: Vec<u16> = vec![
//         0, 1, 2,
//         2, 3, 0,
//     ];
//     Mesh::from_verteices(vertices, indices)
// }

// pub fn cube() -> Mesh {
//     let vertices = vec![
//         -1.0,  1.0, -1.0,  0.3, 0.2, 0.9,  0.0, 1.0,
//          1.0,  1.0, -1.0,  0.3, 0.2, 0.9,  1.0, 1.0,
//          1.0, -1.0, -1.0,  0.3, 0.2, 0.9,  1.0, 0.0,
//         -1.0, -1.0, -1.0,  0.3, 0.2, 0.9,  0.0, 0.0,

//         -1.0,  1.0,  1.0,  0.3, 0.2, 0.9,  0.0, 1.0,
//          1.0,  1.0,  1.0,  0.3, 0.2, 0.9,  1.0, 1.0,
//          1.0, -1.0,  1.0,  0.3, 0.2, 0.9,  1.0, 0.0,
//         -1.0, -1.0,  1.0,  0.3, 0.2, 0.9,  0.0, 0.0,
//     ];
//     let indices: Vec<u16> = vec![
//         0, 1, 2,
//         0, 2, 3,

//         2, 1, 5,
//         2, 5, 6,

//         3, 2, 6,
//         3, 6, 7,

//         0, 3, 7,
//         0, 7, 4,

//         1, 0, 4,
//         1, 4, 5,

//         6, 5, 4,
//         6, 4, 7,
//     ];
//     Mesh::from_verteices(vertices, indices)
// }

// pub fn camera() -> Mesh {
//     let vertices = vec![
//         -0.4,  0.4, -0.4,  1.0, 1.0, 1.0,  0.0, 1.0,
//          0.1,  0.1, -0.1,  0.7, 0.7, 0.7,  1.0, 1.0,
//          0.1, -0.1, -0.1,  0.7, 0.7, 0.7,  1.0, 0.0,
//         -0.4, -0.4, -0.4,  1.0, 1.0, 1.0,  0.0, 0.0,

//         -0.4,  0.4,  0.4,  1.0, 1.0, 1.0,  0.0, 1.0,
//          0.1,  0.1,  0.1,  0.7, 0.7, 0.7,  1.0, 1.0,
//          0.1, -0.1,  0.1,  0.7, 0.7, 0.7,  1.0, 0.0,
//         -0.4, -0.4,  0.4,  1.0, 1.0, 1.0,  0.0, 0.0,
//     ];
//     let indices: Vec<u16> = vec![
//         0, 1, 2,
//         0, 2, 3,

//         2, 1, 5,
//         2, 5, 6,

//         3, 2, 6,
//         3, 6, 7,

//         0, 3, 7,
//         0, 7, 4,

//         1, 0, 4,
//         1, 4, 5,

//         6, 5, 4,
//         6, 4, 7,
//     ];
//     Mesh::from_verteices(vertices, indices)
// }

// pub fn tetrahedron() -> Mesh {
//     let vertices = vec![
//          0.0,  -0.5, -1.0,                0.9, 0.4, 0.7,  0.0, 0.0,
//          1.5f32.sqrt(), -0.5, 0.5,        0.7, 0.4, 0.9,  0.0, 0.0,
//          -1.5f32.sqrt(), -0.5, 0.5,       0.8, 0.4, 0.8,  0.0, 0.0,

//          1.5f32.sqrt(), -0.5, 0.5,        0.7, 0.4, 0.9,  1.0, 0.0,
//          -1.5f32.sqrt(), -0.5, 0.5,       0.8, 0.4, 0.8,  0.0, 0.0,
//          0.0,  2.0f32.sqrt() - 0.5, 0.0,  1.0, 0.4, 0.6,  0.5, 1.0,

//          0.0,  -0.5, -1.0,                0.9, 0.4, 0.7,  0.0, 0.0,
//          -1.5f32.sqrt(), -0.5, 0.5,       0.8, 0.4, 0.8,  1.0, 0.0,
//          0.0,  2.0f32.sqrt() - 0.5, 0.0,  1.0, 0.4, 0.6,  0.5, 1.0,

//          0.0,  -0.5, -1.0,                0.9, 0.4, 0.7,  1.0, 0.0,
//          1.5f32.sqrt(), -0.5, 0.5,        0.7, 0.4, 0.9,  0.0, 0.0,
//          0.0,  2.0f32.sqrt() - 0.5, 0.0,  1.0, 0.4, 0.6,  0.5, 1.0,
//     ];
//     let indices: Vec<u16> = vec![
//         0, 1, 2,
//         3, 4, 5,
//         6, 7, 8,
//         9, 10, 11,
//     ];
//     Mesh::from_verteices(vertices, indices)
// }

// pub fn create_circle(vertex_count: u16, radius: u32) -> Mesh {
//     let normal_radius = (radius as f32) / 100.0;
//     let mut x: f32 = 0.0;
//     let mut y: f32 = 0.0;
//     let mut vertices: Vec<f32> = vec![x, y, 0.0,  0.4, 0.2, 0.7,  
//         x + 0.5, y + 0.5];
//     let mut indices: Vec<u16> = vec![];
    
//     let vertex_count_f32 = vertex_count as f32;
//     let mut vertex_index = 1u16;
//     for i in 0..vertex_count {
//         let angle = (2.0 * PI * i as f32) / vertex_count_f32;
//         x = normal_radius * angle.cos();
//         y = normal_radius * angle.sin();
//         let gradient = ((-x + y) / 2.0) + normal_radius;
//         vec_push_range(&mut vertices, vec![x, y, 0.0,  0.4, 0.2, gradient + 0.2,  
//             x + 0.5, y + 0.5]);

//         let mut next_index = (vertex_index + 1) % vertex_count;
//         next_index = next_index + vertex_count * (next_index == 0) as u16;
//         vec_push_range(&mut indices, vec![vertex_index, next_index, 0]);
//         vertex_index += 1;
//     }
//     Mesh::from_verteices(vertices, indices)
// }

// pub fn create_thor(vertex_count: u16, radius: u32, inner_radius: u32) -> Mesh {
//     let mut vertices: Vec<f32> = vec![];
//     let mut indices: Vec<u16> = vec! [];
//     let mut x: f32;
//     let mut y: f32;
//     let normal_radius = (radius as f32) / 100.0;
//     let normal_inner_radius = (inner_radius as f32) / 100.0;

//     let vertex_count_f32 = vertex_count as f32;
//     let mut vertex_index = 0u16;
//     for i in 0..vertex_count {
//         let angle = (2.0 * PI * i as f32) / vertex_count_f32;
//         x = normal_radius * angle.cos();
//         y = normal_radius * angle.sin();
//         let gradient = ((-x + y) / 2.0) + normal_radius;
//         vec_push_range(&mut vertices, vec![x, y, 0.0,  0.2, gradient + 0.2, 
//             1.0 - gradient / 2.0,  x + 0.5, y + 0.5]);
//         x = normal_inner_radius * angle.cos();
//         y = normal_inner_radius * angle.sin();
//         let gradient = ((-x + y) / 2.0) + normal_radius;
//         vec_push_range(&mut vertices, vec![x, y, 0.0,  0.1, gradient + 0.1, 
//             0.9 - gradient / 2.0,  x + 0.5, y + 0.5]);

//         let next_index = (vertex_index + 2) % (vertex_count * 2);
//         vec_push_range(&mut indices, vec![vertex_index, next_index, next_index + 1]);
//         vec_push_range(&mut indices, vec![vertex_index, vertex_index + 1, next_index + 1]);
//         vertex_index += 2;
//     }
//     Mesh::from_verteices(vertices, indices)
// }

// pub fn create_sphere(radius: f32, sector_count: u32, stack_count:u32) -> Mesh {
//     let mut vertices: Vec<f32> = vec![];
//     let mut indices: Vec<u16> = vec![];
//     let stack_step = PI / stack_count as f32;
//     let sector_step = 2.0 * PI / sector_count as f32;

//     for i in 0..stack_count + 1 {
//         let stack_angle = PI / 2.0 - (i as f32) * stack_step;
//         let xy = radius * stack_angle.cos();
//         let z = radius * stack_angle.sin();
//         let mut k1 = i as u16 * (sector_count as u16 + 1);
//         let mut k2 = k1 + sector_count as u16 + 1;

//         for j in 0..sector_count + 1 {
//             let sector_angle = j as f32 * sector_step;
//             let x = xy * sector_angle.cos();
//             let y = xy * sector_angle.sin();
//             let s = j as f32 / sector_count as f32;
//             let t = i as f32 / stack_count as f32;
//             let color = (z + y) / 8.0 + 0.5;
//             vec_push_range(&mut vertices, vec![x, y, z,  color, color, color,  s, t]);

//             if i != 0 {
//                 vec_push_range(&mut indices, vec![k1, k2, k1 + 1]);
//             }
//             if i != stack_count {
//                 vec_push_range(&mut indices, vec![k1 + 1, k2, k2 + 1]);
//             }
//             k1 += 1; k2 += 1;
//         }
//     }
//     Mesh::from_verteices(vertices, indices)
// }

pub fn normal_cube() -> Mesh {
    let vertices = vec![  
        -1.0, -1.0, -1.0,   0.0,  0.0, -1.0,   0.0, 0.0,
         1.0, -1.0, -1.0,   0.0,  0.0, -1.0,   1.0, 0.0,
         1.0,  1.0, -1.0,   0.0,  0.0, -1.0,   1.0, 1.0,
         1.0,  1.0, -1.0,   0.0,  0.0, -1.0,   1.0, 1.0,
        -1.0,  1.0, -1.0,   0.0,  0.0, -1.0,   0.0, 1.0,
        -1.0, -1.0, -1.0,   0.0,  0.0, -1.0,   0.0, 0.0,

        -1.0, -1.0,  1.0,   0.0,  0.0,  1.0,   0.0, 0.0,
         1.0,  1.0,  1.0,   0.0,  0.0,  1.0,   1.0, 1.0,
         1.0, -1.0,  1.0,   0.0,  0.0,  1.0,   1.0, 0.0,
         1.0,  1.0,  1.0,   0.0,  0.0,  1.0,   1.0, 1.0,
        -1.0, -1.0,  1.0,   0.0,  0.0,  1.0,   0.0, 0.0,
        -1.0,  1.0,  1.0,   0.0,  0.0,  1.0,   0.0, 1.0,
        
        -1.0,  1.0,  1.0,  -1.0,  0.0,  0.0,   1.0, 1.0,
        -1.0, -1.0, -1.0,  -1.0,  0.0,  0.0,   0.0, 0.0,
        -1.0,  1.0, -1.0,  -1.0,  0.0,  0.0,   1.0, 0.0,
        -1.0, -1.0, -1.0,  -1.0,  0.0,  0.0,   0.0, 0.0,
        -1.0,  1.0,  1.0,  -1.0,  0.0,  0.0,   1.0, 1.0,
        -1.0, -1.0,  1.0,  -1.0,  0.0,  0.0,   0.0, 1.0,
        
         1.0,  1.0,  1.0,   1.0,  0.0,  0.0,   1.0, 1.0,
         1.0,  1.0, -1.0,   1.0,  0.0,  0.0,   1.0, 0.0,
         1.0, -1.0, -1.0,   1.0,  0.0,  0.0,   0.0, 0.0,
         1.0, -1.0, -1.0,   1.0,  0.0,  0.0,   0.0, 0.0,
         1.0, -1.0,  1.0,   1.0,  0.0,  0.0,   0.0, 1.0,
         1.0,  1.0,  1.0,   1.0,  0.0,  0.0,   1.0, 1.0,
         
        -1.0, -1.0, -1.0,   0.0, -1.0,  0.0,   0.0, 0.0,
         1.0, -1.0,  1.0,   0.0, -1.0,  0.0,   1.0, 1.0,
         1.0, -1.0, -1.0,   0.0, -1.0,  0.0,   1.0, 0.0,
         1.0, -1.0,  1.0,   0.0, -1.0,  0.0,   1.0, 1.0,
        -1.0, -1.0, -1.0,   0.0, -1.0,  0.0,   0.0, 0.0,
        -1.0, -1.0,  1.0,   0.0, -1.0,  0.0,   0.0, 1.0,
        
        -1.0,  1.0, -1.0,   0.0,  1.0,  0.0,   0.0, 0.0,
         1.0,  1.0, -1.0,   0.0,  1.0,  0.0,   1.0, 0.0,
         1.0,  1.0,  1.0,   0.0,  1.0,  0.0,   1.0, 1.0,
         1.0,  1.0,  1.0,   0.0,  1.0,  0.0,   1.0, 1.0,
        -1.0,  1.0,  1.0,   0.0,  1.0,  0.0,   0.0, 1.0,
        -1.0,  1.0, -1.0,   0.0,  1.0,  0.0,   0.0, 0.0,
    ];
    Mesh::from_verteices(vertices)
}

pub fn vec_push_range<T>(vect: &mut Vec<T>, values: Vec<T>) {
    for i in values { vect.push(i); }
}
