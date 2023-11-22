use super::{
    node::{RenderNodeBuilder, RenderPassKind},
    subpass::{Subpass, SubpassBuilder},
};
use crate::graphics::{
    context::RenderContext, device::GpuDevice, view::RenderView, Graphics, TextureId,
};
use std::collections::HashMap;
use wgpu::TextureView;

pub enum Attachment {
    Surface,
    Texture(TextureId),
}

pub struct ColorAttachment {
    pub attachment: Attachment,
    pub resolve_target: Option<Attachment>,
    pub store_op: wgpu::StoreOp,
    pub clear: bool,
}

pub struct DepthAttachment {
    pub attachment: Attachment,
    pub depth_store_op: wgpu::StoreOp,
    pub stencil_store_op: wgpu::StoreOp,
    pub clear_depth: Option<f32>,
    pub clear_stencil: Option<u32>,
}

pub struct RenderPass {
    id: RenderPassKind,
    colors: Vec<ColorAttachment>,
    depth: Option<DepthAttachment>,
    subpasses: Vec<Subpass>,
}

impl RenderPass {
    pub fn new(
        id: RenderPassKind,
        colors: Vec<ColorAttachment>,
        depth: Option<DepthAttachment>,
        subpasses: Vec<Subpass>,
    ) -> Self {
        Self {
            id,
            colors,
            depth,
            subpasses,
        }
    }

    pub fn id(&self) -> RenderPassKind {
        self.id
    }

    pub fn execute(
        &self,
        graphics: &Graphics,
        view: &RenderView,
        target: &TextureView,
        textures: &HashMap<TextureId, wgpu::TextureView>,
        encoder: &mut wgpu::CommandEncoder,
    ) {
        let ctx = RenderContext::new(graphics, view, &textures);

        let mut render_pass = self.begin_render_pass(view, target, textures, encoder);
        for subpass in &self.subpasses {
            //SORT DRAW COMMANDS HERE

            subpass.execute(&ctx, &mut render_pass);
        }
    }

    fn begin_render_pass<'a>(
        &'a self,
        view: &RenderView,
        target: &'a TextureView,
        textures: &'a HashMap<TextureId, wgpu::TextureView>,
        encoder: &'a mut wgpu::CommandEncoder,
    ) -> wgpu::RenderPass<'a> {
        encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("Render Pass"),
            color_attachments: &self
                .colors
                .iter()
                .map(|color| {
                    Some(wgpu::RenderPassColorAttachment {
                        view: match color.attachment {
                            Attachment::Surface => &target,
                            Attachment::Texture(ref id) => {
                                textures.get(id).expect("Texture not found")
                            }
                        },
                        ops: wgpu::Operations {
                            store: color.store_op,
                            load: match color.clear {
                                true => wgpu::LoadOp::Clear(view.clear_color().into()),
                                false => wgpu::LoadOp::Load,
                            },
                        },
                        resolve_target: match color.resolve_target {
                            Some(ref attachment) => Some(match attachment {
                                Attachment::Surface => &target,
                                Attachment::Texture(ref id) => {
                                    textures.get(id).expect("Texture not found")
                                }
                            }),
                            None => None,
                        },
                    })
                })
                .collect::<Vec<_>>(),
            depth_stencil_attachment: match self.depth {
                Some(ref depth) => Some(wgpu::RenderPassDepthStencilAttachment {
                    view: match depth.attachment {
                        Attachment::Surface => &target,
                        Attachment::Texture(ref id) => textures.get(id).expect("Texture not found"),
                    },
                    depth_ops: Some(wgpu::Operations {
                        load: wgpu::LoadOp::Clear(depth.clear_depth.unwrap_or(1.0)),
                        store: depth.depth_store_op,
                    }),
                    stencil_ops: Some(wgpu::Operations {
                        load: wgpu::LoadOp::Clear(depth.clear_stencil.unwrap_or(0)),
                        store: depth.stencil_store_op,
                    }),
                }),
                None => None,
            },
            ..Default::default()
        })
    }
}

pub struct RenderPassBuilder {
    id: RenderPassKind,
    colors: Vec<ColorAttachment>,
    depth: Option<DepthAttachment>,
    subpasses: Vec<SubpassBuilder>,
}

impl RenderPassBuilder {
    pub fn new(id: RenderPassKind) -> Self {
        Self {
            id,
            colors: Vec::new(),
            depth: None,
            subpasses: Vec::new(),
        }
    }

    pub fn id(&self) -> RenderPassKind {
        self.id
    }

    pub fn with_color(
        mut self,
        attachment: Attachment,
        resolve_target: Option<Attachment>,
        store_op: wgpu::StoreOp,
        clear: bool,
    ) -> Self {
        self.colors.push(ColorAttachment {
            attachment,
            resolve_target,
            store_op,
            clear,
        });

        self
    }

    pub fn with_depth(
        mut self,
        attachment: Attachment,
        depth_store_op: wgpu::StoreOp,
        stencil_store_op: wgpu::StoreOp,
        clear_depth: Option<f32>,
        clear_stencil: Option<u32>,
    ) -> Self {
        self.depth = Some(DepthAttachment {
            attachment,
            depth_store_op,
            stencil_store_op,
            clear_depth,
            clear_stencil,
        });

        self
    }

    pub fn add_subpass(&mut self, subpass: SubpassBuilder) {
        self.subpasses.push(subpass);
    }

    pub fn add_node<T: RenderNodeBuilder>(&mut self, subpass: usize, node: T) {
        self.subpasses[subpass].add_node(node);
    }

    pub fn add_dyn_node(&mut self, subpass: usize, node: Box<dyn RenderNodeBuilder>) {
        self.subpasses[subpass].add_node_dyn(node);
    }

    pub fn build(self, device: &GpuDevice) -> RenderPass {
        RenderPass::new(
            self.id,
            self.colors,
            self.depth,
            self.subpasses
                .into_iter()
                .map(|x| x.build(device))
                .collect(),
        )
    }
}
