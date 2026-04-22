mod tools;
use std::io::Write;

use anyhow::{Context, Result};
use rig::{
    agent::stream_to_stdout, client::CompletionClient, providers::deepseek,
    streaming::StreamingChat,
};

#[tokio::main]
async fn main() -> Result<()> {
    dotenvy::dotenv().ok();

    let api_key = std::env::var("DEEPSEEK_API_KEY")
        .or_else(|_| std::env::var("OPENAI_API_KEY"))
        .context("set DEEPSEEK_API_KEY or OPENAI_API_KEY")?;
    let base_url =
        std::env::var("DEEPSEEK_BASE_URL").unwrap_or_else(|_| "https://api.deepseek.com".into());

    let client = deepseek::Client::builder()
        .api_key(&api_key)
        .base_url(&base_url)
        .build()?;

    let agent = client
        .agent(deepseek::DEEPSEEK_CHAT)
        .preamble("You are a helpful assistant.")
        .tool(tools::ListFilesTool)
        .tool(tools::ReadFileTool)
        .tool(tools::EditFileTool)
        .build();
    let mut history = Vec::new();
    loop {
        print!("你>");
        std::io::stdout()
            .flush()
            .context("failed to flush stdout")?;
        let mut input = String::new();
        let bytes_read = std::io::stdin()
            .read_line(&mut input)
            .context("错误的用户输入")?;
        if bytes_read == 0 {
            break;
        }
        let input = input.trim();
        if input.is_empty() {
            continue;
        }
        if input.eq_ignore_ascii_case("exit") || input.eq_ignore_ascii_case("quit") {
            break;
        }
        let mut stream = agent
            .stream_chat(input, history.clone())
            .multi_turn(20)
            .await;
        let final_res = stream_to_stdout(&mut stream).await?;
        if let Some(new_history) = final_res.history() {
            history = new_history.to_vec();
        }
        println!(); // 换行
    }

    Ok(())
}
