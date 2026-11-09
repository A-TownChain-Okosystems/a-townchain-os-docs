//! Deterministic, bounded event bus for GlobusOS control-plane events.

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct EventId(pub u64);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Event {
    pub id: EventId,
    pub kind: u16,
    pub value: u64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EventError {
    QueueFull,
    Empty,
}

#[derive(Debug)]
pub struct EventBus {
    capacity: usize,
    next_id: u64,
    queue: Vec<Event>,
}

impl EventBus {
    pub fn new(capacity: usize) -> Result<Self, EventError> {
        if capacity == 0 {
            return Err(EventError::QueueFull);
        }
        Ok(Self {
            capacity,
            next_id: 1,
            queue: Vec::new(),
        })
    }

    pub fn publish(&mut self, kind: u16, value: u64) -> Result<EventId, EventError> {
        if self.queue.len() >= self.capacity {
            return Err(EventError::QueueFull);
        }
        let id = EventId(self.next_id);
        self.next_id = self.next_id.saturating_add(1).max(1);
        self.queue.push(Event { id, kind, value });
        Ok(id)
    }

    pub fn receive(&mut self) -> Result<Event, EventError> {
        if self.queue.is_empty() {
            return Err(EventError::Empty);
        }
        Ok(self.queue.remove(0))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn preserves_publish_order_and_bounds_queue() {
        let mut bus = EventBus::new(2).unwrap();
        assert_eq!(bus.publish(1, 10).unwrap(), EventId(1));
        assert_eq!(bus.publish(2, 20).unwrap(), EventId(2));
        assert_eq!(bus.publish(3, 30), Err(EventError::QueueFull));
        assert_eq!(bus.receive().unwrap().kind, 1);
        assert_eq!(bus.receive().unwrap().kind, 2);
    }
}
