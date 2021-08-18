use mq::math::Vec2;

pub struct Map {
    width: i16,
    height: i16,
}

impl Map {
    /// Given a screen pixel, translate it into a Pos.
    pub fn pixel_to_pos(&self, pixel: Vec2, aspect_ratio: f32) -> Option<Pos> {
        let (l, t, r, b, s) = self.layout(aspect_ratio);
        let x = pixel.x;
        let y = pixel.y;
        if x < l || x > r || y < t || y > b {
            return None
        }
        let mut pos = Pos { x: 0, y: 0 };
        pos.x = ((x - l) / s).floor() as i16;
        pos.y = ((y - t) / s).floor() as i16;
        Some(pos)
    }

    /// Given a screen aspect ratio, layout the map. 
    ///
    /// Returns (left, top, right, bottom, tile_size)
    pub fn layout(&self, aspect_ratio: f32) -> (f32, f32, f32, f32, f32) {
        // We use aspect ratio because aspect ratio is screen width / screen height
        let wsize: f32 = aspect_ratio / f32::from(self.width);
        let hsize: f32 = 1.0 / f32::from(self.height);
        let mut tile_size = wsize;
        if hsize < wsize {
            tile_size = hsize;
        }
        let l = (aspect_ratio * 2.0 - tile_size * f32::from(self.width)) / 2.0;
        let r = aspect_ratio * 2.0 - l;
        let t = (2.0 - tile_size * f32::from(self.height)) / 2.0;
        let b = 2.0 - t;
        (l, t, r, b, tile_size)
    }
}

/// Represents a position in the map
///
/// The upper left part of the map is position (0,0)
pub struct Pos {
    x: i16,
    y: i16,
}

