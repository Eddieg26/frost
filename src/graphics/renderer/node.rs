use super::Gpu;
use crate::graphics::RenderContext;
use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum RenderPassKind {
    Deferred = 0,
    Forward = 1,
}

pub trait RenderNode {
    fn draw(&self, context: &RenderContext, pass: &mut wgpu::RenderPass);
}

pub trait RenderNodeBuilder: 'static {
    fn build(&self, device: &Gpu) -> Box<dyn RenderNode>;
}

pub struct RenderNodes {
    nodes: HashMap<RenderPassKind, Vec<(usize, Box<dyn RenderNodeBuilder>)>>,
}

impl RenderNodes {
    pub fn new() -> RenderNodes {
        RenderNodes {
            nodes: HashMap::new(),
        }
    }

    pub fn pop(&mut self) -> Option<(RenderPassKind, Vec<(usize, Box<dyn RenderNodeBuilder>)>)> {
        self.nodes
            .iter_mut()
            .next()
            .map(|(k, v)| (*k, std::mem::take(v)))
    }

    pub fn add_node<T: RenderNodeBuilder>(
        &mut self,
        pass: RenderPassKind,
        subpass: usize,
        node: T,
    ) {
        self.nodes
            .entry(pass)
            .or_insert_with(Vec::new)
            .push((subpass, Box::new(node)));
    }

    pub fn add_node_dyn(
        &mut self,
        pass: RenderPassKind,
        subpass: usize,
        node: Box<dyn RenderNodeBuilder>,
    ) {
        self.nodes
            .entry(pass)
            .or_insert_with(Vec::new)
            .push((subpass, node));
    }
}
