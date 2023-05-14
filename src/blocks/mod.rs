use crate::blocks::comparator::{CComparator, Comparator};
use crate::blocks::facing::Facing;
use crate::blocks::probe::CProbe;
use crate::blocks::redstone::{CRedstone, Redstone};
use crate::blocks::redstone_block::CRedstoneBlock;
use crate::blocks::repeater::{CRepeater, Repeater};
use crate::blocks::solid::{CSolidStrong, CSolidWeak};
use crate::blocks::srepeater::SRepeater;
use crate::blocks::torch::{CTorch, Torch};
use crate::blocks::trigger::CTrigger;
use crate::world::BlockGraph;
use once_cell::sync::Lazy;
use petgraph::stable_graph::NodeIndex;
use std::collections::{HashMap, HashSet, VecDeque};
use std::ops::Add;

mod comparator;
pub mod facing;
mod probe;
mod redstone;
mod redstone_block;
mod repeater;
mod solid;
mod srepeater;
mod torch;
mod trigger;

static SOLID_BLOCKS: Lazy<HashSet<&'static str>> =
    Lazy::new(|| include_str!("../../resources/solid.txt").lines().collect());
static TRANSPARENT_BLOCKS: Lazy<HashSet<&'static str>> = Lazy::new(|| {
    include_str!("../../resources/transparent.txt")
        .lines()
        .collect()
});

/// Blocks that end up in the graph structure of the world.
#[derive(Clone, Debug)]
pub enum Block {
    Redstone(Redstone),
    Repeater(Repeater),
    Torch(Torch),
    Comparator(Comparator),
    SRepeater(SRepeater),
}

/// Blocks used during the creation of the graph structure of the world.
#[derive(Clone, Debug)]
pub enum CBlock {
    Redstone(CRedstone),
    SolidWeak(CSolidWeak),
    SolidStrong(CSolidStrong),
    Trigger(CTrigger),
    Probe(CProbe),
    Repeater(CRepeater),
    RedstoneBlock(CRedstoneBlock),
    Torch(CTorch),
    Comparator(CComparator),
}

#[derive(Debug, Clone, Copy, Ord, PartialOrd, Eq, PartialEq)]
pub enum Edge {
    Rear(u8),
    Side(u8),
}

impl Edge {
    pub fn is_side(&self) -> bool {
        match self {
            Edge::Rear(_) => false,
            Edge::Side(_) => true,
        }
    }

    pub fn strength_loss(&self) -> u8 {
        match self {
            Edge::Rear(v) => *v,
            Edge::Side(v) => *v,
        }
    }
}

impl Add<&Edge> for Edge {
    type Output = Self;

    fn add(self, rhs: &Edge) -> Self::Output {
        match (self, rhs) {
            (Edge::Rear(s1), Edge::Side(s2)) => Edge::Side(s1 + s2),
            (Edge::Rear(s1), Edge::Rear(s2)) => Edge::Rear(s1 + s2),
            _ => unreachable!(),
        }
    }
}

pub trait OutputPower {
    fn output_power(&self) -> u8;
}

impl OutputPower for Block {
    fn output_power(&self) -> u8 {
        match self {
            Block::Redstone(v) => v.output_power(),
            Block::Repeater(v) => v.output_power(),
            Block::Torch(v) => v.output_power(),
            Block::Comparator(v) => v.output_power(),
            Block::SRepeater(v) => v.output_power(),
        }
    }
}

impl Block {
    fn will_lock(&self) -> bool {
        match self {
            Block::Repeater(v) => v.will_lock(),
            Block::Comparator(v) => v.output_power() > 0,
            _ => unreachable!(),
        }
    }
}

pub trait BlockConnections {
    fn can_output(&self, facing: Facing) -> Option<NodeIndex>;

    fn can_input(&self, facing: Facing) -> (Option<NodeIndex>, bool);

    fn add_node(&mut self, blocks: &mut BlockGraph);
}

