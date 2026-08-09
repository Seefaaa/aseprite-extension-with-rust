use std::{
    cell::RefCell,
    collections::{HashMap, hash_map::Entry},
    thread,
};

type Output = String;
type JobId = String;

struct Job {
    rx: flume::Receiver<Output>,
    handle: thread::JoinHandle<()>,
}

#[derive(Default)]
struct Jobs {
    map: HashMap<JobId, Job>,
    next_job: usize,
}

impl Jobs {
    fn start<F: FnOnce() -> Output + Send + 'static>(&mut self, f: F) -> JobId {
        let (tx, rx) = flume::unbounded();

        let handle = thread::spawn(move || {
            let _ = tx.send(f());
        });

        let id = self.next_job.to_string();

        self.next_job += 1;
        self.map.insert(id.clone(), Job { rx, handle });

        id
    }

    fn check(&mut self, id: &str) -> Output {
        let entry = match self.map.entry(id.to_string()) {
            Entry::Occupied(occupied) => occupied,
            Entry::Vacant(_) => return "no such job".to_string(),
        };

        let result = match entry.get().rx.try_recv() {
            Ok(result) => result,
            Err(flume::TryRecvError::Disconnected) => "job paniced".to_string(),
            Err(flume::TryRecvError::Empty) => return "job still running".to_string(),
        };

        let _ = entry.remove().handle.join();

        result
    }
}

thread_local! {
    static JOBS: RefCell<Jobs> = RefCell::default();
}

pub fn start<F: FnOnce() -> Output + Send + 'static>(f: F) -> JobId {
    JOBS.with(|jobs| jobs.borrow_mut().start(f))
}

pub fn check(id: &str) -> Output {
    JOBS.with(|jobs| jobs.borrow_mut().check(id))
}
