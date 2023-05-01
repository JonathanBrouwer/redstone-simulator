use crate::block::Block;
use crate::world::World;
use std::mem;

impl World {
    pub fn step(&mut self) {
        let mut tick_updatable = mem::replace(&mut self.updatable, Vec::new());

        while let Some((x, y, z)) = tick_updatable.pop() {
            let block = self.data[x][y][z];

            match block {
                Block::Redstone(s) => {
                    // find biggest signal strength around this block
                    let s_new = self
                        .neighbours(x, y, z)
                        .map(|(nx, ny, nz)| {
                            let n_block = self.data[nx][ny][nz];
                            match n_block {
                                Block::Redstone(ns) => ns.saturating_sub(1),
                                Block::Trigger(true) => 15,
                                _ => 0,
                            }
                        })
                        .max()
                        .unwrap_or(0);

                    // if signal strength has changed, update neighbours
                    if s != s_new {
                        for (nx, ny, nz) in self.neighbours(x, y, z) {
                            tick_updatable.push((nx, ny, nz));
                        }
                        self.data[x][y][z] = Block::Redstone(s_new);
                    }
                }
                Block::Solid(s) => {
                    // find biggest signal strength around this block
                    let s_new = self
                        .neighbours(x, y, z)
                        .map(|(nx, ny, nz)| {
                            let n_block = self.data[nx][ny][nz];
                            match n_block {
                                Block::Redstone(1..) => 1,
                                _ => 0,
                            }
                        })
                        .max()
                        .unwrap_or(0);

                    // if signal strength has changed, update neighbours
                    if s != s_new {
                        for (nx, ny, nz) in self.neighbours(x, y, z) {
                            tick_updatable.push((nx, ny, nz));
                        }
                        self.data[x][y][z] = Block::Solid(s_new);
                    }
                }
                _ => {}
            }
        }

        print!("");
    }

    pub fn step_with_trigger(&mut self) {
        // put redstone power on triggers
        for &(tx, ty, tz) in &self.triggers {
            self.data[tx][ty][tz] = Block::Trigger(true);
            for (nx, ny, nz) in self.neighbours(tx, ty, tz) {
                self.updatable.push((nx, ny, nz));
            }
        }

        self.step();

        // take redstone power off triggers
        for &(tx, ty, tz) in &self.triggers {
            self.data[tx][ty][tz] = Block::Trigger(false);
            for (nx, ny, nz) in self.neighbours(tx, ty, tz) {
                self.updatable.push((nx, ny, nz));
            }
        }
    }

    fn neighbours(
        &self,
        x: usize,
        y: usize,
        z: usize,
    ) -> impl Iterator<Item = (usize, usize, usize)> {
        let mut vec: heapless::Vec<(usize, usize, usize), 6> = heapless::Vec::new();

        if x != 0 {
            vec.push((x - 1, y, z)).unwrap();
        }
        if x != self.size_x - 1 {
            vec.push((x + 1, y, z)).unwrap();
        }
        if y != 0 {
            vec.push((x, y - 1, z)).unwrap();
        }
        if y != self.size_y - 1 {
            vec.push((x, y + 1, z)).unwrap();
        }
        if z != 0 {
            vec.push((x, y, z - 1)).unwrap();
        }
        if z != self.size_z - 1 {
            vec.push((x, y, z + 1)).unwrap();
        }

        vec.into_iter()
    }
}