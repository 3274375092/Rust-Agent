mod tools;

use std::io::Write;

use anyhow::{Context, Result};
use rig::{
    agent::{self, PromptHook, stream_to_stdout},
    client::CompletionClient,
    completion::{CompletionModel, GetTokenUsage},
    message::Message,
    providers::{deepseek, openai},
    streaming::StreamingChat,
};

#[tokio::main]
async fn main() -> Result<()> {
    dotenvy::dotenv().ok();

    let api_key = std::env::var("API_KEY")
        .or_else(|_| std::env::var("DEEPSEEK_API_KEY"))
        .or_else(|_| std::env::var("OPENAI_API_KEY"))
        .context("set API_KEY, DEEPSEEK_API_KEY, or OPENAI_API_KEY")?;
    let base_url = std::env::var("BASE_URL")
        .or_else(|_| std::env::var("OPENAI_BASE_URL"))
        .or_else(|_| std::env::var("DEEPSEEK_BASE_URL"))
        .unwrap_or_else(|_| "https://api.deepseek.com".into());
    let model = std::env::var("MODEL").unwrap_or_else(|_| "deepseek-v4-pro".into());

    if is_deepseek_api(&base_url) {
        let client = deepseek::Client::builder()
            .api_key(&api_key)
            .base_url(&base_url)
            .build()?;
        let agent = build_agent(client.agent(&model));
        run_agent(agent).await?;
    } else {
        let client = openai::Client::builder()
            .api_key(&api_key)
            .base_url(&base_url)
            .build()?
            .completions_api();
        let agent = build_agent(client.agent(&model));
        run_agent(agent).await?;
    }

    Ok(())
}
//自循环
fn build_agent<M, P>(builder: agent::AgentBuilder<M, P, agent::NoToolConfig>) -> agent::Agent<M, P>
where
    M: CompletionModel,
    P: PromptHook<M>,
{
    builder
        .preamble("你是一个拥有中文语言思维逻辑的人，接下来请用中文语言思维逻辑回答.")
        .tool(tools::ListFilesTool)
        .tool(tools::ReadFileTool)
        .tool(tools::EditFileTool)
        .tool(tools::RunCommandTool)
        .build()
}

fn is_deepseek_api(base_url: &str) -> bool {
    base_url.to_ascii_lowercase().contains("deepseek")
}

async fn run_agent<M, P>(agent: agent::Agent<M, P>) -> Result<()>
where
    M: CompletionModel + 'static,
    M::StreamingResponse: Clone + Unpin + GetTokenUsage + rig::wasm_compat::WasmCompatSend,
    P: PromptHook<M> + 'static,
{
    let history = Vec::new();
    let args = std::env::args().skip(1).collect::<Vec<_>>();
    if args.first().map(String::as_str) == Some("--auto") {
        let goal = args
            .get(1)
            .cloned()
            .unwrap_or_else(|| "Inspect the project and improve it safely.".into());
        auto_loop(agent, goal, history).await
    } else {
        interactive_loop(agent, history).await
    }
}

async fn auto_loop<M, P>(
    agent: agent::Agent<M, P>,
    goal: String,
    mut history: Vec<Message>,
) -> Result<()>
where
    M: CompletionModel + 'static,
    M::StreamingResponse: Clone + Unpin + GetTokenUsage + rig::wasm_compat::WasmCompatSend,
    P: PromptHook<M> + 'static,
{
    let mut no_progress_count = 0;
    for cycle in 1..=50 {
        let prompt = format!(
            r#"
You are running in autonomous mode.

Goal:
{goal}

Cycle:
{cycle}

Work in this exact loop:
1. Observe current state.
2. Decide the next concrete action.
3. Use tools if needed.
4. Verify the result.
5. State whether progress was made.
6. End with one of:
   CONTINUE
   DONE
   BLOCKED
   NO_PROGRESS

Rules:
- Continue only when there is a concrete next action.
- Do not repeat the same failed action.
- Before editing files, state the intended change.
- After editing, verify with an allowed command when possible.
"#
        );
        let mut stream = agent
            .stream_chat(&prompt, history.clone())
            .multi_turn(20)
            .await;
        let final_res = stream_to_stdout(&mut stream).await?;
        if let Some(new_history) = final_res.history() {
            history = new_history.to_vec();
        }
        println!();
        let context = final_res.response();
        if context.contains("DONE") {
            println!("Goal achieved!");
            break;
        } else if context.contains("BLOCKED") {
            println!("Agent is blocked. Stopping.");
            break;
        } else if context.contains("NO_PROGRESS") {
            no_progress_count += 1;
            if no_progress_count >= 3 {
                println!("No progress for 3 cycles. Stopping.");
                break;
            }
        } else {
            no_progress_count = 0;
        }
    }

    Ok(())
}
async fn interactive_loop<M, P>(agent: agent::Agent<M, P>, mut history: Vec<Message>) -> Result<()>
where
    M: CompletionModel + 'static,
    M::StreamingResponse: Clone + Unpin + GetTokenUsage + rig::wasm_compat::WasmCompatSend,
    P: PromptHook<M> + 'static,
{
    loop {
        print!("You> ");
        std::io::stdout()
            .flush()
            .context("failed to flush stdout")?;

        let mut input = String::new();
        let bytes_read = std::io::stdin()
            .read_line(&mut input)
            .context("failed to read user input")?;
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
        println!();
    }
    Ok(())
}
