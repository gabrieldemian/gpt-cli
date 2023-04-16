use dotenv::dotenv;
use gpt_cli::*;
use log::info;
use reqwest::header::{HeaderMap, CONTENT_TYPE};

fn main() -> Result<(), &'static str> {
    pretty_env_logger::init();
    dotenv().ok();

    let key = std::env::vars().find(|(k, _)| k == &"OPENAI_API_KEY".to_string());
    if key.is_none() {
        return Err("You need to paste a key OPENAI_API_KEY on a .env file");
    }
    let (_, key) = key.unwrap();

    let prompt = "write a short poem about how God conquered death".to_string();

    let client = reqwest::blocking::Client::new();
    let mut headers = HeaderMap::new();

    let body = serde_json::to_string(&CompletionBody {
        // model: "gpt-3.5-turbo".to_string(),
        model: "text-davinci-003".to_string(),
        max_tokens: Some(5),
        prompt,
        top_p: Some(1.0),
        stream: Some(false),
        temperature: None,
    })
    .unwrap();

    headers.insert(CONTENT_TYPE, "application/json".parse().unwrap());

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

    println!("{}", body.choices[0].text.clone().unwrap());

    Ok(())
}
