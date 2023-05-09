use crate::blocks::facing::Facing;
use crate::blocks::{Block, BlockConnections};
use crate::world::RedGraph;
use petgraph::stable_graph::NodeIndex;

#[derive(Copy, Clone, Debug, Default)]
pub struct CRedstoneBlock {
    /// `NodeIndex` of this block in the graph. Initially set to `None`.
    node: Option<NodeIndex>,
}

impl BlockConnections for CRedstoneBlock {
    fn can_output(&self, _facing: Facing) -> Option<NodeIndex> {
        self.node
    }

    fn can_input(&self, _facing: Facing) -> Option<NodeIndex> {
        None
    }

    fn add_node<F, G>(&mut self, blocks: &mut RedGraph, _add_probe: &mut F, _add_trigger: &mut G)
    where
        F: FnMut(NodeIndex),
        G: FnMut(NodeIndex),
    {
        self.node = Some(blocks.add_node(Block::RedstoneBlock));
    }
}