use crate::tasks::spawn_task_from_message;
use core::jobs::{Cancellable, Handle, LoopState};
use core::TaskMessage;
use rpiledbind::MatrixHolder;
use std::sync::mpsc;
use std::sync::mpsc::{Receiver, SendError, Sender};
use std::sync::Mutex;

lazy_static! {
    static ref TASK_MANAGER: Mutex<TaskManager> = Mutex::new(TaskManager::new());
}

pub fn send_message(payload: TaskMessage) -> Result<(), SendError<TaskMessage>> {
    TASK_MANAGER.lock().unwrap().send_message(payload)
}

pub fn start_task_manager() {
    TASK_MANAGER.lock().unwrap().start();
}

#[allow(dead_code)]
pub fn stop_task_manager() {
    TASK_MANAGER.lock().unwrap().stop();
}

#[derive(Debug, Clone)]
pub struct TaskError;

pub struct TaskManager {
    h: Option<Handle<TaskError>>,
    tx: Option<Mutex<Sender<TaskMessage>>>,
}

impl TaskManager {
    pub fn new() -> Self {
        Self { h: None, tx: None }
    }

    pub fn send_message(&self, msg: TaskMessage) -> Result<(), SendError<TaskMessage>> {
        if let Some(tx) = &self.tx {
            let t = tx.lock().unwrap().clone();
            t.send(msg)?
        }
        Ok(())
    }

    pub fn start(&mut self) {
        let (tx, rx): (Sender<TaskMessage>, Receiver<TaskMessage>) = mpsc::channel();
        let task_service = TaskService::new(rx);
        self.tx = Some(Mutex::new(tx));
        self.h = Some(task_service.spawn());
    }

    pub fn stop(&mut self) {
        // Need to move h out of self so we can consume it with
        // the h.wait()
        let self_h = std::mem::replace(&mut self.h, None);
        if let Some(h) = self_h {
            h.cancel();
            h.wait().unwrap();
            self.h = None;
            self.tx = None;
        }
    }
}

// Lets one task run and receives messages about different
// tasks to run which will cancel the current task and start
// another one.
pub struct TaskService {
    rx: Receiver<TaskMessage>,
    current_task_handle: Option<Handle<TaskError>>,
    matrix: MatrixHolder,
}

impl TaskService {
    fn new(rx: Receiver<TaskMessage>) -> Self {
        let mut matrix = MatrixHolder::new();
        matrix.lock_matrix().set_brightness(30);
        Self {
            rx,
            current_task_handle: None,
            matrix,
        }
    }
}

impl Cancellable for TaskService {
    type Error = TaskError;
    fn for_each(&mut self) -> Result<LoopState, Self::Error> {
        match self.rx.recv_timeout(std::time::Duration::from_secs(5)) {
            Ok(msg) => {
                // If an existing task exists, stop it.
                let self_h = std::mem::replace(&mut self.current_task_handle, None);
                if let Some(current_task) = self_h {
                    current_task.cancel();
                    current_task.wait().unwrap();
                }

                let next_task_handle = spawn_task_from_message(&self.matrix, &msg);
                self.current_task_handle = Some(next_task_handle);
            }
            Err(_) => {} // timed out
        }
        Ok(LoopState::Continue)
    }
}