fn can_connect(source: &CBlock, target: &CBlock, facing: Facing) -> bool {
    #[rustfmt::skip]
    return match (source, target) {
        (CBlock::Redstone(_), CBlock::Redstone(_)) => true,
        (CBlock::Redstone(_), CBlock::SolidWeak(_)) => true,
        (CBlock::Redstone(_), CBlock::Probe(_)) => true,
        (CBlock::Redstone(_), CBlock::Repeater(v)) if facing == v.facing().rev() => true,
        (CBlock::Redstone(_), CBlock::Comparator(_)) => true,

        (CBlock::Trigger(_), CBlock::Redstone(_)) => true,
        (CBlock::Trigger(_), CBlock::Repeater(v)) if facing == v.facing().rev() => true,
        (CBlock::Trigger(_), CBlock::Torch(_)) => true,
        (CBlock::Trigger(_), CBlock::Comparator(v)) if facing == v.facing().rev() => true,

        (CBlock::SolidStrong(_), CBlock::Redstone(_)) => true,
        (CBlock::SolidWeak(_) | CBlock::SolidStrong(_), CBlock::Repeater(v)) if facing == v.facing().rev() => true,
        (CBlock::SolidWeak(_) | CBlock::SolidStrong(_), CBlock::Torch(_)) => true,
        (CBlock::SolidWeak(_) | CBlock::SolidStrong(_), CBlock::Comparator(v)) if facing == v.facing().rev() => true,

        (CBlock::Repeater(_), CBlock::Redstone(_)) => true,
        (CBlock::Repeater(_), CBlock::SolidStrong(_)) => true,
        (CBlock::Repeater(_), CBlock::Probe(_)) => true,
        (CBlock::Repeater(_), CBlock::Repeater(_)) => true,
        (CBlock::Repeater(_), CBlock::Comparator(_)) => true,

        (CBlock::RedstoneBlock(_), CBlock::Redstone(_)) => true,
        (CBlock::RedstoneBlock(_), CBlock::Repeater(v)) if facing == v.facing().rev() => true,
        (CBlock::RedstoneBlock(_), CBlock::Torch(_)) => true,
        (CBlock::RedstoneBlock(_), CBlock::Comparator(_)) => true,

        (CBlock::Torch(_), CBlock::Redstone(_)) => true,
        (CBlock::Torch(_), CBlock::SolidStrong(_)) if facing == Facing::Up => true,
        (CBlock::Torch(_), CBlock::Probe(_)) if facing == Facing::Up => true,
        (CBlock::Torch(_), CBlock::Repeater(v)) if facing == v.facing().rev() => true,
        (CBlock::Torch(_), CBlock::Comparator(v)) if facing == v.facing().rev() => true,

        (CBlock::Comparator(_), CBlock::Redstone(_)) => true,
        (CBlock::Comparator(_), CBlock::SolidStrong(_)) => true,
        (CBlock::Comparator(_), CBlock::Probe(_)) => true,
        (CBlock::Comparator(_), CBlock::Repeater(_)) => true,
        (CBlock::Comparator(_), CBlock::Comparator(_)) => true,

        _ => false,
    };
}

impl BlockConnections for CBlock {
    fn can_output(&self, facing: Facing) -> Option<NodeIndex> {
        match self {
            CBlock::Redstone(v) => v.can_output(facing),
            CBlock::SolidWeak(v) => v.can_output(facing),
            CBlock::SolidStrong(v) => v.can_output(facing),
            CBlock::Trigger(v) => v.can_output(facing),
            CBlock::Probe(_) => None,
            CBlock::Repeater(v) => v.can_output(facing),
            CBlock::RedstoneBlock(v) => v.can_output(facing),
            CBlock::Torch(v) => v.can_output(facing),
            CBlock::Comparator(v) => v.can_output(facing),
        }
    }

    fn can_input(&self, facing: Facing) -> (Option<NodeIndex>, bool) {
        match self {
            CBlock::Redstone(v) => v.can_input(facing),
            CBlock::SolidWeak(v) => v.can_input(facing),
            CBlock::SolidStrong(v) => v.can_input(facing),
            CBlock::Trigger(_) => (None, false),
            CBlock::Probe(v) => v.can_input(facing),
            CBlock::Repeater(v) => v.can_input(facing),
            CBlock::RedstoneBlock(_) => (None, false),
            CBlock::Torch(v) => v.can_input(facing),
            CBlock::Comparator(v) => v.can_input(facing),
        }
    }

    fn add_node(&mut self, blocks: &mut BlockGraph) {
        match self {
            CBlock::Redstone(v) => v.add_node(blocks),
            CBlock::SolidWeak(v) => v.add_node(blocks),
            CBlock::SolidStrong(v) => v.add_node(blocks),
            CBlock::Trigger(v) => v.add_node(blocks),
            CBlock::Probe(v) => v.add_node(blocks),
            CBlock::Repeater(v) => v.add_node(blocks),
            CBlock::RedstoneBlock(v) => v.add_node(blocks),
            CBlock::Torch(v) => v.add_node(blocks),
            CBlock::Comparator(v) => v.add_node(blocks),
        }
    }
}

