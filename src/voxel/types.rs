use bevy::{platform::collections::HashMap, prelude::*};
pub const GRID_SIZE: usize = 16;
pub type Voxel = u8;

#[derive(Reflect, Clone, Copy, PartialEq, PartialOrd, Ord, Eq, Hash)]
pub struct ChunkPosition {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}
#[derive(Reflect, Clone)]
pub struct VoxelVolume {
    values: Vec<Voxel>,
}
#[derive(Reflect, Clone)]
pub struct VoxelChunk {
    pub position: ChunkPosition,
    pub volume: VoxelVolume,
}
#[derive(Reflect, Resource)]
pub struct VoxelWorld {
    pub chunks: HashMap<ChunkPosition, VoxelChunk>,
}

impl VoxelVolume {
    pub fn get(&self, x: u32, y: u32, z: u32) -> Option<Voxel> {
        Some(self.values[self.to_index(x, y, z)?])
    }

    pub fn to_index(&self, x: u32, y: u32, z: u32) -> Option<usize> {
        let (x, y, z) = (x as usize, y as usize, z as usize);
        if x >= GRID_SIZE || y >= GRID_SIZE || z >= GRID_SIZE {
            return None;
        }
        Some(x + y * GRID_SIZE + z * GRID_SIZE * GRID_SIZE)
    }

    pub fn set(&mut self, x: u32, y: u32, z: u32, v: Voxel) -> Option<()> {
        let index = self.to_index(x, y, z)?;
        self.values[index] = v;
        Some(())
    }
}

impl Default for VoxelVolume {
    fn default() -> Self {
        Self {
            values: vec![0; GRID_SIZE * GRID_SIZE * GRID_SIZE],
        }
    }
}
