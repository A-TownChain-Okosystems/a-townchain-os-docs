//! Deterministic input event normalization and policy.
//! Device drivers remain outside this userspace orchestration layer.

use std::collections::VecDeque;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InputKind {
    Key,
    Pointer,
    Touch,
    Gamepad,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct InputEvent {
    pub sequence: u64,
    pub kind: InputKind,
    pub code: u32,
    pub value: i32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InputError {
    QueueFull,
}

#[derive(Debug)]
pub struct InputManager {
    next_sequence: u64,
    capacity: usize,
    queue: VecDeque<InputEvent>,
}

impl InputManager {
    pub fn new(capacity: usize) -> Self {
        Self {
            next_sequence: 1,
            capacity,
            queue: VecDeque::new(),
        }
    }

    pub fn push(
        &mut self,
        kind: InputKind,
        code: u32,
        value: i32,
    ) -> Result<InputEvent, InputError> {
        if self.queue.len() >= self.capacity {
            return Err(InputError::QueueFull);
        }
        let event = InputEvent {
            sequence: self.next_sequence,
            kind,
            code,
            value,
        };
        self.next_sequence = self.next_sequence.saturating_add(1).max(1);
        self.queue.push_back(event);
        Ok(event)
    }

    pub fn pop(&mut self) -> Option<InputEvent> {
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
    fn fifo_and_sequence_are_deterministic() {
        let mut m = InputManager::new(2);
        assert_eq!(m.push(InputKind::Key, 30, 1).unwrap().sequence, 1);
        assert_eq!(m.push(InputKind::Pointer, 1, 7).unwrap().sequence, 2);
        assert_eq!(m.push(InputKind::Touch, 1, 0), Err(InputError::QueueFull));
        assert_eq!(m.pop().unwrap().sequence, 1);
    }
}
