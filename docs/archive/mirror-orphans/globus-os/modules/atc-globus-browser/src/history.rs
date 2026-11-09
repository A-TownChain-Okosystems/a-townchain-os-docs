use std::collections::VecDeque;

/// A single successful browser navigation entry.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HistoryEntry {
    /// Canonical URL recorded for the navigation.
    pub url: String,
}

/// Bounded-in-process navigation history for one browser instance.
#[derive(Debug, Default, Clone)]
pub struct History {
    entries: VecDeque<HistoryEntry>,
    cursor: Option<usize>,
}

impl History {
    /// Creates an empty navigation history.
    pub fn new() -> Self {
        Self::default()
    }

    /// Adds a URL and discards any forward branch.
    pub fn push(&mut self, url: impl Into<String>) {
        if let Some(cursor) = self.cursor {
            self.entries.truncate(cursor + 1);
        }
        self.entries.push_back(HistoryEntry { url: url.into() });
        self.cursor = Some(self.entries.len() - 1);
    }

    /// Returns the active history entry.
    pub fn current(&self) -> Option<&HistoryEntry> {
        self.cursor.and_then(|i| self.entries.get(i))
    }

    /// Moves one entry backward, if possible.
    pub fn back(&mut self) -> Option<&HistoryEntry> {
        let cursor = self.cursor?;
        if cursor == 0 {
            return None;
        }
        self.cursor = Some(cursor - 1);
        self.current()
    }

    /// Moves one entry forward, if possible.
    pub fn forward(&mut self) -> Option<&HistoryEntry> {
        let cursor = self.cursor?;
        let next = cursor + 1;
        if next >= self.entries.len() {
            return None;
        }
        self.cursor = Some(next);
        self.current()
    }

    /// Returns the number of retained history entries.
    pub fn len(&self) -> usize {
        self.entries.len()
    }

    /// Returns whether the history contains no entries.
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn navigation_truncates_forward_history() {
        let mut h = History::new();
        h.push("https://a.example");
        h.push("https://b.example");
        assert_eq!(h.back().map(|e| e.url.as_str()), Some("https://a.example"));
        h.push("https://c.example");
        assert!(h.forward().is_none());
        assert_eq!(h.len(), 2);
    }
}
