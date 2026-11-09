//! Bounded userspace IPC endpoints.
//! Capability checks and transport execution remain kernel/ShivaCore concerns.

use globus_process::ProcessId;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct EndpointId(pub u64);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Message {
    pub sender: ProcessId,
    pub correlation: u64,
    pub value: u64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IpcError {
    UnknownEndpoint,
    PermissionDenied,
    QueueFull,
    Empty,
}

#[derive(Debug)]
pub struct Endpoint {
    pub id: EndpointId,
    pub owner: ProcessId,
    capacity: usize,
    queue: Vec<Message>,
}

#[derive(Debug, Default)]
pub struct IpcManager {
    next_id: u64,
    endpoints: Vec<Endpoint>,
}

impl IpcManager {
    pub fn new() -> Self {
        Self {
            next_id: 1,
            endpoints: Vec::new(),
        }
    }

    pub fn create(&mut self, owner: ProcessId, capacity: usize) -> Result<EndpointId, IpcError> {
        if capacity == 0 {
            return Err(IpcError::QueueFull);
        }
        let id = EndpointId(self.next_id);
        self.next_id = self.next_id.saturating_add(1).max(1);
        self.endpoints.push(Endpoint {
            id,
            owner,
            capacity,
            queue: Vec::new(),
        });
        Ok(id)
    }

    pub fn send(
        &mut self,
        endpoint: EndpointId,
        sender: ProcessId,
        correlation: u64,
        value: u64,
    ) -> Result<(), IpcError> {
        let e = self
            .endpoints
            .iter_mut()
            .find(|e| e.id == endpoint)
            .ok_or(IpcError::UnknownEndpoint)?;
        if e.queue.len() >= e.capacity {
            return Err(IpcError::QueueFull);
        }
        e.queue.push(Message {
            sender,
            correlation,
            value,
        });
        Ok(())
    }

    pub fn receive(
        &mut self,
        endpoint: EndpointId,
        receiver: ProcessId,
    ) -> Result<Message, IpcError> {
        let e = self
            .endpoints
            .iter_mut()
            .find(|e| e.id == endpoint)
            .ok_or(IpcError::UnknownEndpoint)?;
        if e.owner != receiver {
            return Err(IpcError::PermissionDenied);
        }
        if e.queue.is_empty() {
            return Err(IpcError::Empty);
        }
        Ok(e.queue.remove(0))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn endpoint_is_bounded_and_owner_gated() {
        let mut ipc = IpcManager::new();
        let owner = ProcessId(1);
        let other = ProcessId(2);
        let ep = ipc.create(owner, 1).unwrap();
        ipc.send(ep, other, 9, 42).unwrap();
        assert_eq!(ipc.send(ep, other, 10, 43), Err(IpcError::QueueFull));
        assert_eq!(ipc.receive(ep, other), Err(IpcError::PermissionDenied));
        assert_eq!(ipc.receive(ep, owner).unwrap().value, 42);
    }
}
