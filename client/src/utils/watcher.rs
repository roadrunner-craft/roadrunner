use notify::{
    event::Event as NotifyEvent, RecommendedWatcher, RecursiveMode, Result as NotifyResult,
    Watcher as NotifyWatcher,
};
use std::path::Path;
use std::sync::mpsc::{channel, Receiver};

pub struct Watcher {
    watcher: RecommendedWatcher,
    receiver: Receiver<NotifyResult<NotifyEvent>>,
}

impl Watcher {
    pub fn new(path: &Path) -> Self {
        let (tx, rx) = channel();

        let mut watcher: RecommendedWatcher =
            NotifyWatcher::new_immediate(move |res| tx.send(res).unwrap()).unwrap();

        watcher
            .watch(path, RecursiveMode::Recursive)
            .expect(format!("<watcher> could not create watcher for {:?}", path).as_str());

        Self {
            watcher,
            receiver: rx,
        }
    }

    pub fn poll(&self) -> bool {
        if self.receiver.try_recv().is_ok() {
            // clear the watcher channel
            loop {
                if self.receiver.try_recv().is_err() {
                    break;
                }
            }

            true
        } else {
            false
        }
    }
}
