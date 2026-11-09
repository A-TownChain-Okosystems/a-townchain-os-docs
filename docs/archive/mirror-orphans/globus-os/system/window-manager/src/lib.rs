//! Window lifecycle and geometry policy. Rendering remains owned by the compositor.

use std::collections::BTreeMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct WindowId(pub u64);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Rect {
    pub x: i32,
    pub y: i32,
    pub width: u32,
    pub height: u32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WindowState {
    Created,
    Visible,
    Minimized,
    Hidden,
    Closed,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WindowAction {
    Show,
    Minimize,
    Restore,
    Hide,
    Close,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Window {
    pub id: WindowId,
    pub owner: u64,
    pub rect: Rect,
    pub state: WindowState,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WindowError {
    DuplicateId,
    Missing,
    InvalidTransition,
    InvalidGeometry,
}

#[derive(Debug, Default)]
pub struct WindowManager {
    windows: BTreeMap<WindowId, Window>,
}

impl WindowManager {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn create(&mut self, id: WindowId, owner: u64, rect: Rect) -> Result<(), WindowError> {
        if rect.width == 0 || rect.height == 0 {
            return Err(WindowError::InvalidGeometry);
        }
        if self.windows.contains_key(&id) {
            return Err(WindowError::DuplicateId);
        }
        self.windows.insert(
            id,
            Window {
                id,
                owner,
                rect,
                state: WindowState::Created,
            },
        );
        Ok(())
    }
    pub fn apply(&mut self, id: WindowId, action: WindowAction) -> Result<(), WindowError> {
        let w = self.windows.get_mut(&id).ok_or(WindowError::Missing)?;
        match (w.state, action) {
            (WindowState::Created, WindowAction::Show)
            | (WindowState::Hidden, WindowAction::Show) => w.state = WindowState::Visible,
            (WindowState::Visible, WindowAction::Minimize) => w.state = WindowState::Minimized,
            (WindowState::Minimized, WindowAction::Restore) => w.state = WindowState::Visible,
            (WindowState::Visible | WindowState::Minimized, WindowAction::Hide) => {
                w.state = WindowState::Hidden
            }
            (
                WindowState::Created
                | WindowState::Visible
                | WindowState::Minimized
                | WindowState::Hidden,
                WindowAction::Close,
            ) => w.state = WindowState::Closed,
            _ => return Err(WindowError::InvalidTransition),
        }
        Ok(())
    }
    pub fn get(&self, id: WindowId) -> Option<&Window> {
        self.windows.get(&id)
    }
    pub fn windows(&self) -> impl Iterator<Item = &Window> {
        self.windows.values()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn lifecycle() {
        let mut m = WindowManager::new();
        m.create(
            WindowId(1),
            7,
            Rect {
                x: 0,
                y: 0,
                width: 800,
                height: 600,
            },
        )
        .unwrap();
        m.apply(WindowId(1), WindowAction::Show).unwrap();
        m.apply(WindowId(1), WindowAction::Minimize).unwrap();
        m.apply(WindowId(1), WindowAction::Restore).unwrap();
        m.apply(WindowId(1), WindowAction::Close).unwrap();
        assert_eq!(m.get(WindowId(1)).unwrap().state, WindowState::Closed);
    }
}
