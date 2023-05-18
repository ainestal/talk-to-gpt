mod chat_context;
mod gpt;

use anyhow::{Context, Result};

use crate::gpt::GPT;

#[tokio::main]
async fn main() -> Result<()> {
    let token =
        std::env::var("OPENAI_API_KEY").context("OPENAI_API_KEY environment variable not set")?;

    let mut gpt = GPT::new(token)?;

    println!("Initialised GPT-4 chatbot. Enter your message to start a conversation.");
    println!("You can quit by pressing Ctrl+C (linux), or Cmd+C (Mac).");
    println!("--------------------------------------");
    loop {
        println!("- Enter your message and press Enter:");
        let mut input = String::new();
        std::io::stdin()
            .read_line(&mut input)
            .context("Failed to read your input")?;

        println!("- AI:");
        let answer = gpt
            .completion(input)
            .await
            .context("Could not get an answer from GPT")?;

        println!("{}", format!("{}", answer.content));
        println!("--------------------------------------");
    }
}
