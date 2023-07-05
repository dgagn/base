use lettre::{message::header::ContentType, FileTransport, Message, Transport};
use std::{env::current_dir, path::PathBuf};

pub struct EmailClient {
    driver: EmailClientDriver,
}

pub enum EmailClientDriver {
    FileSystem { path: PathBuf },
}

#[derive(Debug)]
pub struct Email<'a> {
    pub to: &'a str,
    pub subject: &'a str,
    pub body: &'a str,
}

impl EmailClient {
    pub fn new() -> Self {
        let dir = current_dir().expect("Failed to get current directory");
        let default_path = dir.join("storage/emails");

        Self {
            driver: EmailClientDriver::FileSystem { path: default_path },
        }
    }

    pub fn send(&self, email: &Email) -> anyhow::Result<()> {
        let message_id = uuid::Uuid::new_v4().to_string();
        let message = Message::builder()
            .message_id(Some(message_id))
            .to(email.to.parse()?)
            .from(self.sender_address().parse()?)
            .subject(email.subject)
            .header(ContentType::TEXT_PLAIN)
            .body(email.body.to_string())?;

        match &self.driver {
            EmailClientDriver::FileSystem { path } => {
                FileTransport::new(path).send(&message).unwrap();
                Ok(())
            }
        }
    }

    fn sender_address(&self) -> &str {
        match &self.driver {
            EmailClientDriver::FileSystem { .. } => "test@localhost",
        }
    }
}
