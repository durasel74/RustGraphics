use cgmath::{ Matrix4, Vector3, vec3, Rad };
use super::{ Mesh, ShaderProgram };

pub enum LightType {
    Directional,
    Point,
    Spotlight,
}

pub struct Light {
    power: f32,
    position: Vector3<f32>,
    rotation: Vector3<f32>,
    scale: f32,

    direction: Vector3<f32>,
    ambient: Vector3<f32>,
    diffuse: Vector3<f32>,
    specular: Vector3<f32>,

    constant: f32,
    linear: f32,
    quadratic: f32,

    cut_off: f32,
    outer_cut_off: f32,

    meshes: Vec<Mesh>,
    light_type: LightType,

    radius: f32,
}
impl Light {
    pub fn new() -> Self {
        Light {
            power: 1.0,
            position: vec3(0.0, 0.0, 0.0),
            rotation: vec3(0.0, 0.0, 0.0),
            scale: 1.0,

            direction: vec3(0.0, 0.0, 0.0),
            ambient: vec3(0.0, 0.0, 0.0),
            diffuse: vec3(0.0, 0.0, 0.0),
            specular: vec3(0.0, 0.0, 0.0),

            constant: 1.0,
            linear: 0.0,
            quadratic: 0.0,

            cut_off: 0.0,
            outer_cut_off: 0.0,

            meshes: vec![],
            light_type: LightType::Directional,


            radius: 0.0,
        }
    }

    pub fn power(&self) -> f32 { self.power }
    pub fn set_power(&mut self, value: f32) { self.power = value; }
    pub fn position(&self) -> Vector3<f32> { self.position }
    pub fn set_position(&mut self, value: Vector3<f32>) { self.position = value; }
    pub fn rotation(&self) -> Vector3<f32> { self.rotation }
    pub fn set_rotation(&mut self, value: Vector3<f32>) { self.rotation = value; } 
    pub fn scale(&self) -> f32 { self.scale }
    pub fn set_scale(&mut self, value: f32) { self.scale = value; }

    pub fn direction(&self) -> Vector3<f32> { self.direction }
    pub fn set_direction(&mut self, value: Vector3<f32>) { self.direction = value; }
    pub fn ambient(&self) -> Vector3<f32> { self.ambient * self.power }
    pub fn set_ambient(&mut self, value: Vector3<f32>) { self.ambient = value; }
    pub fn diffuse(&self) -> Vector3<f32> { self.diffuse * self.power }
    pub fn set_diffuse(&mut self, value: Vector3<f32>) { self.diffuse = value; }
    pub fn specular(&self) -> Vector3<f32> { self.specular * self.power }
    pub fn set_specular(&mut self, value: Vector3<f32>) { self.specular = value; }

    pub fn constant(&self) -> f32 { self.constant }
    pub fn set_constant(&mut self, value: f32) { self.constant = value; }
    pub fn linear(&self) -> f32 { self.linear }
    pub fn set_linear(&mut self, value: f32) { self.linear = value; }
    pub fn quadratic(&self) -> f32 { self.quadratic }
    pub fn set_quadratic(&mut self, value: f32) { self.quadratic = value; }

    pub fn cut_off(&self) -> f32 { self.cut_off }
    pub fn set_cut_off(&mut self, value: f32) { self.cut_off = value; }
    pub fn outer_cut_off(&self) -> f32 { self.outer_cut_off }
    pub fn set_outer_cut_off(&mut self, value: f32) { self.outer_cut_off = value; }

    pub fn meshes(&self) -> &Vec<Mesh> { &self.meshes }
    pub fn set_meshes(&mut self, value: Vec<Mesh>) { self.meshes = value; }
    pub fn light_type(&self) -> &LightType { &self.light_type }
    pub fn set_light_type(&mut self, value: LightType) { self.light_type = value; }



    pub fn radius(&self) -> f32 { self.radius }
    pub fn set_radius(&mut self, value: f32) { self.radius = value; }


    
    pub fn transform_matrix(&self) -> Matrix4<f32> {
        let pos_matrix = Matrix4::from_translation(self.position.clone());
        let rot_matrix = Matrix4::from_angle_x(Rad(self.rotation.x.to_radians())) * 
            Matrix4::from_angle_y(Rad(self.rotation.y.to_radians())) *
            Matrix4::from_angle_z(Rad(self.rotation.z.to_radians()));
        let sca_matrix = Matrix4::from_scale(self.scale.clone());
        pos_matrix * rot_matrix * sca_matrix
    }

    pub fn configure_shader(&self, shader_program: &ShaderProgram, field_name: &str) {
        match self.light_type {
            LightType::Directional => self.configure_shader_directional(shader_program, field_name),
            LightType::Point => self.configure_shader_point(shader_program, field_name),
            LightType::Spotlight => self.configure_shader_spot(shader_program, field_name),
        }
    }

    fn configure_shader_directional(&self, shader_program: &ShaderProgram, field_name: &str) {
        shader_program.set_uniform_vector3(&format!("{}.ambient", field_name), &self.ambient());
        shader_program.set_uniform_vector3(&format!("{}.diffuse", field_name), &self.diffuse());
        shader_program.set_uniform_vector3(&format!("{}.specular", field_name), &self.specular());
        
        shader_program.set_uniform_vector3(&format!("{}.direction", field_name), &self.direction());
    }

    fn configure_shader_point(&self, shader_program: &ShaderProgram, field_name: &str) {
        shader_program.set_uniform_vector3(&format!("{}.ambient", field_name), &self.ambient());
        shader_program.set_uniform_vector3(&format!("{}.diffuse", field_name), &self.diffuse());
        shader_program.set_uniform_vector3(&format!("{}.specular", field_name), &self.specular());

        shader_program.set_uniform_vector3(&format!("{}.position", field_name), &self.position());
        shader_program.set_uniform_float(&format!("{}.constant", field_name), self.constant());
        shader_program.set_uniform_float(&format!("{}.linear", field_name), self.linear());
        shader_program.set_uniform_float(&format!("{}.quadratic", field_name), self.quadratic());
    }

    fn configure_shader_spot(&self, shader_program: &ShaderProgram, field_name: &str) {
        shader_program.set_uniform_vector3(&format!("{}.ambient", field_name), &self.ambient());
        shader_program.set_uniform_vector3(&format!("{}.diffuse", field_name), &self.diffuse());
        shader_program.set_uniform_vector3(&format!("{}.specular", field_name), &self.specular());

        shader_program.set_uniform_vector3(&format!("{}.position", field_name), &self.position());
        shader_program.set_uniform_vector3(&format!("{}.direction", field_name), &self.direction());
        shader_program.set_uniform_float(&format!("{}.cutOff", field_name), self.cut_off().to_radians().cos());
        shader_program.set_uniform_float(&format!("{}.outerCutOff", field_name), self.outer_cut_off().to_radians().cos());
    }
}
