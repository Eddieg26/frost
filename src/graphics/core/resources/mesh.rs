use crate::{graphics::vertex::Vertex, shared::Bounds};
use std::rc::Rc;
use wgpu::util::DeviceExt;

#[derive(Clone, Copy)]
pub struct SubMesh {
    pub index_start: u32,
    pub index_count: u32,
}

#[derive(Clone)]
pub struct Mesh {
    vertex_buffer: Rc<wgpu::Buffer>,
    index_buffer: Rc<wgpu::Buffer>,
    submeshes: Vec<SubMesh>,
    vertex_count: u32,
    index_count: u32,
    bounds: Bounds,
}

impl Mesh {
    pub(crate) fn new(
        device: &wgpu::Device,
        vertices: &[Vertex],
        indices: &[u32],
        submeshes: &[SubMesh],
    ) -> Mesh {
        let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Vertex Buffer"),
            contents: bytemuck::cast_slice(vertices),
            usage: wgpu::BufferUsages::VERTEX,
        });
        let index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Index Buffer"),
            contents: bytemuck::cast_slice(indices),
            usage: wgpu::BufferUsages::INDEX,
        });

        let bounds = Bounds::from_points(
            &vertices
                .iter()
                .map(|v| glam::Vec3::new(v.position[0], v.position[1], v.position[2]))
                .collect::<Vec<_>>(),
        );

        Mesh {
            vertex_buffer: Rc::new(vertex_buffer),
            index_buffer: Rc::new(index_buffer),
            submeshes: submeshes.to_vec(),
            vertex_count: vertices.len() as u32,
            index_count: indices.len() as u32,
            bounds,
        }
    }

    pub fn vertex_buffer(&self) -> &wgpu::Buffer {
        &self.vertex_buffer
    }

    pub fn index_buffer(&self) -> &wgpu::Buffer {
        &self.index_buffer
    }

    pub fn submeshes(&self) -> &[SubMesh] {
        &self.submeshes
    }

    pub fn vertex_count(&self) -> u32 {
        self.vertex_count
    }

    pub fn index_count(&self) -> u32 {
        self.index_count
    }

    pub fn bounds(&self) -> &Bounds {
        &self.bounds
    }
}
