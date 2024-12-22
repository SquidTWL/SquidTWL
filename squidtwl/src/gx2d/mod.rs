use crate::gx::engine::{EngineA, EngineB, GraphicsEngine};

pub mod framebuffer;
pub mod tilemap;

/**
 * Wraps the two 2D engines for the NTR/TWL. This should only be created once; in the future, this
 * invariant will be enforced.
 */
pub struct Graphics2D {
    pub engine_a: GraphicsEngine<EngineA>,
    pub engine_b: GraphicsEngine<EngineB>,
}

impl Graphics2D {
    /**
     * Attempts to create a new ``Graphics2D`` instance. This function should only ever be called
     * once.
     *
     * If this function was called before, this function *should* return ``None`` on all subsequent
     * calls. Do not rely on this returning a ``Some`` on subsequent calls.
     */
    pub fn new() -> Option<Graphics2D> {
        let engine_a = GraphicsEngine::<EngineA>::new();
        let engine_b = GraphicsEngine::<EngineB>::new();

        return Some(Graphics2D { engine_a, engine_b });
    }
}
