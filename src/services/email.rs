use std::time::Duration;
use apalis::prelude::Job;
use lettre::message::header::ContentType;
use lettre::{AsyncSmtpTransport, AsyncTransport, Message, Tokio1Executor, Transport};
use lettre::transport::smtp::authentication::Credentials;
use lettre::transport::smtp::Error;
use log::{error, info};
use tokio::time::Instant;
use crate::config::ENV;

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct Email {
    subject: String,
    from: String,
    message: String,
    to: Vec<String>,
}

impl Job for Email {
    const NAME: &'static str = "apalis::Email";
}

impl Email {
    fn new (to: String, subject: &str, message: String) -> Self {
        Self {
            subject: subject.to_string(),
            from: "noreply@framedpay.com".to_string(),
            message: message.to_string(),
            to: vec![to],
        }
    }

    pub fn html(to: String, subject: &str, template: String) -> Self {
        Self::new(to, subject, template)
    }

    pub fn from(&mut self, email: &str) -> &mut Self {
        self.from = email.to_string();
        self
    }

    pub fn to(&mut self, emails: Vec<String>) -> &mut Self {
        self.to = emails;
        self
    }

    pub async fn send(&mut self) -> Result<(), Error> {
        let mut email = Message::builder();
        for to in &self.to {
            email = email.to(to.parse().unwrap());
        }

        let email = email.from(self.from.parse().unwrap())
            .subject(self.subject.clone())
            .header(ContentType::TEXT_HTML)
            .body(self.message.clone())
            .unwrap();

        let creds = Credentials::new(ENV.mail_username.clone(), ENV.mail_password.clone());
        let mailer = AsyncSmtpTransport::<Tokio1Executor>::starttls_relay(ENV.mail_host.as_str())
            .unwrap()
            .port(ENV.mail_port)
            .credentials(creds)
            .timeout(Some(Duration::new(5, 0)))
            .build();

        info!("Started '{}':'{}'", ENV.mail_username.clone(), ENV.mail_password.clone());
        mailer.send(email).await?;

        Ok(())
    }

    pub async fn run(job: Email, _: apalis::prelude::JobContext) {
        match job.clone().send().await {
            Ok(_) => {},
            Err(e) => error!("Could not send email: {:?}", e),
        };
    }

}

