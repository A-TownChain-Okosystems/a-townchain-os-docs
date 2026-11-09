//! Per-user input/display session ownership.
use crate::SurfaceId;
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SessionId(pub u64);
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SessionState {
    Starting,
    Active,
    Locked,
    Stopped,
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DeviceSession {
    pub id: SessionId,
    pub user_id: u64,
    pub state: SessionState,
    pub focused: Option<SurfaceId>,
}
#[derive(Debug, Default)]
pub struct SessionManager {
    sessions: Vec<DeviceSession>,
    next_id: u64,
}
impl SessionManager {
    pub fn new() -> Self {
        Self {
            sessions: Vec::new(),
            next_id: 1,
        }
    }
    pub fn create(&mut self, user_id: u64) -> SessionId {
        let id = SessionId(self.next_id);
        self.next_id += 1;
        self.sessions.push(DeviceSession {
            id,
            user_id,
            state: SessionState::Starting,
            focused: None,
        });
        id
    }
    pub fn activate(&mut self, id: SessionId) -> bool {
        if let Some(s) = self.sessions.iter_mut().find(|s| s.id == id) {
            s.state = SessionState::Active;
            true
        } else {
            false
        }
    }
    pub fn lock_user(&mut self, u: u64) {
        for s in &mut self.sessions {
            if s.user_id == u {
                s.state = SessionState::Locked;
                s.focused = None
            }
        }
    }
    pub fn set_focus(&mut self, id: SessionId, s: SurfaceId) -> bool {
        if let Some(x) = self
            .sessions
            .iter_mut()
            .find(|x| x.id == id && x.state == SessionState::Active)
        {
            x.focused = Some(s);
            true
        } else {
            false
        }
    }
    pub fn sessions(&self) -> &[DeviceSession] {
        &self.sessions
    }
}
