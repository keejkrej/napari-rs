use std::collections::BTreeMap;
use std::fmt;
use std::str::FromStr;
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct TaskStatusId(u64);

impl TaskStatusId {
    pub fn get(self) -> u64 {
        self.0
    }
}

impl From<u64> for TaskStatusId {
    fn from(value: u64) -> Self {
        Self(value)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Status {
    Pending,
    Busy,
    Completed,
    Cancelled,
    Failed,
    StartFailed,
}

impl Status {
    pub fn is_active(self) -> bool {
        matches!(self, Self::Pending | Self::Busy)
    }
}

impl fmt::Display for Status {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(match self {
            Self::Pending => "pending",
            Self::Busy => "busy",
            Self::Completed => "completed",
            Self::Cancelled => "cancelled",
            Self::Failed => "failed",
            Self::StartFailed => "start_failed",
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ParseStatusError;

impl fmt::Display for ParseStatusError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("invalid task status")
    }
}

impl std::error::Error for ParseStatusError {}

impl FromStr for Status {
    type Err = ParseStatusError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value.to_ascii_lowercase().as_str() {
            "pending" => Ok(Self::Pending),
            "busy" => Ok(Self::Busy),
            "completed" => Ok(Self::Completed),
            "cancelled" => Ok(Self::Cancelled),
            "failed" => Ok(Self::Failed),
            "start_failed" => Ok(Self::StartFailed),
            _ => Err(ParseStatusError),
        }
    }
}

pub struct TaskStatusItem {
    pub id: TaskStatusId,
    provider: String,
    timestamps: Vec<String>,
    statuses: Vec<Status>,
    descriptions: Vec<String>,
    cancel_callback: Option<Box<dyn FnMut() -> bool>>,
}

impl TaskStatusItem {
    pub fn new(
        id: TaskStatusId,
        provider: impl Into<String>,
        status: Status,
        description: impl Into<String>,
        cancel_callback: Option<Box<dyn FnMut() -> bool>>,
    ) -> Self {
        Self {
            id,
            provider: provider.into(),
            timestamps: vec![timestamp()],
            statuses: vec![status],
            descriptions: vec![description.into()],
            cancel_callback,
        }
    }

    pub fn update(&mut self, status: Status, description: impl Into<String>) {
        self.timestamps.push(timestamp());
        self.statuses.push(status);
        self.descriptions.push(description.into());
    }

    pub fn cancel(&mut self) -> bool {
        self.update(Status::Cancelled, "");
        self.cancel_callback
            .as_mut()
            .is_some_and(|callback| callback())
    }

    pub fn state(&self) -> (&str, &str, Status, &str) {
        (
            &self.provider,
            self.timestamps
                .last()
                .expect("task status item always has one timestamp"),
            *self
                .statuses
                .last()
                .expect("task status item always has one status"),
            self.descriptions
                .last()
                .expect("task status item always has one description"),
        )
    }

    pub fn status_history(&self) -> &[Status] {
        &self.statuses
    }

    pub fn description_history(&self) -> &[String] {
        &self.descriptions
    }
}

impl fmt::Display for TaskStatusItem {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let (provider, timestamp, status, description) = self.state();
        write!(
            formatter,
            "TaskStatusItem: ({provider}, {}, {timestamp}, {status}, {description})",
            self.id.get()
        )
    }
}

#[derive(Default)]
pub struct TaskStatusManager {
    next_id: u64,
    tasks: BTreeMap<TaskStatusId, TaskStatusItem>,
}

impl TaskStatusManager {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn register_task_status(
        &mut self,
        provider: impl Into<String>,
        task_status: Status,
        description: impl Into<String>,
        cancel_callback: Option<Box<dyn FnMut() -> bool>>,
    ) -> TaskStatusId {
        self.next_id += 1;
        let id = TaskStatusId(self.next_id);
        let item = TaskStatusItem::new(id, provider, task_status, description, cancel_callback);
        self.tasks.insert(id, item);
        id
    }

    pub fn update_task_status(
        &mut self,
        status_id: TaskStatusId,
        task_status: Status,
        description: impl Into<String>,
    ) -> bool {
        if let Some(item) = self.tasks.get_mut(&status_id) {
            item.update(task_status, description);
            true
        } else {
            false
        }
    }

    pub fn is_busy(&self) -> bool {
        self.tasks.values().any(|item| item.state().2.is_active())
    }

    pub fn get_status(&self) -> Vec<String> {
        self.tasks
            .values()
            .filter_map(|item| {
                let (provider, _, status, description) = item.state();
                status
                    .is_active()
                    .then(|| format!("{provider}: {description}"))
            })
            .collect()
    }

    pub fn cancel_all(&mut self) {
        for item in self.tasks.values_mut() {
            item.cancel();
        }
    }

    pub fn get(&self, id: TaskStatusId) -> Option<&TaskStatusItem> {
        self.tasks.get(&id)
    }

    pub fn len(&self) -> usize {
        self.tasks.len()
    }

    pub fn is_empty(&self) -> bool {
        self.tasks.is_empty()
    }
}

fn timestamp() -> String {
    match SystemTime::now().duration_since(UNIX_EPOCH) {
        Ok(duration) => format!("{}.{:09}", duration.as_secs(), duration.subsec_nanos()),
        Err(error) => {
            let duration = error.duration();
            format!("-{}.{:09}", duration.as_secs(), duration.subsec_nanos())
        }
    }
}
