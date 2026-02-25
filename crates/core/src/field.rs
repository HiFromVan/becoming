use serde::{Deserialize, Serialize};

/// 场 — 宇宙中每一个位置都携带的量
///
/// 温度是场，压力是场，引力势是场。
/// 生命出现之前，宇宙只有场在演化。
pub trait Field: Clone + Send + Sync {
    type Value: Clone + Send + Sync;

    fn width(&self) -> u32;
    fn height(&self) -> u32;
    fn get(&self, x: u32, y: u32) -> &Self::Value;
    fn get_mut(&mut self, x: u32, y: u32) -> &mut Self::Value;
    fn set(&mut self, x: u32, y: u32, value: Self::Value);
}

/// 标量场 — 每个位置一个数
#[derive(Clone, Serialize, Deserialize)]
pub struct ScalarField {
    pub width: u32,
    pub height: u32,
    pub data: Vec<f32>,
}

impl ScalarField {
    pub fn zeros(width: u32, height: u32) -> Self {
        Self {
            width,
            height,
            data: vec![0.0; (width * height) as usize],
        }
    }

    pub fn filled(width: u32, height: u32, value: f32) -> Self {
        Self {
            width,
            height,
            data: vec![value; (width * height) as usize],
        }
    }
}

impl Field for ScalarField {
    type Value = f32;

    fn width(&self) -> u32 { self.width }
    fn height(&self) -> u32 { self.height }

    fn get(&self, x: u32, y: u32) -> &f32 {
        &self.data[(y * self.width + x) as usize]
    }

    fn get_mut(&mut self, x: u32, y: u32) -> &mut f32 {
        &mut self.data[(y * self.width + x) as usize]
    }

    fn set(&mut self, x: u32, y: u32, value: f32) {
        self.data[(y * self.width + x) as usize] = value;
    }
}

/// 向量场 — 每个位置一个方向和大小
#[derive(Clone, Serialize, Deserialize)]
pub struct VectorField {
    pub width: u32,
    pub height: u32,
    pub data: Vec<[f32; 2]>,
}

impl VectorField {
    pub fn zeros(width: u32, height: u32) -> Self {
        Self {
            width,
            height,
            data: vec![[0.0, 0.0]; (width * height) as usize],
        }
    }
}

impl Field for VectorField {
    type Value = [f32; 2];

    fn width(&self) -> u32 { self.width }
    fn height(&self) -> u32 { self.height }

    fn get(&self, x: u32, y: u32) -> &[f32; 2] {
        &self.data[(y * self.width + x) as usize]
    }

    fn get_mut(&mut self, x: u32, y: u32) -> &mut [f32; 2] {
        &mut self.data[(y * self.width + x) as usize]
    }

    fn set(&mut self, x: u32, y: u32, value: [f32; 2]) {
        self.data[(y * self.width + x) as usize] = value;
    }
}
