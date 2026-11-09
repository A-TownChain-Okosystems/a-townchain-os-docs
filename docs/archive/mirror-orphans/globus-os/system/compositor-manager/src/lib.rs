//! Deterministic composition surface registry.
//! Actual GPU/display execution remains below this policy layer.

use std::collections::BTreeMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SurfaceId(pub u64);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Layer {
    pub z: i32,
    pub visible: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Surface {
    pub id: SurfaceId,
    pub layer: Layer,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CompositorError {
    DuplicateId,
    Missing,
}

#[derive(Debug, Default)]
pub struct CompositorManager {
    surfaces: BTreeMap<SurfaceId, Surface>,
}

impl CompositorManager {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn register(&mut self, s: Surface) -> Result<(), CompositorError> {
        if self.surfaces.contains_key(&s.id) {
            return Err(CompositorError::DuplicateId);
        }
        self.surfaces.insert(s.id, s);
        Ok(())
    }
    pub fn set_layer(&mut self, id: SurfaceId, layer: Layer) -> Result<(), CompositorError> {
        self.surfaces
            .get_mut(&id)
            .ok_or(CompositorError::Missing)?
            .layer = layer;
        Ok(())
    }
    pub fn visible_surfaces(&self) -> impl Iterator<Item = &Surface> {
        self.surfaces.values().filter(|s| s.layer.visible)
    }
    pub fn composition_order(&self) -> Vec<SurfaceId> {
        let mut v = self.visible_surfaces().collect::<Vec<_>>();
        v.sort_by_key(|s| (s.layer.z, s.id));
        v.into_iter().map(|s| s.id).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn order_is_deterministic() {
        let mut m = CompositorManager::new();
        m.register(Surface {
            id: SurfaceId(2),
            layer: Layer {
                z: 5,
                visible: true,
            },
        })
        .unwrap();
        m.register(Surface {
            id: SurfaceId(1),
            layer: Layer {
                z: 5,
                visible: true,
            },
        })
        .unwrap();
        assert_eq!(m.composition_order(), vec![SurfaceId(1), SurfaceId(2)]);
    }
}
