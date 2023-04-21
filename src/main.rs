use anyhow::{Context, Result};
use clap::Parser;
use dialoguer::{theme::ColorfulTheme, Select};
use dotenv::dotenv;
use gpt_cli::*;
use log::info;

fn main() -> Result<()> {
    dotenv().ok();
    pretty_env_logger::init();
    let args = Args::parse();

    let key = std::env::var("OPENAI_API_KEY").with_context(|| {
        "❗OpenAI key not found. You need to export OPEN_API_KEY on .bashrc or .zshrc"
    })?;

    let client = reqwest::blocking::Client::new();

    let body = CompletionBody {
        model: args.model.clone(),
        max_tokens: Some(args.tokens.unwrap_or(200)),
        prompt: "Linux command to ".to_owned() + args.prompt.as_str(),
        temperature: Some(0.0),
        stream: Some(false),
        top_p: None,
    };

    info!("args: {:#?}", args);

    let r = client
        .post("https://api.openai.com/v1/completions")
        .json(&body)
        .bearer_auth(key)
        .send()
        .unwrap();

    info!("status {}", r.status());

    let status = r.status();
    let body: CompletionResp = r
        .json()
        .with_context(|| format!("Request to ChatGPT server failed with status code {status}"))?;

    info!("body: {:#?}", body);

    let command = body.choices[0].text.clone().unwrap();
    let command = command.trim();

    println!("\nyour command is:");
    println!("{}", command);
    println!();

    let selections = &["Copy to clipboard", "Cancel"];
    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("What now?")
        .default(0)
        .items(&selections[..])
        .interact()?;

    if selection == 0 {
        cli_clipboard::set_contents(command.to_owned()).unwrap();
    }

    Ok(())
}
