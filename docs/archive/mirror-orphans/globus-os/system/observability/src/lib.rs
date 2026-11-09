//! Bounded deterministic metrics and tracing primitives.

use std::collections::BTreeMap;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Span {
    pub sequence: u64,
    pub name: String,
}
#[derive(Debug, Default)]
pub struct Observability {
    counters: BTreeMap<String, u64>,
    spans: Vec<Span>,
    next: u64,
}
/// Fehler bei invaliden Metrik-/Span-Namen (leer oder nur Whitespace).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ObservabilityError {
    EmptyName,
}

impl Observability {
    pub fn new() -> Self {
        Self {
            next: 1,
            ..Self::default()
        }
    }
    pub fn increment(&mut self, name: &str) -> Result<u64, ObservabilityError> {
        if name.trim().is_empty() {
            return Err(ObservabilityError::EmptyName);
        }
        let v = self.counters.entry(name.to_owned()).or_default();
        *v = v.saturating_add(1);
        Ok(*v)
    }
    pub fn counter(&self, name: &str) -> u64 {
        self.counters.get(name).copied().unwrap_or(0)
    }
    pub fn start_span(&mut self, name: &str) -> Result<u64, ObservabilityError> {
        if name.trim().is_empty() {
            return Err(ObservabilityError::EmptyName);
        }
        let id = self.next;
        self.next = self.next.saturating_add(1);
        self.spans.push(Span {
            sequence: id,
            name: name.to_owned(),
        });
        Ok(id)
    }
    pub fn spans(&self) -> impl Iterator<Item = &Span> {
        self.spans.iter()
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn metrics_and_trace_are_deterministic() {
        let mut o = Observability::new();
        assert_eq!(o.increment("boot"), Ok(1));
        assert_eq!(o.increment("boot"), Ok(2));
        assert_eq!(o.start_span("userspace"), Ok(1));
        assert_eq!(o.spans().next().unwrap().name, "userspace");
    }
}
