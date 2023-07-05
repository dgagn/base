mod email;
use email::{Email, EmailClient};

fn main() {
    let email_client = EmailClient::new();
    let email = Email {
        to: "ddanygagnon@gmail.com",
        subject: "Hello from Rust",
        body: "Hello from Rust",
    };

    email_client.send(&email).unwrap()
}
