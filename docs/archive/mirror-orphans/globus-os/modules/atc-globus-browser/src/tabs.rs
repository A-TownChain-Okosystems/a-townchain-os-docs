use crate::{Browser, BrowserConfig, BrowserError};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct TabId(u64);

pub struct Tab {
    id: TabId,
    browser: Browser,
}

impl Tab {
    pub fn id(&self) -> TabId {
        self.id
    }

    pub fn browser(&mut self) -> &mut Browser {
        &mut self.browser
    }
}

#[derive(Default)]
pub struct TabManager {
    next_id: u64,
    tabs: Vec<Tab>,
    active: Option<TabId>,
}

impl TabManager {
    pub fn new() -> Self {
        Self {
            next_id: 1,
            ..Self::default()
        }
    }

    pub fn open(&mut self, config: BrowserConfig) -> Result<TabId, BrowserError> {
        let id = TabId(self.next_id);
        self.next_id = self
            .next_id
            .checked_add(1)
            .ok_or_else(|| BrowserError::ResourceLimit("tab id exhausted".into()))?;
        self.tabs.push(Tab {
            id,
            browser: Browser::new(config)?,
        });
        self.active = Some(id);
        Ok(id)
    }

    pub fn close(&mut self, id: TabId) -> bool {
        let before = self.tabs.len();
        self.tabs.retain(|tab| tab.id != id);
        if self.active == Some(id) {
            self.active = self.tabs.last().map(|tab| tab.id);
        }
        before != self.tabs.len()
    }

    pub fn active(&self) -> Option<TabId> {
        self.active
    }

    pub fn activate(&mut self, id: TabId) -> bool {
        if self.tabs.iter().any(|tab| tab.id == id) {
            self.active = Some(id);
            true
        } else {
            false
        }
    }

    pub fn get_mut(&mut self, id: TabId) -> Option<&mut Tab> {
        self.tabs.iter_mut().find(|tab| tab.id == id)
    }

    pub fn len(&self) -> usize {
        self.tabs.len()
    }

    pub fn is_empty(&self) -> bool {
        self.tabs.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tabs_have_unique_ids_and_close_updates_active() {
        let mut tabs = TabManager::new();
        let a = tabs.open(BrowserConfig::default()).expect("tab");
        let b = tabs.open(BrowserConfig::default()).expect("tab");
        assert_ne!(a, b);
        assert_eq!(tabs.active(), Some(b));
        assert!(tabs.close(b));
        assert_eq!(tabs.active(), Some(a));
    }
}
