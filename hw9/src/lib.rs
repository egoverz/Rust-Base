#![deny(unsafe_code)]
use crate::sync::{Arc, Condvar, Mutex, thread};

#[cfg(feature = "loom")]
pub mod sync {
    pub use loom::sync::{Arc, Condvar, Mutex};
    pub use loom::thread;
}

#[cfg(not(feature = "loom"))]
pub mod sync {
    pub use std::sync::{Arc, Condvar, Mutex};
    pub use std::thread;
}

pub type Task = fn(i64);
struct Shared {
    state: Mutex<State>,
    has_work: Condvar,
}

struct State {
    queue: Vec<TaskItem>,
    shutting_down: bool,
}

struct TaskItem {
    num: i64,
}

pub struct ThreadPool {
    shared: Arc<Shared>,
    workers: Vec<thread::JoinHandle<()>>,
    task: Task,
}

impl ThreadPool {
    /// Create a pool with `worker_count` workers.
    ///
    /// # Panics
    ///
    /// Should panic when `worker_count == 0`.
    pub fn new(worker_count: usize, task: Task) -> Self {
        if worker_count == 0 {
            panic!("worker_count must > 0");
        }

        let state = State {
            queue: Vec::new(),
            shutting_down: false,
        };

        let shared = Arc::new(Shared {
            state: Mutex::new(state),
            has_work: Condvar::new(),
        });

        let mut handles = Vec::new();
        for i in 0..worker_count {
            let shared_clone = shared.clone();
            let handle = thread::spawn(move || {
                loop {
                    let maybe_num = {
                        let mut state = shared_clone.state.lock().expect("mutex poisoned");
                        while state.queue.is_empty() && !state.shutting_down {
                            state = shared_clone.has_work.wait(state).expect("mutex poisoned");
                        }

                        if let Some(num) = state.queue.pop() {
                            Some(num)
                        } else if state.shutting_down {
                            None
                        } else {
                            continue;
                        }
                    };

                    match maybe_num {
                        Some(num) => task(num.num),
                        None => break,
                    }
                }
            });
            handles.push(handle);
        }

        ThreadPool {
            shared,
            workers: handles,
            task,
        }

        // let _ = (worker_count, task);
        // todo!("create shared queue, spawn workers, and return ThreadPool")
    }

    /// Add one number to the work queue.
    pub fn execute(&self, num: i64) {
        let mut state = self.shared.state.lock().expect("mutex poisoned");
        state.queue.push(TaskItem { num });
        self.shared.has_work.notify_one();
    }

    /// Finish all queued work and stop all workers.
    pub fn shutdown(self) {
        {
            let mut state = self.shared.state.lock().expect("mutex poisoned");
            state.shutting_down = true;
            self.shared.has_work.notify_all();
        }

        for worker in self.workers {
            worker.join().expect("worked panicked");
        }

    }
}
