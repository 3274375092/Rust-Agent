# Rust Agent

一个基于 Rust、Tokio 和 `rig-core` 的命令行 AI Agent 示例项目。项目使用 OpenAI-compatible Chat Completions 接口，因此可以接入 DeepSeek、OpenAI、OpenRouter、Ollama、LM Studio 等兼容 API 的模型服务。

## 功能

- 支持通过 `.env` 配置 API Key、Base URL 和模型名。
- 使用 OpenAI-compatible Chat Completions 兼容层调用模型。
- 支持多轮对话和流式输出。
- 内置工具：
  - `list_files`：递归列出目录内容，并跳过 `.git`、`target`、`node_modules`、`.env`、`Cargo.lock`。
  - `read_file`：读取文件内容，支持 `start_line` / `end_line` 指定行范围。
  - `edit_file`：替换整个文件，或替换文件中第一次出现的指定内容。
  - `run_command`：执行白名单内的项目命令并返回输出。

## 环境要求

- Rust stable，建议使用最新版。
- 一个支持 OpenAI-compatible Chat Completions 的 API 服务。
- 如果要使用文件/命令工具，模型需要支持 tool calling/function calling。

检查 Rust 版本：

```powershell
rustc --version
cargo --version
```

## 配置

复制示例环境文件：

```powershell
Copy-Item .env.example .env
```

推荐使用通用配置项：

```dotenv
API_KEY=sk-your-api-key
BASE_URL=https://api.deepseek.com
MODEL=deepseek-v4-pro
```

配置项说明：

- `API_KEY`：OpenAI-compatible API Key，优先读取。
- `BASE_URL`：OpenAI-compatible API 地址。
- `MODEL`：Agent 使用的聊天模型。
- `DEEPSEEK_API_KEY`：兼容旧配置；当 `API_KEY` 不存在时读取。
- `OPENAI_API_KEY`：兼容旧配置；当 `API_KEY` 和 `DEEPSEEK_API_KEY` 都不存在时读取。
- `DEEPSEEK_BASE_URL` / `OPENAI_BASE_URL`：兼容旧配置；当 `BASE_URL` 不存在时读取。

`.env` 已被 `.gitignore` 忽略，请不要提交真实密钥。

## `.env` 加载位置

`.env` 不会被编译进可执行文件，它是在程序启动时由 `dotenvy` 读取的。

当前代码使用 `dotenvy::dotenv().ok()`，因此默认从“启动程序时的当前工作目录”查找 `.env`。

开发时通常放在项目根目录：

```powershell
cd E:\Rust\Rust-Agent
cargo run
```

这会读取：

```text
E:\Rust\Rust-Agent\.env
```

发布 exe 时，可以把 `.env` 和程序放在同一目录：

```text
my-agent\
├── rig-agent.exe
└── .env
```

然后从该目录启动：

```powershell
cd my-agent
.\rig-agent.exe
```

这会读取：

```text
my-agent\.env
```

如果从其他目录启动 exe，当前代码会优先查找启动目录下的 `.env`，不是 exe 所在目录。

## 常见模型服务配置

DeepSeek：

```dotenv
API_KEY=sk-your-deepseek-key
BASE_URL=https://api.deepseek.com
MODEL=deepseek-v4-pro
```

OpenAI：

```dotenv
API_KEY=sk-your-openai-key
BASE_URL=https://api.openai.com/v1
MODEL=gpt-4.1
```

OpenRouter：

```dotenv
API_KEY=sk-or-your-openrouter-key
BASE_URL=https://openrouter.ai/api/v1
MODEL=deepseek/deepseek-chat-v3.1
```

Ollama 本地服务：

```dotenv
API_KEY=ollama
BASE_URL=http://localhost:11434/v1
MODEL=qwen2.5-coder:7b
```

LM Studio 本地服务：

```dotenv
API_KEY=lm-studio
BASE_URL=http://localhost:1234/v1
MODEL=local-model
```

## 运行

```powershell
cargo run
```

进入交互后输入问题即可。输入下面任一命令退出：

```text
exit
quit
```

## 常用开发命令

```powershell
cargo fmt --check
cargo check
cargo clippy --all-targets --all-features
cargo test
```

## 命令执行工具白名单

`run_command` 不开放任意 shell 命令，只允许部分项目审查命令：

- `cargo check`
- `cargo test`
- `cargo clippy`
- `cargo fmt`
- `cargo build`
- `git status`
- `git diff`
- `git log`
- `git show`
- `rustc --version`

命令执行会设置 60 秒超时，并把 `stdout`、`stderr`、`exit_code` 和 `success` 返回给模型。

## 安全提醒

- 不要把 `.env` 或真实 API Key 提交到仓库。
- `edit_file` 能修改本地文件，建议先通过 `git diff` 确认改动。
- `run_command` 当前使用白名单限制，扩展命令时应避免允许删除、移动、网络下载或长期运行的命令。
- 不是所有 OpenAI-compatible 模型都支持工具调用；如果工具调用失败，请换成支持 function calling 的模型。

## 项目结构

```text
.
├── Cargo.toml
├── .env.example
├── README.md
└── src
    ├── main.rs
    └── tools.rs
```
