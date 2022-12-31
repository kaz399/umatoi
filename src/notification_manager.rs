use anyhow::Result;
use log::{debug, error};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use thiserror::Error;
use uuid::Uuid;

pub type HandlerFunction<T> = Box<dyn Fn(T) + Send + Sync + 'static>;

pub struct NotificationManager<T> {
    order: Arc<Mutex<Vec<uuid::Uuid>>>,
    pub handlers: Arc<Mutex<HashMap<uuid::Uuid, HandlerFunction<T>>>>,
}

#[derive(Error, Debug, PartialEq, Eq)]
pub enum NotificationManagerError {
    #[error("handler function '{0}' is not found")]
    HandlerNotFound(uuid::Uuid),
    #[error("handler name '{0}' is already used (same handler?)")]
    HandlerNameIsUsed(uuid::Uuid),
    #[error("internal error of handler.rs")]
    FoundBug,
}

impl<T> Default for NotificationManager<T> {
    fn default() -> Self {
        Self {
            order: Arc::new(Mutex::new(Vec::new())),
            handlers: Arc::new(Mutex::new(HashMap::new())),
        }
    }
}

impl<T> NotificationManager<T>
where
    T: Clone + Send + Sync,
{
    pub fn new() -> Self {
        NotificationManager::default()
    }

    /// register notification handler
    pub fn register(&self, func: HandlerFunction<T>) -> Result<uuid::Uuid> {
        let id = Uuid::new_v4();
        debug!("uuid: {}", id);
        let order_binding = self.order.clone();
        let mut order = order_binding.lock().unwrap();
        if !order.contains(&id) {
            let handlers_binding = self.handlers.clone();
            let mut handlers = handlers_binding.lock().unwrap();
            order.push(id);
            handlers.insert(id, func);
            Ok(id)
        } else {
            Err(NotificationManagerError::HandlerNameIsUsed(id).into())
        }
    }

    /// unregister notification handler
    pub fn unregister(&self, id: uuid::Uuid) -> Result<bool> {
        let order_binding = self.order.clone();
        let mut order = order_binding.lock().unwrap();
        for (index, registered_id) in order.iter().enumerate() {
            if id == *registered_id {
                let handlers_binding = self.handlers.clone();
                let mut handlers = handlers_binding.lock().unwrap();
                handlers.remove(registered_id);
                order.remove(index);
                return Ok(true);
            }
        }
        Err(NotificationManagerError::HandlerNotFound(id).into())
    }

    /// invoke all handlers
    pub fn invoke_all_handlers(&self, data: T) -> Result<bool> {
        let order_binding = self.order.clone();
        let order = order_binding.lock().unwrap();
        let handlers_binding = self.handlers.clone();
        let handlers = handlers_binding.lock().unwrap();
        for id in order.iter() {
            debug!("invoke handler {}", id);
            if let Some(handler) = handlers.get(id) {
                handler(data.clone());
            } else {
                return Err(NotificationManagerError::FoundBug.into());
            }
        }
        Ok(true)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const NOTIFY_DATA_ARRAY: [u8; 9] = [1, 2, 3, 4, 5, 66, 77, 88, 99];

    fn _setup() {
        let _ = env_logger::builder().is_test(true).try_init();
    }

    fn func1(data: Vec<u8>) {
        println!("func 1 {:?}", data);
        assert_eq!(NOTIFY_DATA_ARRAY.to_vec(), *data);
    }

    fn func2(data: Vec<u8>) {
        println!("func 2 {:?}", data);
        assert_eq!(NOTIFY_DATA_ARRAY.to_vec(), *data);
    }

    fn func3(data: Vec<u8>) {
        println!("func 3 {:?}", data);
        assert_eq!(NOTIFY_DATA_ARRAY.to_vec(), *data);
    }

    #[test]
    fn notification_manager_register() {
        _setup();
        let mut notification_manager: NotificationManager<Vec<u8>> = NotificationManager::new();

        let _handler1 = notification_manager.register(Box::new(&func1)).unwrap();
        let _handler2 = notification_manager.register(Box::new(&func2)).unwrap();
        let _handler3 = notification_manager.register(Box::new(&func3)).unwrap();

        assert_eq!(notification_manager.handlers.len(), 3);
    }

    #[test]
    fn notification_manager_unregister1() {
        _setup();
        let mut notification_manager: NotificationManager<Vec<u8>> = NotificationManager::new();

        let handler1 = notification_manager.register(Box::new(&func1)).unwrap();
        let handler2 = notification_manager.register(Box::new(&func2)).unwrap();
        let handler3 = notification_manager.register(Box::new(&func3)).unwrap();

        assert_eq!(notification_manager.handlers.len(), 3);

        notification_manager.unregister(handler3).unwrap();
        notification_manager.unregister(handler2).unwrap();
        notification_manager.unregister(handler1).unwrap();

        assert_eq!(notification_manager.handlers.len(), 0);
    }

    #[test]
    fn notification_manager_unregister2() {
        _setup();
        let mut notification_manager: NotificationManager<Vec<u8>> = NotificationManager::new();

        let handler1 = notification_manager.register(Box::new(&func1)).unwrap();
        let handler2 = notification_manager.register(Box::new(&func2)).unwrap();
        let handler3 = notification_manager.register(Box::new(&func3)).unwrap();

        assert_eq!(notification_manager.handlers.len(), 3);

        notification_manager.unregister(handler1).unwrap();
        notification_manager.unregister(handler2).unwrap();
        notification_manager.unregister(handler3).unwrap();

        assert_eq!(notification_manager.handlers.len(), 0);
    }

    #[test]
    fn notification_manager_invoke() {
        _setup();
        let mut notification_manager: NotificationManager<Vec<u8>> = NotificationManager::new();

        let handler1 = notification_manager.register(Box::new(&func1)).unwrap();
        let handler2 = notification_manager.register(Box::new(&func2)).unwrap();
        let handler3 = notification_manager.register(Box::new(&func3)).unwrap();

        assert_eq!(notification_manager.handlers.len(), 3);

        let data = Vec::from(NOTIFY_DATA_ARRAY);
        let result = notification_manager.invoke_all_handlers(data);
        assert!(result.is_ok());

        notification_manager.unregister(handler1).unwrap();
        notification_manager.unregister(handler2).unwrap();
        notification_manager.unregister(handler3).unwrap();

        assert_eq!(notification_manager.handlers.len(), 0);
    }
}
