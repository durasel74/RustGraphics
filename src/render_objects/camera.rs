use cgmath::prelude::InnerSpace;
use cgmath::{ Rad, Matrix, Matrix4, Vector3, vec3, vec4, PerspectiveFov, Ortho };

pub struct Camera {
    position: Vector3<f32>,
    target: Vector3<f32>,
    direction: Vector3<f32>,
    is_look_at: bool,
    field_of_view: f32,
    is_ortho: bool,
    view_size: (f32, f32),
    ortho_factor: f32,
}
impl Camera {
    pub fn new() -> Self {
        Camera { 
            position: vec3(0.0, 0.0, 0.0),
            target: vec3(0.0, 0.0, 0.0), 
            direction: vec3(0.0, 0.0, 1.0),
            is_look_at: false,
            field_of_view: 70.0,
            is_ortho: false,
            view_size: (800.0, 600.0),
            ortho_factor: 5.0,
        }
    }

    pub fn position(&self) -> Vector3<f32> { self.position }
    pub fn set_position(&mut self, value: Vector3<f32>) { self.position = value }

    pub fn target(&self) -> Vector3<f32> { self.target }
    pub fn set_target(&mut self, value: Vector3<f32>) { self.target = value }

    pub fn direction(&self) -> Vector3<f32> { 
        if self.is_look_at{ (&self.position - &self.target).normalize() }
        else { self.direction }
    }
    pub fn set_direction(&mut self, value: Vector3<f32>) { self.direction = value.normalize(); }

    pub fn right(&self) -> Vector3<f32> {
        let cross = vec3(0.0, 1.0, 0.0).cross(self.direction());
        cross.normalize()
    }
    
    pub fn up(&self) -> Vector3<f32> {
        self.direction().cross(self.right())
    }

    pub fn is_look_at(&self) -> bool { self.is_look_at }
    pub fn set_is_look_at(&mut self, value: bool) { self.is_look_at = value; }

    pub fn field_of_view(&self) -> f32 { self.field_of_view }
    pub fn set_field_of_view(&mut self, value: f32) { self.field_of_view = value; }

    pub fn view_size(&self) -> (f32, f32) { self.view_size }
    pub fn set_view_size(&mut self, value: (f32, f32)) { self.view_size = value; }

    pub fn is_ortho(&self) -> bool { self.is_ortho }
    pub fn set_is_ortho(&mut self, value: bool) { self.is_ortho = value; }

    pub fn ortho_factor(&self) -> f32 { self.ortho_factor }
    pub fn set_ortho_factor(&mut self, value: f32) { 
        if value <= 1.0 { self.ortho_factor = 1.0; }
        else { self.ortho_factor = value;  }
    }

    pub fn view_matrix(&self) -> Matrix4<f32> {
        let direction_matrix = Matrix4::from_cols(
            self.right().extend(0.0),
            self.up().extend(0.0),
            self.direction().extend(0.0),
            vec4(0.0, 0.0, 0.0, 1.0),
        );
        let pos = vec3(-self.position.x, -self.position.y, -self.position.z);
        let position_matrix = Matrix4::from_translation(pos);
        direction_matrix.transpose() * position_matrix
    }

    pub fn projection_matrix(&self) -> Matrix4<f32> {
        let width = self.view_size.0;
        let mut height = self.view_size.1;
        if height == 0.0 { height = 1.0 }
        let aspect = width / height;

        if self.is_ortho {
            Matrix4::from(Ortho {
                left: -aspect * self.ortho_factor,
                right: aspect * self.ortho_factor,
                bottom: -1.0 * self.ortho_factor,
                top: 1.0 * self.ortho_factor,
                near: 0.1,
                far: 300.0,
            })
        }
        else {
            Matrix4::from(PerspectiveFov { 
                fovy: Rad(self.field_of_view.to_radians()),
                aspect, 
                near: 0.1,
                far: 300.0
            })
        }
    }
}
