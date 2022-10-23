use math::geometry::Rect;
use math::random::noise::NoiseFn;
use std::ops::Range;

pub struct HeightMapOptions {
    pub rect: Rect,
    pub range: Range<u8>,
}

pub struct HeightMap {
    values: Vec<u8>,
    options: HeightMapOptions,
}

impl HeightMap {
    pub fn new<N: NoiseFn<[i64; 2]>>(rect: Rect, range: Range<u8>, noise: N) -> Self {
        let height = (range.end - range.start) as f64;
        Self {
            values: (0..rect.size.y as i64)
                .flat_map(|y| {
                    (0..rect.size.x as i64)
                        .map(move |x| (rect.origin.x as i64 + x, rect.origin.y as i64 + y))
                })
                .map(|(x, y)| (noise.get([x, y]) * height) as u8 + range.start)
                .collect(),
            options: HeightMapOptions { rect, range },
        }
    }

    pub fn height(&self, x: u8, z: u8) -> u8 {
        self.values[(x + z * self.options.rect.size.x as u8) as usize]
    }
}
