//! Bounded, deterministic notification queue.

use std::collections::VecDeque;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct NotificationId(pub u64);
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Notification {
    pub id: NotificationId,
    pub priority: u8,
    pub message: String,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NotificationError {
    EmptyMessage,
    QueueFull,
}

#[derive(Debug)]
pub struct NotificationManager {
    next_id: u64,
    capacity: usize,
    queue: VecDeque<Notification>,
}

impl NotificationManager {
    pub fn new(capacity: usize) -> Self {
        Self {
            next_id: 1,
            capacity,
            queue: VecDeque::new(),
        }
    }
    pub fn push(
        &mut self,
        priority: u8,
        message: impl Into<String>,
    ) -> Result<NotificationId, NotificationError> {
        let message = message.into();
        if message.trim().is_empty() {
            return Err(NotificationError::EmptyMessage);
        }
        if self.queue.len() >= self.capacity {
            return Err(NotificationError::QueueFull);
        }
        let id = NotificationId(self.next_id);
        self.next_id = self.next_id.saturating_add(1);
        self.queue.push_back(Notification {
            id,
            priority,
            message,
        });
        Ok(id)
    }
    pub fn pop(&mut self) -> Option<Notification> {
        self.queue.pop_front()
    }
    pub fn len(&self) -> usize {
        self.queue.len()
    }
    pub fn is_empty(&self) -> bool {
        self.queue.is_empty()
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn bounded_fifo() {
        let mut m = NotificationManager::new(1);
        assert_eq!(m.push(1, "hello").unwrap(), NotificationId(1));
        assert_eq!(m.push(1, "x"), Err(NotificationError::QueueFull));
        assert_eq!(m.pop().unwrap().id, NotificationId(1));
    }
}
