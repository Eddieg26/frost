#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Color {
    pub r: f64,
    pub g: f64,
    pub b: f64,
    pub a: f64,
}

impl Color {
    pub fn new(r: f64, g: f64, b: f64, a: f64) -> Self {
        Self { r, g, b, a }
    }

    pub fn white() -> Self {
        Self::new(1.0, 1.0, 1.0, 1.0)
    }

    pub fn black() -> Self {
        Self::new(0.0, 0.0, 0.0, 1.0)
    }

    pub fn red() -> Self {
        Self::new(1.0, 0.0, 0.0, 1.0)
    }

    pub fn green() -> Self {
        Self::new(0.0, 1.0, 0.0, 1.0)
    }

    pub fn blue() -> Self {
        Self::new(0.0, 0.0, 1.0, 1.0)
    }
}

impl Into<[f64; 4]> for Color {
    fn into(self) -> [f64; 4] {
        [self.r, self.g, self.b, self.a]
    }
}

impl Into<[f64; 3]> for Color {
    fn into(self) -> [f64; 3] {
        [self.r, self.g, self.b]
    }
}

impl Into<[u8; 4]> for Color {
    fn into(self) -> [u8; 4] {
        [
            (self.r * 255.0) as u8,
            (self.g * 255.0) as u8,
            (self.b * 255.0) as u8,
            (self.a * 255.0) as u8,
        ]
    }
}

impl From<[f64; 4]> for Color {
    fn from(color: [f64; 4]) -> Self {
        Self::new(color[0], color[1], color[2], color[3])
    }
}

impl From<[f64; 3]> for Color {
    fn from(color: [f64; 3]) -> Self {
        Self::new(color[0], color[1], color[2], 1.0)
    }
}

impl From<[u8; 4]> for Color {
    fn from(color: [u8; 4]) -> Self {
        Self::new(
            color[0] as f64 / 255.0,
            color[1] as f64 / 255.0,
            color[2] as f64 / 255.0,
            color[3] as f64 / 255.0,
        )
    }
}

impl Into<wgpu::Color> for Color {
    fn into(self) -> wgpu::Color {
        wgpu::Color {
            r: self.r,
            g: self.g,
            b: self.b,
            a: self.a,
        }
    }
}

impl Into<[f32; 4]> for Color {
    fn into(self) -> [f32; 4] {
        [self.r as f32, self.g as f32, self.b as f32, self.a as f32]
    }
}

impl From<&Color> for [f32; 4] {
    fn from(color: &Color) -> Self {
        [
            color.r as f32,
            color.g as f32,
            color.b as f32,
            color.a as f32,
        ]
    }
}
