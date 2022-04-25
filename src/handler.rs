use log::{debug, error};
use std::collections::HashMap;
use std::error::Error;
use thiserror::Error;
use uuid::Uuid;

pub type HandlerFunction<T> = Box<dyn Fn(T) + Send + Sync + 'static>;

pub struct NotifyManager<T> {
    order: Vec<uuid::Uuid>,
    pub handlers: HashMap<uuid::Uuid, HandlerFunction<T>>,
}

#[derive(Error, Debug, PartialEq, Eq)]
pub enum NotifyManagerError {
    #[error("handler function '{0}' is not found")]
    HandlerNotFound(uuid::Uuid),
    #[error("handler name '{0}' is already used (same handler?)")]
    HandlerNameIsUsed(uuid::Uuid),
    #[error("internal error of handler.rs")]
    FoundBug,
}

impl<T> Default for NotifyManager<T> {
    fn default() -> Self {
        Self {
            order: Vec::new(),
            handlers: HashMap::new(),
        }
    }
}

impl<T> NotifyManager<T>
where
    T: Clone + Send + Sync,
{
    pub fn new() -> Self {
        NotifyManager::default()
    }

    /// register notify handler
    pub fn register(
        &mut self,
        func: HandlerFunction<T>,
    ) -> Result<uuid::Uuid, Box<dyn Error + Send + Sync + 'static>> {
        let id = Uuid::new_v4();
        debug!("uuid: {}", id);
        if !self.order.contains(&id) {
            self.order.push(id);
            self.handlers.insert(id, func);
            Ok(id)
        } else {
            Err(NotifyManagerError::HandlerNameIsUsed(id).into())
        }
    }

    /// unregister notify handler
    pub fn unregister(
        &mut self,
        id: uuid::Uuid,
    ) -> Result<bool, Box<dyn Error + Send + Sync + 'static>> {
        for (index, registered_id) in self.order.iter().enumerate() {
            if id == *registered_id {
                self.handlers.remove(registered_id);
                self.order.remove(index);
                return Ok(true);
            }
        }
        Err(Box::new(NotifyManagerError::HandlerNotFound(id)))
    }

    /// invoke all handlers
    pub fn invoke_all_handlers(
        &self,
        data: T,
    ) -> Result<bool, Box<dyn Error + Send + Sync + 'static>> {
        for id in self.order.iter() {
            debug!("invoke handler {}", id);
            if let Some(handler) = self.handlers.get(id) {
                handler(data.clone());
            } else {
                return Err(NotifyManagerError::FoundBug.into());
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
    fn notify_manager_register() {
        _setup();
        let mut notify_manager: NotifyManager<Vec<u8>> = NotifyManager::new();

        let _handler1 = notify_manager.register(Box::new(&func1)).unwrap();
        let _handler2 = notify_manager.register(Box::new(&func2)).unwrap();
        let _handler3 = notify_manager.register(Box::new(&func3)).unwrap();

        assert_eq!(notify_manager.handlers.len(), 3);
    }

    #[test]
    fn notify_manager_unregister1() {
        _setup();
        let mut notify_manager: NotifyManager<Vec<u8>> = NotifyManager::new();

        let handler1 = notify_manager.register(Box::new(&func1)).unwrap();
        let handler2 = notify_manager.register(Box::new(&func2)).unwrap();
        let handler3 = notify_manager.register(Box::new(&func3)).unwrap();

        assert_eq!(notify_manager.handlers.len(), 3);

        notify_manager.unregister(handler3).unwrap();
        notify_manager.unregister(handler2).unwrap();
        notify_manager.unregister(handler1).unwrap();

        assert_eq!(notify_manager.handlers.len(), 0);
    }

    #[test]
    fn notify_manager_unregister2() {
        _setup();
        let mut notify_manager: NotifyManager<Vec<u8>> = NotifyManager::new();

        let handler1 = notify_manager.register(Box::new(&func1)).unwrap();
        let handler2 = notify_manager.register(Box::new(&func2)).unwrap();
        let handler3 = notify_manager.register(Box::new(&func3)).unwrap();

        assert_eq!(notify_manager.handlers.len(), 3);

        notify_manager.unregister(handler1).unwrap();
        notify_manager.unregister(handler2).unwrap();
        notify_manager.unregister(handler3).unwrap();

        assert_eq!(notify_manager.handlers.len(), 0);
    }

    #[test]
    fn notify_manager_invoke() {
        _setup();
        let mut notify_manager: NotifyManager<Vec<u8>> = NotifyManager::new();

        let handler1 = notify_manager.register(Box::new(&func1)).unwrap();
        let handler2 = notify_manager.register(Box::new(&func2)).unwrap();
        let handler3 = notify_manager.register(Box::new(&func3)).unwrap();

        assert_eq!(notify_manager.handlers.len(), 3);

        let data = Vec::from(NOTIFY_DATA_ARRAY);
        let result = notify_manager.invoke_all_handlers(data);
        assert!(result.is_ok());

        notify_manager.unregister(handler1).unwrap();
        notify_manager.unregister(handler2).unwrap();
        notify_manager.unregister(handler3).unwrap();

        assert_eq!(notify_manager.handlers.len(), 0);
    }
}
