use std::sync::mpsc::channel;
use std::sync::mpsc::Sender;
use std::sync::Arc;
use std::sync::Mutex;
use std::thread;

type Thunk<'a> = Box<dyn FnOnce() + Send + 'a>;

pub struct ThreadPool {
    job_sender: Sender<Thunk<'static>>,
}

impl ThreadPool {
    pub fn new(capacity: usize) -> Self {
        let (tx, rx) = channel::<Thunk<'static>>();
        let mrx = Arc::new(Mutex::new(rx));
        for _ in 0..capacity {
            let job_receiver = mrx.clone();
            thread::spawn(move || loop {
                let msg = {
                    let lock = job_receiver.lock().unwrap();
                    lock.recv()
                };

                match msg {
                    Ok(f) => f(),
                    _ => (),
                }
            });
        }
        Self { job_sender: tx }
    }

    pub fn run<F>(&mut self, job: F)
    where
        F: FnOnce() + Send + 'static,
    {
        self.job_sender.send(Box::new(job)).unwrap();
    }
}