impl CBlock {
    pub(crate) fn from_id(id: &str) -> Vec<Self> {
        let (id, meta) = id
            .split_once('[')
            .map_or((id, ""), |(x, y)| (x, y.trim_end_matches(']')));

        let meta = meta
            .split(',')
            .filter(|v| !v.is_empty())
            .map(|key_value| key_value.split_once('=').unwrap())
            .collect::<HashMap<&str, &str>>();

        match id {
            "minecraft:redstone_wire" => vec![CBlock::Redstone(CRedstone::from(meta))],
            "minecraft:gold_block" => vec![CBlock::Trigger(CTrigger::default())],
            "minecraft:lightning_rod" => vec![CBlock::Trigger(CTrigger::default())],
            "minecraft:diamond_block" => vec![CBlock::Probe(CProbe::default())],
            "minecraft:redstone_block" => vec![CBlock::RedstoneBlock(CRedstoneBlock::default())],
            "minecraft:redstone_torch" => vec![CBlock::Torch(CTorch::from(meta))],
            "minecraft:redstone_wall_torch" => vec![CBlock::Torch(CTorch::from(meta))],
            "minecraft:comparator" => vec![CBlock::Comparator(CComparator::from(meta))],
            "minecraft:repeater" => vec![CBlock::Repeater(CRepeater::from(meta))],
            id if SOLID_BLOCKS.contains(id) => vec![
                CBlock::SolidWeak(CSolidWeak::default()),
                CBlock::SolidStrong(CSolidStrong::default()),
            ],
            id if TRANSPARENT_BLOCKS.contains(id) => vec![],
            _ => panic!("Undefined block with id {id}."),
        }
    }
}

impl CBlock {
    pub fn is_transparent(&self) -> bool {
        match self {
            CBlock::SolidWeak { .. } => false,
            CBlock::SolidStrong { .. } => false,
            CBlock::Redstone { .. } => true,
            CBlock::RedstoneBlock { .. } => false,
            CBlock::Trigger { .. } => false,
            CBlock::Repeater { .. } => true,
            CBlock::Comparator { .. } => true,
            CBlock::Torch { .. } => true,
            CBlock::Probe { .. } => false,
        }
    }

    pub fn add_edge(&self, target: &CBlock, facing: Facing, blocks: &mut BlockGraph) {
        let Some(idx) = self.can_output(facing) else {
            return;
        };

        let (Some(n_idx), alternate) = target.can_input(facing) else {
            return;
        };

        if !can_connect(self, target, facing) {
            return;
        }

        let redstone_connection =
            matches!(self, CBlock::Redstone(_)) && matches!(target, CBlock::Redstone(_));

        let weight = match (alternate, redstone_connection) {
            (true, _) => Edge::Side(0),
            (false, false) => Edge::Rear(0),
            (false, true) => Edge::Rear(1),
        };

        blocks.add_edge(idx, n_idx, weight);
    }
}

pub trait Updatable {
    fn update(
        &mut self,
        idx: NodeIndex,
        tick_updatable: &mut VecDeque<NodeIndex>,
        blocks: &BlockGraph,
    ) -> bool;

    fn late_updatable(
        &mut self,
        idx: NodeIndex,
        updatable: &mut VecDeque<NodeIndex>,
        tick_counter: usize,
    ) -> bool;
}

impl Updatable for Block {
    #[inline(always)]
    fn update(
        &mut self,
        idx: NodeIndex,
        tick_updatable: &mut VecDeque<NodeIndex>,
        blocks: &BlockGraph,
    ) -> bool {
        match self {
            Block::Repeater(v) => v.update(idx, tick_updatable, blocks),
            Block::Torch(v) => v.update(idx, tick_updatable, blocks),
            Block::Comparator(v) => v.update(idx, tick_updatable, blocks),
            Block::Redstone(v) => v.update(idx, tick_updatable, blocks),
            Block::SRepeater(v) => v.update(idx, tick_updatable, blocks),
        }
    }

    fn late_updatable(
        &mut self,
        idx: NodeIndex,
        updatable: &mut VecDeque<NodeIndex>,
        tick_counter: usize,
    ) -> bool {
        match self {
            Block::Repeater(v) => v.late_updatable(idx, updatable, tick_counter),
            Block::Torch(v) => v.late_updatable(idx, updatable, tick_counter),
            Block::Comparator(v) => v.late_updatable(idx, updatable, tick_counter),
            Block::Redstone(_) => unreachable!(),
            Block::SRepeater(v) => v.late_updatable(idx, updatable, tick_counter),
        }
    }
}

pub fn redstone_min() -> Block {
    Block::Redstone(Redstone::with_signal(0))
}

pub fn redstone_max() -> Block {
    Block::Redstone(Redstone::with_signal(15))
}

pub fn srepeater_powered() -> Block {
    Block::SRepeater(SRepeater::with_power(true))
}

pub fn srepeater_unpowered() -> Block {
    Block::SRepeater(SRepeater::with_power(false))
}
