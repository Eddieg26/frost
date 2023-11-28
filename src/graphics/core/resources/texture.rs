pub type Dimension = wgpu::TextureDimension;
pub type Format = wgpu::TextureFormat;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FilterMode {
    Point,
    Bilinear,
    Trilinear,
}

impl Into<wgpu::FilterMode> for FilterMode {
    fn into(self) -> wgpu::FilterMode {
        match self {
            Self::Point => wgpu::FilterMode::Nearest,
            Self::Bilinear => wgpu::FilterMode::Linear,
            Self::Trilinear => wgpu::FilterMode::Linear,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WrapMode {
    Repeat,
    Clamp,
    Mirror,
}

impl Into<wgpu::AddressMode> for WrapMode {
    fn into(self) -> wgpu::AddressMode {
        match self {
            Self::Repeat => wgpu::AddressMode::Repeat,
            Self::Clamp => wgpu::AddressMode::ClampToEdge,
            Self::Mirror => wgpu::AddressMode::MirrorRepeat,
        }
    }
}

pub trait Texture: 'static {
    fn width(&self) -> u32;
    fn height(&self) -> u32;
    fn depth(&self) -> u32;
    fn dimension(&self) -> Dimension;
    fn format(&self) -> Format;
    fn filter_mode(&self) -> FilterMode;
    fn wrap_mode(&self) -> WrapMode;
    fn mipmaps(&self) -> bool;
    fn pixels(&self) -> &[u8];
    fn view(&self) -> &wgpu::TextureView;
    fn sampler(&self) -> &wgpu::Sampler;
    fn as_any(&self) -> &dyn std::any::Any;
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any;
}

pub struct TextureInfo {
    pub width: u32,
    pub height: u32,
    pub depth: u32,
    pub dimension: Dimension,
    pub format: Format,
    pub filter_mode: FilterMode,
    pub wrap_mode: WrapMode,
    pub mipmaps: bool,
    pub pixels: Vec<u8>,
}

pub struct Texture2d {
    width: u32,
    height: u32,
    format: Format,
    filter_mode: FilterMode,
    wrap_mode: WrapMode,
    mipmaps: bool,
    pixels: Vec<u8>,
    view: wgpu::TextureView,
    sampler: wgpu::Sampler,
}

impl Texture2d {
    pub fn new(
        width: u32,
        height: u32,
        format: Format,
        filter_mode: FilterMode,
        wrap_mode: WrapMode,
        mipmaps: bool,
        pixels: Vec<u8>,
        view: wgpu::TextureView,
        sampler: wgpu::Sampler,
    ) -> Self {
        Self {
            width,
            height,
            format,
            filter_mode,
            wrap_mode,
            mipmaps,
            pixels,
            view,
            sampler,
        }
    }
}

impl Texture for Texture2d {
    fn width(&self) -> u32 {
        self.width
    }

    fn height(&self) -> u32 {
        self.height
    }

    fn depth(&self) -> u32 {
        1
    }

    fn dimension(&self) -> Dimension {
        Dimension::D2
    }

    fn format(&self) -> Format {
        self.format
    }

    fn filter_mode(&self) -> FilterMode {
        self.filter_mode
    }

    fn wrap_mode(&self) -> WrapMode {
        self.wrap_mode
    }

    fn mipmaps(&self) -> bool {
        self.mipmaps
    }

    fn pixels(&self) -> &[u8] {
        &self.pixels
    }

    fn view(&self) -> &wgpu::TextureView {
        &self.view
    }

    fn sampler(&self) -> &wgpu::Sampler {
        &self.sampler
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}
