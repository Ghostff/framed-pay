use std::fmt::Debug;
use std::sync::Arc;
use chrono::{Utc, Duration};
use futures_util::StreamExt;
use log::{info};
use serde::Serialize;
use serde_json::json;
use sqlx::{PgPool};
use tokio::sync::{Semaphore};
use tokio::sync::mpsc::{Sender};
use crate::config::ENV;
use crate::errors::{QueueError};
use crate::models::job::{Job as JobModel, JobStatus};
use crate::repositories::job_repository as repo;
use crate::services::email::Email;

pub trait Queueable: Serialize {
    const NAME: &'static str;
    const CHANNEL: u8;
    // 0: Email
}

#[derive(Clone)]
pub struct Job {
    pool: PgPool,
    sender: Sender<u8>,
    name: Option<String>,
    priority: i16,
    max_retries: i16,
    retry_after_minutes: i16,
}

impl Job {
    pub fn new(pool: PgPool, sender: Sender<u8>) -> Job {
        Self {
            pool,
            sender,
            name: None,
            priority: 1,
            max_retries: 0,
            retry_after_minutes: 0,
        }
    }

    #[allow(unused)]
    pub fn set_name(&mut self, name: &str) -> &mut Job {
        self.name = Some(name.to_string());
        self
    }

    #[allow(unused)]
    pub fn set_priority(&mut self, priority: i16) -> &mut Job {
        self.priority = priority;
        self
    }

    #[allow(unused)]
    pub fn set_max_retries(&mut self, max_retries: i16, retry_after_minutes: i16) -> &mut Job {
        self.max_retries = max_retries;
        self.retry_after_minutes = retry_after_minutes;
        self
    }

    pub async fn queue<T: Queueable>(&self, payload: T) -> Result<(), QueueError> {
        match repo::create(
            &self.pool,
            T::NAME,
            &json!(&payload),
            &self.priority,
            &self.max_retries,
            &self.name,
            &self.retry_after_minutes,
        ).await {
            Ok(_) => match self.sender.send(T::CHANNEL).await {
                Ok(_) => Ok(()),
                Err(e) => Err(QueueError::Error(format!("{:?}", e)))
            },
            Err(e) => Err(QueueError::Error(format!("{:?}", e)))
        }
    }

    async fn job_failed<T: Debug>(error: T, mut job: JobModel, pool: &PgPool) {
        let now = Utc::now();
        job.error_message = Option::from(format!("{:?}", error));
        if job.max_retries == job.retries {
            job.completed_at = Option::from(now);
            job.status = JobStatus::FAILED.to_string()
        } else {
            job.retries += 1;
            job.status = JobStatus::QUEUED.to_string();
            job.ready_after = Option::from(now + Duration::minutes(job.retry_after_minutes as i64))
        }

        if let Err(e) = repo::update(pool, &job).await {
            log::error!("job_failed[Job:{}:{}] failed {:?}", job.job_type, job.id, e)
        };
    }

    async fn job_passed(mut job: JobModel, pool: &PgPool) {
        job.completed_at = Option::from(Utc::now());
        job.status = JobStatus::COMPLETED.to_string();
        if let Err(e) = repo::update(pool, &job).await {
            log::error!("job_passed[Job:{}:{}] failed {:?}", job.job_type, job.id, e)
        };
    }

    pub async fn dispatch_emails(&self) {
        let sem = Arc::new(Semaphore::new(ENV.max_concurrent_email_job_task));
        let mut rows = sqlx::query_as!(
            JobModel,
            "SELECT * FROM jobs WHERE job_type = $1 AND completed_at IS NULL AND NOW() > ready_after ORDER By priority DESC, created_at ASC",
            Email::NAME
        ).fetch(&self.pool);

        while let Some(row) = rows.next().await {
            match row {
                Ok(job) => {
                    // Clone the Arc to share it with the spawned task
                    let permit = sem.clone().acquire_owned().await.expect("Failed to acquire semaphore");
                    let conn = self.pool.clone();
                    tokio::task::spawn(async move {
                        info!("[Job:{}:{}] Running", job.job_type, job.id);
                        let mut email: Email = serde_json::from_value(job.payload.clone()).unwrap();
                        match email.send().await {
                            Ok(_) => Job::job_passed(job, &conn).await,
                            Err(e) => Job::job_failed(e, job, &conn).await
                        };
                        drop(permit);
                    });
                }
                Err(e) => {
                    log::error!("Failed {:?} ", e);
                }
            };
        }
    }

    pub async fn dispatch(&self, channel: u8) {
        info!("Dispatching Jobs on channel: {}", channel);
        if channel == Email::CHANNEL {
            self.dispatch_emails().await
        }
    }
}