use crate::graphics::{
    material::MaterialInfo, mesh::Mesh, texture::Texture, vertex::Vertex, BufferId, MaterialId,
    MeshId, TextureId,
};

pub trait GraphicsResources {
    fn buffer(&self, id: &BufferId) -> Option<&wgpu::Buffer>;
    fn texture(&self, id: &TextureId) -> Option<&wgpu::Texture>;
    fn mesh(&self, id: &MeshId) -> Option<&Mesh>;
    fn create_vertex_buffer(&mut self, id: &BufferId, vertices: &Vec<Vertex>);
    fn create_index_buffer(&mut self, id: &BufferId, indices: &Vec<u32>);
    fn create_uniform_buffer(&mut self, id: &BufferId, buffer: &[u8]);
    fn create_texture(&mut self, id: &TextureId, texture: &dyn Texture);
    fn create_material(&mut self, id: &MaterialId, info: &MaterialInfo);
    fn create_mesh(&mut self, id: &MeshId, vertices: &Vec<Vertex>, indices: &Vec<u32>);
}
