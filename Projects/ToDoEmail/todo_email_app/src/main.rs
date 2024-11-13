use chrono::prelude::*;
use lettre::{Message, SmtpTransport, Transport};
use lettre::transport::smtp::authentication::Credentials;
use serde::{Serialize, Deserialize};
use std::fs::OpenOptions;
use std::io::{self, Write, Read};
use std::path::Path;

# [derive(Serialize, Deserialize, Debug)]
struct TodoItem {
    description : String,
    created_at: DateTime <Utc>
}

fn add_todo_item(todo_list: &mut Vec<TodoItem>, description: String) {
    let item = TodoItem {
        description,
        created_at: Utc::now(),
    };
    todo_list.push(item);
}

fn save_todo_list(todo_list: &Vec<TodoItem>, file_path: &str) -> io::Result<()> {
    let json = serde_json::to_string(todo_list)?;
    let mut file = OpenOptions::new().write(true).create(true).truncate(true).open(file_path)?;
    file.write_all(json.as_bytes())?;
    Ok(())
}

fn load_todo_list(file_path: &str) -> io::Result<Vec<TodoItem>> {
    if !Path::new(file_path).exists() {
        return Ok(Vec::new());
    }
    let mut file = OpenOptions::new().read(true).open(file_path)?;
    let mut json = String::new();
    file.read_to_string(&mut json)?;
    let todo_list: Vec<TodoItem> = serde_json::from_str(&json)?;
    Ok(todo_list)
}

fn send_email(subject: &str, body: &str, to_email: &str, from_email: &str, smtp_server: &str, smtp_port: u16, login: &str, password: &str) {
    let email = Message::builder()
        .from(from_email.parse().unwrap())
        .to(to_email.parse().unwrap())
        .subject(subject)
        .body(body.to_string())
        .unwrap();

    let creds = Credentials::new(login.to_string(), password.to_string());

    let mailer = SmtpTransport::starttls_relay(smtp_server)
        .unwrap()
        .port(smtp_port)
        .credentials(creds)
        .build();

    match mailer.send(&email) {
        Ok(_) => println!("Email sent successfully"),
        Err(e) => println!("Failed to send email: {:?}", e),
    }
}

fn main() -> io::Result<()> {
    let file_path = "todo_list.json";
    let mut todo_list = load_todo_list(file_path)?;

    loop {
        println!("1. Add To-Do Item");
        println!("2. Send To-Do List via Email");
        println!("3. Exit");

        let mut choice = String::new();
        io::stdin().read_line(&mut choice)?;
        let choice = choice.trim();

        match choice {
            "1" => {
                println!("Enter the description of the To-Do item:");
                let mut description = String::new();
                io::stdin().read_line(&mut description)?;
                let description = description.trim().to_string();
                add_todo_item(&mut todo_list, description);
                save_todo_list(&todo_list, file_path)?;
            }
            "2" => {
                let report_content = todo_list.iter()
                    .map(|item| format!("{} - Created at: {}", item.description, item.created_at))
                    .collect::<Vec<_>>()
                    .join("\n");
                
                let subject = format!("Daily To-Do List - {}", Utc::now().format("%Y-%m-%d"));
                let to_email = "recipient@example.com";
                let from_email = "your_email@example.com";
                let smtp_server = "smtp.example.com";
                let smtp_port = 587;
                let login = "your_email@example.com";
                let password = "your_email_password";

                send_email(&subject, &report_content, to_email, from_email, smtp_server, smtp_port, login, password);
            }
            "3" => break,
            _ => println!("Invalid choice, please try again."),
        }
    }

    Ok(())
}