use crate::infrastructure::broadcasting::mpsc::error::{
    AlreadyExistsError, NoEntryWasFoundError, SendOnClosedChannelError,
};
use std::collections::HashMap;
use std::sync::mpsc::{Receiver, Sender};
use std::sync::{mpsc, Arc, Mutex};

pub trait Channel<T: Send + Sync + Clone>: Send + Sync {
    // adds a new one receiver by key
    fn add(&self, key: String) -> Result<Receiver<T>, AlreadyExistsError>;
    // sends the data to one receiver by key or to all receivers in case when key is
    // None
    fn send(&self, key: Option<String>, data: T) -> Result<(), SendOnClosedChannelError>;
    // removes the target receiver by key
    fn remove(&self, key: String);
    // returns a target sender by key
    fn sender(&self, key: Option<String>) -> Result<Vec<Arc<Sender<T>>>, NoEntryWasFoundError>;
}

pub struct Chan<T> {
    map: Arc<Mutex<HashMap<String, Arc<Sender<T>>>>>,
}

impl<T> Chan<T> {
    pub fn new() -> Chan<T> {
        let map: HashMap<String, Arc<Sender<T>>> = HashMap::new();
        Self {
            map: Arc::new(Mutex::new(map)),
        }
    }
}

impl<T: Send + Sync + Clone> Channel<T> for Chan<T> {
    fn add(&self, key: String) -> Result<Receiver<T>, AlreadyExistsError> {
        let (sender, receiver) = mpsc::channel::<T>();
        // unwrap here is safe due to only already exists concurrency problems can have
        // affect it for example, if other thread was locked mutex and had
        // failed without unlocking it (deadlock).
        let mut map = self.map.lock().unwrap();

        if map.contains_key(&key) {
            return Err(AlreadyExistsError::new(
                "map key already exists".to_string(),
            ));
        }

        map.insert(key, Arc::new(sender));

        Ok(receiver)
    }

    fn send(&self, key: Option<String>, data: T) -> Result<(), SendOnClosedChannelError> {
        if let Some(key) = key {
            let mut map = self.map.lock().unwrap();
            let sender = map.get(&key).unwrap();
            if let Err(e) = sender.send(data.clone()) {
                return Err(SendOnClosedChannelError::new(e.to_string()));
            }
        } else {
            for included_map in self.map.lock().unwrap().iter() {
                let sender = included_map.1.clone();
                if let Err(e) = sender.send(data.clone()) {
                    return Err(SendOnClosedChannelError::new(e.to_string()));
                }
            }
        }
        Ok(())
    }

    fn remove(&self, key: String) {
        self.map.lock().unwrap().remove(&key);
    }

    fn sender(&self, key: Option<String>) -> Result<Vec<Arc<Sender<T>>>, NoEntryWasFoundError> {
        let mut map = self.map.lock().unwrap();

        if let Some(key) = key {
            if !map.contains_key(&key) {
                return Err(NoEntryWasFoundError::new());
            }
            Ok(vec![map.get(&key).unwrap().clone()])
        } else {
            let mut senders = Vec::with_capacity(map.len());
            for (_, map) in self.map.lock().unwrap().iter() {
                senders.push(map.clone());
            }
            Ok(senders)
        }
    }
}
