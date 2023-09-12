/// The minimum voxel and size of a quad, without an orientation. To get the
/// actual corners of the quad, combine with an [`OrientedBlockFace`].
///
/// When using these values for materials and lighting, you can access them
/// using either the quad's minimum voxel coordinates or the vertex coordinates
/// given by `OrientedBlockFace::quad_corners`.
#[derive(Clone, Copy, Debug)]
pub struct UnorientedQuad {
    /// The minimum voxel in the quad.
    pub minimum: [f32; 3],
    /// Width of the quad.
    pub width: f32,
    /// Height of the quad.
    pub height: f32,
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
