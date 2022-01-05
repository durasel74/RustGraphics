pub struct RGBA {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}
impl RGBA {
    pub fn new(r: f32, g: f32, b: f32, a: f32) -> RGBA {
        RGBA { r, g, b, a }
    }
}

pub struct Figure {
    pub vertices: Vec<f32>,
    pub indices: Vec<u32>,
}

pub fn triangle90() -> Figure {
    Figure {
        vertices: vec![
            -0.55, 0.45, 0.0,
            0.45, -0.55, 0.0,
            -0.55, -0.55, 0.0,
        ],
        indices: vec![
            0, 1, 2,
        ],
    }
}

pub fn triangle90alter() -> Figure {
    Figure {
        vertices: vec![
            -0.45, 0.55, 0.0,
            0.55, 0.55, 0.0,
            0.55, -0.45, 0.0,
        ],
        indices: vec![
            0, 1, 2,
        ],
    }
}


// Old version
#[allow(dead_code)]
pub fn triangle() -> Figure {
    Figure {
        vertices: vec![
            0.5, -0.5, 0.0,   0.8, 0.2, 0.8,
            -0.5, -0.5, 0.0,  0.8, 0.2, 1.0,
            0.0,  0.5, 0.0,   0.9, 0.2, 0.8,
        ],
        indices: vec![
            0, 1, 2,
        ],
    }
}

#[allow(dead_code)]
pub fn square() -> Figure {
    Figure {
        vertices: vec![
            -0.5,  0.5, 0.0,   1.0, 1.0, 1.0,
            0.5, 0.5, 0.0,   0.9, 0.9, 0.9,
            0.5, -0.5, 0.0,   0.8, 0.8, 0.8,
            -0.5,  -0.5, 0.0,   0.9, 0.9, 0.9,
        ],
        indices: vec![
            0, 1, 2,
            2, 3, 0,
        ],
    }

    // // Violet gradient
    // 0.8, 0.2, 0.8,
    // 0.9, 0.2, 0.8,
    // 0.8, 0.2, 0.8,
    // 0.8, 0.2, 1.0,
}

#[allow(dead_code)]
pub fn herringbone() -> Figure {
    Figure {
        vertices: vec![
            0.0, 0.0, 0.0,   0.0, 0.48, 0.1,
            -0.5, -0.5, 0.0,   0.0, 0.35, 0.1,
            0.5, -0.5, 0.0,   0.0, 0.35, 0.1,

            0.0, 0.4, 0.0,   0.0, 0.52, 0.1,
            -0.4, -0.1, 0.0,   0.0, 0.45, 0.1,
            0.4, -0.1, 0.0,   0.0, 0.45, 0.1,

            0.0, 0.7, 0.0,   0.1, 0.6, 0.2,
            -0.3, 0.3, 0.0,   0.0, 0.5, 0.1,
            0.3, 0.3, 0.0,   0.0, 0.5, 0.1,
        ],
        indices: vec![
            0, 1, 2,
            3, 4, 5,
            6, 7, 8,
        ],
    }
}
