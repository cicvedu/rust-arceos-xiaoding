use alloc::{collections::VecDeque, sync::Arc};
use core::ops::Deref;
use core::sync::atomic::{AtomicIsize, Ordering};


use crate::BaseScheduler;

const MAX_TIME_SLICE: isize = 512;

/// A task wrapper for the [`SimpleScheduler`].
pub struct SimpleTask<T> {
    inner: T,
    time_slice: AtomicIsize
}

impl<T> SimpleTask<T> {
    /// Creates a new [`SimpleTask`] from the inner task struct.
    pub const fn new(inner: T) -> Self {
        Self {
            inner,
            time_slice: AtomicIsize::new(MAX_TIME_SLICE)
        }
    }

    /// Returns a reference to the inner task struct.
    pub const fn inner(&self) -> &T {
        &self.inner
    }

    /// Get time_slice
    fn times_slice(&self) -> isize {
        return self.time_slice.load(Ordering::Acquire)
    }

    /// Reset time_slice to default
    fn reset_time_slice(&self) {
        self.time_slice.store(MAX_TIME_SLICE, Ordering::Release);
    }
}

impl<T> Deref for SimpleTask<T> {
    type Target = T;
    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

/// A simple scheduler.
///
/// When a task is added to the scheduler, it's placed at the end of the ready
/// queue. When picking the next task to run, the head of the ready queue is
/// taken.
///
/// As it's a cooperative scheduler, it does nothing when the timer tick occurs.
///
pub struct SimpleScheduler<T> {
    ready_queue: VecDeque<Arc<SimpleTask<T>>>,
}

impl<T> SimpleScheduler<T> {
    /// Creates a new empty [`SimpleScheduler`].
    pub const fn new() -> Self {
        Self {
            ready_queue: VecDeque::new(),
        }
    }
    /// get the name of scheduler
    pub fn scheduler_name() -> &'static str {
        "Simple"
    }
}

impl<T> BaseScheduler for SimpleScheduler<T> {
    type SchedItem = Arc<SimpleTask<T>>;

    fn init(&mut self) {}

    fn add_task(&mut self, task: Self::SchedItem) {
        trace!("######### add_task");
        self.ready_queue.push_back(task);
    }

    fn remove_task(&mut self, task: &Self::SchedItem) -> Option<Self::SchedItem> {
        trace!("######### remove_task");
        self.ready_queue
            .iter()
            .position(|t| Arc::ptr_eq(t, task))
            .and_then(|idx| self.ready_queue.remove(idx))
    }

    fn pick_next_task(&mut self) -> Option<Self::SchedItem> {
        trace!("######### pick_next_task");
        self.ready_queue.pop_front()
    }

    fn put_prev_task(&mut self, prev: Self::SchedItem, preempt: bool) {
        // task has time slice && preempt mode
        if prev.times_slice() > 0 && preempt {
            // insert to queue front
            self.ready_queue.push_front(prev)
        } else {    // not enough time slice
            prev.reset_time_slice();    // reset time slice to default 
            self.ready_queue.push_back(prev)    // push back to end of queue
        }
        
    }

    fn task_tick(&mut self, current: &Self::SchedItem) -> bool {
        let old = current.time_slice.fetch_sub(1, Ordering::Release);
        return old <= 1
    }

    fn set_priority(&mut self, _task: &Self::SchedItem, _prio: isize) -> bool {
        false
    }
}
