use std::collections::HashMap;
use apalis::prelude::Job;
use log::error;
use sendgrid::{SendgridError, SendgridResult};
use reqwest::blocking::Response;
use sendgrid::error::RequestNotSuccessful;
use sendgrid::v3::{Content, Message, Personalization, Email as SGEmail, Sender};
use crate::config::ENV;

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct Email {
    subject: String,
    from: String,
    message: String,
    to: Vec<String>,
    data: HashMap<String, String>
}

impl Job for Email {
    const NAME: &'static str = "apalis::Email";
}

impl Email {
    fn new (to: String, subject: &str, message: &str, data: HashMap<String, String>) -> Self {
        Self {
            subject: subject.to_string(),
            from: format!("noreply@{}", ENV.app_domain.clone()),
            message: message.to_string(),
            to: vec![to],
            data
        }
    }

    pub fn simple(to: String, subject: &str, message: &str, data: HashMap<String, String>) -> Self {
        Self::new(to, subject, message, data)
    }

    pub fn html(to: String, subject: &str, template: &str, data: HashMap<String, String>) -> Self {
        Self::new(to, subject, template, data)
    }

    pub fn from(&mut self, email: &str) -> &mut Self {
        self.from = email.to_string();
        self
    }

    pub fn to(&mut self, emails: Vec<String>) -> &mut Self {
        self.to = emails;
        self
    }

    pub fn send(&mut self) -> SendgridResult<Response> {
        let mut emails: Vec<SGEmail> = self.to.iter().map(|email| SGEmail::new(email)).collect();
        let mut personalize = Personalization::new(emails.remove(0));
        for email in &emails {
            personalize = personalize.add_to(email.clone());
        }

        let message = Message::new(SGEmail::new(self.from.clone()))
            .set_subject(self.subject.as_mut_str())
            .add_content(Content::new().set_content_type("text/html").set_value("Test"))
            .add_personalization(personalize);

        let resp = Sender::new(ENV.sendgrid_api_key.to_string()).send(&message)?;
        if resp.error_for_status_ref().is_err() {
            return Err(RequestNotSuccessful::new(resp.status(), resp.text()?).into());
        }

        Ok(resp)
    }

    pub async fn run(job: Email, _: apalis::prelude::JobContext) -> SendgridResult<Response> {
        job.clone().send()
    }

}

