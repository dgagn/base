use lettre::{message::header::ContentType, FileTransport, Message, Transport};
use std::path::PathBuf;

pub struct EmailClient {
    driver: Box<dyn EmailDriver>,
}

pub enum EmailClientDriver {
    FileSystem { path: PathBuf },
}

pub trait EmailDriver {
    fn send_message(&self, message: &Message) -> anyhow::Result<()>;
    fn sender_address(&self) -> &str;
}

#[derive(Debug)]
pub struct Email<'a> {
    pub to: &'a str,
    pub subject: &'a str,
    pub body: &'a str,
}

impl EmailClient {
    pub fn send(&self, email: &Email) -> anyhow::Result<()> {
        let message_id = uuid::Uuid::new_v4().to_string();
        let message = Message::builder()
            .message_id(Some(message_id))
            .to(email.to.parse()?)
            .from(self.driver.sender_address().parse()?)
            .subject(email.subject)
            .header(ContentType::TEXT_PLAIN)
            .body(email.body.to_string())?;

        self.driver.send_message(&message)
    }
}

struct FileEmailDriver {
    path: PathBuf,
}

impl EmailDriver for FileEmailDriver {
    fn send_message(&self, message: &Message) -> anyhow::Result<()> {
        let id = FileTransport::new(&self.path).send(message)?;
        tracing::info!("Email sent to file transport with id {}", id);
        Ok(())
    }

    fn sender_address(&self) -> &str {
        "test@localhost"
    }
}

#[cfg(test)]
mod tests {
    use lettre::transport::stub::StubTransport;

    use super::*;
    struct MockEmailDriver {
        transport: StubTransport,
    }

    impl EmailDriver for MockEmailDriver {
        fn send_message(&self, message: &Message) -> anyhow::Result<()> {
            self.transport.send(message)?;
            tracing::info!("Email sent to mock transport");
            Ok(())
        }

        fn sender_address(&self) -> &str {
            "test@localhost"
        }
    }

    #[tokio::test]
    async fn test_send_email() {
        let driver = MockEmailDriver {
            transport: StubTransport::new_ok(),
        };
        let client = EmailClient {
            driver: Box::new(driver),
        };
        let email = Email {
            to: "test@localhost",
            subject: "Test email",
            body: "This is a test email",
        };

        let result = client.send(&email);

        assert!(result.is_ok());
    }
}
