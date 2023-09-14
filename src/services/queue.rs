use actix_web::web::Data;
use apalis::prelude::{Job, JobId, Storage};
use apalis::redis::RedisStorage;
use log::error;
use serde::de::DeserializeOwned;
use serde::Serialize;

pub async fn dispatch<T>(queue: Data<RedisStorage<T>>, arg: T)
    where
    T: Job + Serialize + DeserializeOwned + Send + Unpin + Sync + 'static
{
    let storage = &*queue.into_inner();
    let mut storage = storage.clone();
    let result = storage.push(arg).await;
    match result {
        Ok(_) => {},
        Err(e) => error!("Error trying to dispatch job {:?}", e)
    }
}
