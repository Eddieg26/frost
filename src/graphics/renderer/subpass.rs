use super::node::{RenderNode, RenderNodeBuilder};
use crate::graphics::{context::RenderContext, device::GpuDevice};

pub struct Subpass {
    nodes: Vec<Box<dyn RenderNode>>,
}

impl Subpass {
    pub fn new(nodes: Vec<Box<dyn RenderNode>>) -> Self {
        Self { nodes }
    }

    pub fn execute(&self, context: &RenderContext, pass: &mut wgpu::RenderPass) {
        for node in &self.nodes {
            node.draw(context, pass);
        }
    }
}

pub struct SubpassBuilder {
    nodes: Vec<Box<dyn RenderNodeBuilder>>,
}

impl SubpassBuilder {
    pub fn new() -> Self {
        Self { nodes: Vec::new() }
    }

    pub fn add_node<T: RenderNodeBuilder + 'static>(&mut self, node: T) {
        self.nodes.push(Box::new(node));
    }

    pub fn add_node_dyn(&mut self, node: Box<dyn RenderNodeBuilder>) {
        self.nodes.push(node);
    }

    pub fn build(self, device: &GpuDevice) -> Subpass {
        Subpass {
            nodes: self.nodes.into_iter().map(|n| n.build(device)).collect(),
        }
    }
}
