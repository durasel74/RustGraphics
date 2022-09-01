use cgmath::{ Vector2, Vector3 };
use obj::{ TexturedVertex };
use super::{ Vertex, RenderData, Material };

#[derive(Clone)]
pub struct Mesh {
    render_data: RenderData,
    indices_count: u32,
    material: Material,
}
impl Mesh {
    pub fn from_vertices(vertices: Vec<Vertex>, indices: Vec<u32>) -> Self {
        let render_data = RenderData::from_vertices(&vertices, &indices);
        let indices_count = indices.len() as u32;
        let material = Material::new();
        Mesh { render_data, indices_count, material }
    }

    pub fn render_data(&self) -> &RenderData { &self.render_data }
    pub fn indices_count(&self) -> u32 { self.indices_count }
    pub fn material(&self) -> &Material { &self.material }
    pub fn set_material(&mut self, value: Material) { self.material = value; }
}
