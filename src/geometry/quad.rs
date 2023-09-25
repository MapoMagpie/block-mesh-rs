/// The minimum voxel and size of a quad, without an orientation. To get the
/// actual corners of the quad, combine with an [`OrientedBlockFace`].
///
/// When using these values for materials and lighting, you can access them
/// using either the quad's minimum voxel coordinates or the vertex coordinates
/// given by `OrientedBlockFace::quad_corners`.
pub trait Quad {
    /// The minimum voxel in the quad.
    fn minimum(&self) -> [f32; 3];
    /// Width of the quad.
    fn width(&self) -> f32;
    /// Height of the quad.
    fn height(&self) -> f32;
}

#[derive(Debug, Copy, Clone)]
pub struct UnorientedQuad {
    minimum: [f32; 3],
    width: f32,
    height: f32,
}

impl Quad for UnorientedQuad {
    fn minimum(&self) -> [f32; 3] {
        self.minimum
    }
    fn width(&self) -> f32 {
        self.width
    }
    fn height(&self) -> f32 {
        self.height
    }
}

impl UnorientedQuad {
    pub fn new(minimum: [f32; 3], width: f32, height: f32) -> Self {
        Self {
            minimum,
            width,
            height,
        }
    }
    pub fn from_minimum(minimum: [f32; 3]) -> Self {
        Self {
            minimum,
            width: 1.0,
            height: 1.0,
        }
    }
}
