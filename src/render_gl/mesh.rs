use super::RenderData;

pub struct Mesh {
    vertices: Vec<f32>,
    indices: Vec<u16>,
    render_data: RenderData,
}
impl Mesh {
    pub fn from_verteices(vertices: Vec<f32>, indices: Vec<u16>) -> Mesh
    {
        let render_data = RenderData::from_verteices(&vertices, &indices);
        Mesh { vertices, indices, render_data }
    }

    pub fn vertices(&self) -> &Vec<f32> { &self.vertices }
    pub fn indices(&self) -> &Vec<u16> { &self.indices }
    pub fn render_data(&self) -> &RenderData { &self.render_data }
}
impl Drop for Mesh {
    fn drop(&mut self) {
        
    }
}
