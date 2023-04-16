use clap::Parser;
use cli_clipboard;
use dialoguer::{theme::ColorfulTheme, Select};
use dotenv::dotenv;
use gpt_cli::*;
use log::info;
use reqwest::header::{HeaderMap, CONTENT_TYPE};

fn main() -> Result<(), &'static str> {
    dotenv().ok();
    pretty_env_logger::init();
    let args = Args::parse();

    let key = std::env::var("OPENAI_API_KEY").map_err(|_| {
        "❗OpenAI key not found. You need to export OPEN_API_KEY on .bashrc or .zshrc"
    })?;

    let client = reqwest::blocking::Client::new();
    let mut headers = HeaderMap::new();

    let body = serde_json::to_string(&CompletionBody {
        model: args.model.clone(),
        max_tokens: Some(args.tokens.unwrap_or(200)),
        prompt: "Linux command to ".to_owned() + args.prompt.clone().as_str(),
        top_p: Some(1.0),
        stream: Some(false),
        temperature: None,
    })
    .unwrap();

    headers.insert(CONTENT_TYPE, "application/json".parse().unwrap());

    info!("args: {:#?}", args);

    let r = client
        .post("https://api.openai.com/v1/completions")
        .headers(headers)
        .body(body)
        .bearer_auth(key)
        .send()
        .unwrap();

    info!("status {}", r.status());

    let body: CompletionResp = serde_json::from_str(r.text().unwrap().as_str()).unwrap();

    info!("body: {:#?}", body);

    let command = body.choices[0].text.clone().unwrap();
    let command = command.trim();

    println!("\nyour command is:");
    println!("{}", command);
    print!("\n");

    let selections = &["Copy to clipboard", "Cancel"];
    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("What now?")
        .default(0)
        .items(&selections[..])
        .interact()
        .unwrap();

    match selection {
        0 => {
            cli_clipboard::set_contents(command.to_owned()).unwrap();
        }
        _ => {}
    };

    Ok(())
}
