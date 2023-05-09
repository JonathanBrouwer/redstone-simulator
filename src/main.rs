use crate::world::World;
use std::fs::File;

mod blocks;
mod schematic;
pub mod world;
mod world_data;
mod world_logic;
mod world_prune;

fn main() {
    let world = World::from(File::open("./schematics/cycle.schem").unwrap());

    println!("{:?}", world.blocks.node_count());

    // for _ in 0..10000 {
    //     world.step_with_trigger();
    //     for _ in 0..40 {
    //         world.step();
    //     }
    // }
}
