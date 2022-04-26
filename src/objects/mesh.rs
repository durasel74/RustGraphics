use cgmath::{ Vector2, Vector3 };
use obj::{ TexturedVertex };
use super::{ Vertex, RenderData, Material };

#[derive(Clone)]
pub struct Mesh {
    render_data: RenderData,
    indices_count: u16,
    material: Material,
}
impl Mesh {
    pub fn from_obj(model: &obj::Obj<TexturedVertex, u16>) -> Self {
        let obj_vertices = &model.vertices;
        let obj_indices = &model.indices;

        let mut vertices = Vec::new();
        for i in obj_vertices {
            let pos = Vector3 { x: i.position[0], y: i.position[1], z: i.position[2]};
            let norm = Vector3 { x: i.normal[0], y: i.normal[1], z: i.normal[2] };
            let tex = Vector2 { x: i.texture[0], y: i.texture[1] };
            let vertex = Vertex { position: pos, normal: norm, tex_coords: tex };
            vertices.push(vertex);
        }
        let render_data = RenderData::from_verteices(&vertices, obj_indices);
        let indices_count = obj_indices.len() as u16;
        let material = Material::new();
        Mesh { render_data, indices_count, material }
    }

    pub fn render_data(&self) -> &RenderData { &self.render_data }
    pub fn indices_count(&self) -> u16 { self.indices_count }
    pub fn material(&self) -> &Material { &self.material }
    pub fn set_material(&mut self, value: Material) { self.material = value; }
}
