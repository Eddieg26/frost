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

impl TextureInfo {
    pub fn white(format: Format) -> Self {
        let pixels = match format {
            Format::Rgba8UnormSrgb => vec![255, 255, 255, 255],
            Format::Rgba8Unorm => vec![255, 255, 255, 255],
            Format::Rgba32Float => vec![255, 255, 255, 255],
            _ => panic!("Unsupported format: {:?}", format),
        };

        Self {
            width: 1,
            height: 1,
            depth: 1,
            dimension: Dimension::D2,
            format,
            filter_mode: FilterMode::Bilinear,
            wrap_mode: WrapMode::Clamp,
            mipmaps: false,
            pixels,
        }
    }

    pub fn black(format: Format) -> Self {
        let pixels = match format {
            Format::Rgba8UnormSrgb => vec![0, 0, 0, 255],
            Format::Rgba8Unorm => vec![0, 0, 0, 255],
            Format::Rgba32Float => vec![0, 0, 0, 255],
            _ => panic!("Unsupported format: {:?}", format),
        };

        Self {
            width: 1,
            height: 1,
            depth: 1,
            dimension: Dimension::D2,
            format,
            filter_mode: FilterMode::Bilinear,
            wrap_mode: WrapMode::Clamp,
            mipmaps: false,
            pixels,
        }
    }

    pub fn gray(format: Format) -> Self {
        let pixels = match format {
            Format::Rgba8UnormSrgb => vec![128, 128, 128, 255],
            Format::Rgba8Unorm => vec![128, 128, 128, 255],
            Format::Rgba32Float => vec![128, 128, 128, 255],
            _ => panic!("Unsupported format: {:?}", format),
        };

        Self {
            width: 1,
            height: 1,
            depth: 1,
            dimension: Dimension::D2,
            format,
            filter_mode: FilterMode::Bilinear,
            wrap_mode: WrapMode::Clamp,
            mipmaps: false,
            pixels,
        }
    }
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

impl Texture2d {
    pub fn new_info(device: &wgpu::Device, queue: &wgpu::Queue, info: &TextureInfo) -> Texture2d {
        let gpu_texture = device.create_texture(&wgpu::TextureDescriptor {
            dimension: wgpu::TextureDimension::D2,
            format: info.format,
            label: None,
            mip_level_count: if info.mipmaps { 1 } else { 0 },
            sample_count: 1,
            size: wgpu::Extent3d {
                depth_or_array_layers: 1,
                height: info.height,
                width: info.width,
            },
            usage: wgpu::TextureUsages::COPY_DST | wgpu::TextureUsages::TEXTURE_BINDING,
            view_formats: &[],
        });

        queue.write_texture(
            wgpu::ImageCopyTexture {
                texture: &gpu_texture,
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
                aspect: wgpu::TextureAspect::All,
            },
            &info.pixels,
            wgpu::ImageDataLayout {
                offset: 0,
                bytes_per_row: Some(info.width * 4),
                rows_per_image: Some(info.height),
            },
            wgpu::Extent3d {
                depth_or_array_layers: info.depth,
                height: info.height,
                width: info.width,
            },
        );

        let view = gpu_texture.create_view(&wgpu::TextureViewDescriptor::default());

        let sampler = device.create_sampler(&wgpu::SamplerDescriptor {
            label: None,
            address_mode_u: info.wrap_mode.into(),
            address_mode_v: info.wrap_mode.into(),
            address_mode_w: info.wrap_mode.into(),
            mag_filter: info.filter_mode.into(),
            min_filter: info.filter_mode.into(),
            mipmap_filter: info.filter_mode.into(),
            lod_min_clamp: 0.0,
            lod_max_clamp: 100.0,
            compare: None,
            anisotropy_clamp: 1,
            border_color: None,
        });

        Texture2d::new(
            info.width,
            info.height,
            info.format,
            info.filter_mode,
            info.wrap_mode,
            info.mipmaps,
            info.pixels.clone(),
            view,
            sampler,
        )
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
