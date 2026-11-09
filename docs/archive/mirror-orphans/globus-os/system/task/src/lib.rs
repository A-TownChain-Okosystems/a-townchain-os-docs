//! Task lifecycle facade above process/thread primitives.
use globus_process::{ProcessId, ProcessState, Scheduler};
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TaskAction {
    Start,
    Pause,
    Resume,
    Stop,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TaskError {
    UnknownTask,
    InvalidTransition,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Task {
    pub id: ProcessId,
    pub priority: u8,
    pub state: ProcessState,
}
#[derive(Debug, Default)]
pub struct TaskManager {
    tasks: Vec<Task>,
    scheduler: Scheduler,
}
impl TaskManager {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn create(&mut self, id: ProcessId, priority: u8) -> Result<(), TaskError> {
        if self.tasks.iter().any(|t| t.id == id) {
            return Err(TaskError::InvalidTransition);
        }
        self.tasks.push(Task {
            id,
            priority,
            state: ProcessState::Created,
        });
        Ok(())
    }
    pub fn apply(&mut self, id: ProcessId, a: TaskAction) -> Result<(), TaskError> {
        let t = self
            .tasks
            .iter_mut()
            .find(|t| t.id == id)
            .ok_or(TaskError::UnknownTask)?;
        t.state = match (a, t.state) {
            (TaskAction::Start, ProcessState::Created) => ProcessState::Ready,
            (TaskAction::Pause, ProcessState::Running) => ProcessState::Blocked,
            (TaskAction::Resume, ProcessState::Blocked) => ProcessState::Ready,
            (
                TaskAction::Stop,
                ProcessState::Created
                | ProcessState::Ready
                | ProcessState::Running
                | ProcessState::Blocked,
            ) => ProcessState::Exited,
            _ => return Err(TaskError::InvalidTransition),
        };
        self.scheduler.register(globus_process::ProcessInfo {
            id: t.id,
            state: t.state,
            priority: t.priority,
        });
        self.scheduler.set_state(t.id, t.state);
        Ok(())
    }
    #[allow(clippy::should_implement_trait)] // Domain-API: naechster Task, kein Iterator
    pub fn next(&mut self) -> Option<ProcessId> {
        self.scheduler.pick_next()
    }
    pub fn tasks(&self) -> &[Task] {
        &self.tasks
    }
}
