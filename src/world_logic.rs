use crate::block::Block;
use crate::world::World;
use petgraph::prelude::*;
use petgraph::stable_graph::NodeIndex;
use petgraph::{Incoming, Outgoing};
use std::mem;
use itertools::Itertools;

impl World {
    pub fn step(&mut self) {
        let mut tick_updatable: Vec<NodeIndex> = mem::take(&mut self.updatable);

        // Tick updates
        while let Some(idx) = tick_updatable.pop() {
            let mut block = self.blocks[idx].clone();

            let s_new = self
                .blocks
                .edges_directed(idx, Incoming)
                .map(|edge| {
                    self.blocks[edge.source()]
                        .output_power()
                        .saturating_sub(*edge.weight())
                })
                .max()
                .unwrap_or(0);

            if match &mut block {
                Block::Redstone(s) => {
                    if *s != s_new {
                        *s = s_new;
                        tick_updatable.extend(self.blocks.neighbors_directed(idx, Outgoing));
                    }
                    false
                }
                Block::Solid(s) => {
                    if *s != s_new {
                        *s = s_new;
                        tick_updatable.extend(self.blocks.neighbors_directed(idx, Outgoing));
                    }
                    false
                }
                Block::Repeater {
                    powered,
                    next_powered,
                    count,
                    ..
                } => {
                    let s_new = s_new > 0;
                    // if signal strength has changed, update neighbours
                    match (s_new, *next_powered == s_new, *count == 0) {
                        // Signal changed upwards: update next signal and reset count.
                        (true, false, _) => {
                            *next_powered = s_new;
                            *count = 0;
                        }
                        // Signal changed downward, and is not propagating already: update next signal.
                        (false, false, true) => {
                            *next_powered = s_new;
                        }
                        // Other cases.
                        (_, _, _) => {}
                    };

                    *powered != *next_powered
                }
            } {
                self.updatable.push(idx);
            }

            self.blocks[idx] = block;
        }

        // End-of-tick updates
        for &idx in self.updatable.clone().iter().unique() {
            match &mut self.blocks[idx] {
                Block::Repeater { powered, next_powered, delay, count } => {
                    *count += 1;
                    if *count == *delay {
                        *powered = *next_powered;
                        *count = 0;
                        self.updatable.extend(self.blocks.neighbors_directed(idx, Outgoing));
                    }
                },
                _ => {}
            };
        }
    }

    pub fn step_with_trigger(&mut self) {
        // put redstone power on triggers
        for &t in &self.triggers {
            self.blocks[t] = Block::Solid(15);
            for n in self.blocks.neighbors_directed(t, Outgoing) {
                self.updatable.push(n);
            }
        }

        self.step();

        // take redstone power off triggers
        for &t in &self.triggers {
            self.blocks[t] = Block::Solid(0);
            for n in self.blocks.neighbors_directed(t, Outgoing) {
                self.updatable.push(n);
            }
        }
    }
}
