use cgmath::{ Vector2, Vector3, vec2, vec3 };
use obj::{ TexturedVertex };
use super::{ Vertex, Texture };
use super::RenderData;

#[derive(Clone)]
pub struct Mesh {
    // vertices: Vec<Vertex>,
    indices: Vec<u16>,
    // textures: Vec<Texture>,

    render_data: RenderData,
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
        let indices = obj_indices.clone();
        let render_data = RenderData::from_verteices(&vertices, &indices);
        // Mesh { vertices, indices, render_data }
        Mesh { indices, render_data }
    }

    // pub fn vertices(&self) -> &Vec<Vertex> { &self.vertices }
    pub fn indices(&self) -> &Vec<u16> { &self.indices }
    //pub fn textures(&self) -> &Vec<Texture> { &self.textures }
    pub fn render_data(&self) -> &RenderData { &self.render_data }
}
