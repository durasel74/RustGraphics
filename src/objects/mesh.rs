//use super::RenderData;
use super::RenderDataNormal;

#[derive(Clone)]
pub struct Mesh {
    vertices: Vec<f32>,
    render_data: RenderDataNormal,
}
impl Mesh {
    // pub fn from_verteices(vertices: Vec<f32>, indices: Vec<u16>) -> Mesh
    // {
    //     let render_data = RenderData::from_verteices(&vertices, &indices);
    //     Mesh { vertices, indices, render_data }
    // }

    pub fn from_verteices(vertices: Vec<f32>) -> Mesh
    {
        let render_data = RenderDataNormal::from_verteices(&vertices);
        Mesh { vertices, render_data }
    }

    pub fn vertices(&self) -> &Vec<f32> { &self.vertices }
    //pub fn indices(&self) -> &Vec<u16> { &self.indices }
    pub fn render_data(&self) -> &RenderDataNormal { &self.render_data }
}
