use std::collections::HashMap;
use std::sync::{mpsc, Arc, Mutex};
use std::sync::mpsc::{Receiver, Sender};
use crate::infrastructure::broadcasting::mpsc::error::{NoEntryWasFoundError, SendOnClosedChannelError};

pub trait Channel<T>: Send + Sync {
    // adds a new one pair of sender and receiver by key
    fn add(&self, key: String);
    // sends the data to one receiver by key or to all receivers in case when key is None
    fn send(&self, key: Option<String>, data: T) -> Result<(), SendOnClosedChannelError>;
    // removes the target pair of sender and receiver by key
    fn remove(&self, key: String);
    // returns a target sender by key
    fn sender(&self, key: String) -> Result<Sender<T>, NoEntryWasFoundError>;
    // returns a target receiver by key
    fn receiver(&self, key: String) -> Result<Receiver<T>, NoEntryWasFoundError>;
}

pub struct Chan<T> {
    map: Arc<Mutex<HashMap<String, (Sender<T>, Receiver<T>)>>>,
}

impl <T> Chan<T> {
    pub fn new() -> Chan<T> {
        let map: HashMap<String, (Sender<T>, Receiver<T>)> = HashMap::new();
        Self { map: Arc::new(Mutex::new(map)) }
    }
}

impl <T> Channel<T> for Chan<T> {
    fn add(&self, key: String) {
        let (s, r) = mpsc::channel::<T>();
        // unwrap here is safe due to only already exists concurrency problems can have affect it
        // for example, if other thread was locked mutex and had failed without unlocking it (deadlock).
        let mut map = self.map.lock().unwrap();

        if map.contains_key(&key) {
            return;
        }

        map.insert(key, (s, r));
    }

    fn send(&self, key: Option<String>, data: Arc<T>) -> Result<(), SendOnClosedChannelError> {
        if let Some(key) = key {
            let mut map = self.map.lock().unwrap();
            let sender = map.get(&key).unwrap().0.clone();
            if let Err(e) = sender.send(Some(data.clone())) {
                return Err(SendOnClosedChannelError::new(e.to_string()));
            }
        } else {
            for included_map in self.map.lock().unwrap().iter() {
                let sender = included_map.1.0.clone();
                if let Err(e) = sender.send(Some(data.clone())) {
                    return Err(SendOnClosedChannelError::new(e.to_string()));
                }
            }
        }
        Ok(())
    }

    fn remove(&self, key: String) {
        self.map.lock().unwrap().remove(&key);
    }

    fn sender(&self, key: String) -> Result<Sender<T>, NoEntryWasFoundError> {
        let mut map = self.map.lock().unwrap();

        if !map.contains_key(&key) {
            return Err(NoEntryWasFoundError::new());
        }

        Ok(map.get(&key).unwrap().0.clone())
    }

    fn receiver(&self, key: String) -> Result<Receiver<T>, NoEntryWasFoundError> {
        let mut map = self.map.lock().unwrap();

        if !map.contains_key(&key) {
            return Err(NoEntryWasFoundError::new());
        }

        Ok(map.get(&key).unwrap().1.clone())
    }
}