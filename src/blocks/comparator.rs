use crate::blocks::facing::Facing;
use crate::blocks::{Block, BlockConnections, Edge, InputSide, OutputPower, ToBlock, Updatable};
use crate::world::data::TileMap;
use crate::world::BlockGraph;
use nbt::Value;
use petgraph::prelude::EdgeRef;
use petgraph::stable_graph::NodeIndex;
use petgraph::Incoming;
use std::collections::{HashMap};
use std::sync::atomic::{AtomicU8, AtomicUsize, Ordering};

#[derive(Debug)]
pub struct Comparator {
    /// Signal ranges from 0 to 15 inclusive.
    signal: AtomicU8,

    /// Signal of the comparator during the next tick.
    next_signal: AtomicU8,

    entity_power: Option<u8>,

    /// Mode of the comparator, can be in `Compare` or `Subtract` mode.
    // todo: we can most likely get rid off this by having both a `Comparator` and `Subtractor`.
    mode: ComparatorMode,

    last_update: AtomicUsize,
}

impl CComparator {
    pub fn contains_entity_power(&self) -> bool {
        self.entity_power.is_some()
    }
}

#[derive(Copy, Clone, Debug)]
pub struct CComparator {
    /// Signal ranges from 0 to 15 inclusive.
    signal: u8,

    /// Direction of the input side of the repeater.
    facing: Facing,

    /// Mode of the comparator, can be in `Compare` or `Subtract` mode.
    mode: ComparatorMode,

    entity_power: Option<u8>,
}

#[derive(Copy, Clone, Debug)]
pub enum ComparatorMode {
    Compare,
    Subtract,
}

impl From<&str> for ComparatorMode {
    fn from(s: &str) -> Self {
        match s {
            "compare" => Self::Compare,
            "subtract" => Self::Subtract,
            _ => unreachable!(),
        }
    }
}

impl OutputPower for Comparator {
    fn output_power(&self) -> u8 {
        self.signal.load(Ordering::Relaxed)
    }
}

impl BlockConnections for CComparator {
    fn can_output(&self, facing: Facing) -> bool {
        self.facing == facing.rev()
    }

    fn can_input(&self, facing: Facing) -> Option<InputSide> {
        if self.facing == facing.rotate_left() || self.facing == facing.rotate_right() {
            Some(InputSide::Side)
        } else if self.facing == facing.rev() {
            Some(InputSide::Rear)
        } else {
            None
        }
    }
}
impl ToBlock for CComparator {
    fn to_block(&self) -> Block {
        Block::Comparator(Comparator {
            signal: AtomicU8::new(self.signal),
            next_signal: AtomicU8::new(self.signal),
            entity_power: self.entity_power,
            mode: self.mode,
            last_update: AtomicUsize::new(usize::MAX),
        })
    }
}

impl Updatable for Comparator {
    #[inline(always)]
    fn update(
        &self,
        idx: NodeIndex,
        _tick_updatable: &mut Vec<NodeIndex>,
        blocks: &BlockGraph,
    ) -> bool {
        let rear = blocks
            .edges_directed(idx, Incoming)
            .filter_map(|edge| match edge.weight() {
                Edge::Rear(s) => Some(blocks[edge.source()].output_power().saturating_sub(*s)),
                Edge::Side(_) => None,
            })
            .max()
            .max(self.entity_power)
            .unwrap_or(0);

        let side = blocks
            .edges_directed(idx, Incoming)
            .filter_map(|edge| match edge.weight() {
                Edge::Rear(_) => None,
                Edge::Side(s) => Some(blocks[edge.source()].output_power().saturating_sub(*s)),
            })
            .max()
            .unwrap_or(0);

        self.next_signal.store(match self.mode {
            ComparatorMode::Compare if side <= rear => rear,
            ComparatorMode::Compare => 0,
            ComparatorMode::Subtract => rear.saturating_sub(side),
        }, Ordering::Relaxed);

        self.signal.load(Ordering::Relaxed) != self.next_signal.load(Ordering::Relaxed)
    }

    fn late_updatable(
        &self,
        _idx: NodeIndex,
        _updatable: &mut Vec<NodeIndex>,
        tick_counter: usize,
    ) -> bool {
        if tick_counter == self.last_update.load(Ordering::Relaxed) {
            return false;
        }
        self.last_update.store(tick_counter, Ordering::Relaxed);

        self.signal.store(self.next_signal.load(Ordering::Relaxed), Ordering::Relaxed);
        true
    }
}

impl From<HashMap<&str, &str>> for CComparator {
    fn from(meta: HashMap<&str, &str>) -> Self {
        CComparator {
            signal: 0,
            facing: Facing::from(meta["facing"]),
            mode: ComparatorMode::from(meta["mode"]),
            entity_power: None,
        }
    }
}

impl CComparator {
    pub fn update_from_tile(&mut self, p: (usize, usize, usize), tile_map: &TileMap) {
        // Check what the signal of the comparator is.
        let Some(Value::Byte(s)) = tile_map.get(&p).unwrap().props.get("OutputSignal") else{
            unreachable!("Every comparator should have an OutputSignal.");
        };
        self.signal = *s as u8;

        // Checks the power the entity behind it gives off.
        // todo: currently only checks for furnaces and defaults to an output of 1.
        self.entity_power = tile_map.get(&self.facing().front(p)).and_then(|b| {
            if b.id == "minecraft:furnace" {
                Some(1)
            } else {
                None
            }
        });
    }

    pub fn facing(&self) -> Facing {
        self.facing
    }
}
