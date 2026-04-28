# Rust Agent

一个基于 Rust 和 `rig-core` 的命令行 AI agent。它兼容 OpenAI Chat Completions 风格的接口，可以连接 DeepSeek、OpenAI、OpenRouter、Ollama、LM Studio 等服务，并在项目目录中读取文件、编辑文件、执行受限检查命令。

程序支持两种运行方式：

- 交互模式：手动输入消息，agent 在当前进程内保留对话上下文。
- 自循环模式：使用 `--auto` 指定目标，agent 会按观察、计划、执行、验证的节奏持续工作，直到完成、阻塞、连续无进展或达到循环上限。

## 功能

- OpenAI-compatible API 配置。
- DeepSeek 官方 API 自动使用 `rig-core` 的 native DeepSeek provider。
- 会话内上下文传递。
- 文件列表、文件读取、文件编辑工具。
- 受限命令执行工具。
- 自循环模式最多运行 50 轮，每轮最多 20 次 multi-turn 工具调用。
- 连续 3 轮 `NO_PROGRESS` 后自动停止。

## 环境要求

- Rust toolchain
- 一个可用的 OpenAI-compatible API 服务
- 支持 tool calling 或 function calling 的模型
- PowerShell 或其他终端

## 快速开始

复制环境变量模板：

```powershell
Copy-Item .env.example .env
```

编辑 `.env`：

```dotenv
API_KEY=sk-your-api-key
BASE_URL=https://api.deepseek.com
MODEL=deepseek-v4-pro
```

运行交互模式：

```powershell
cargo run
```

看到提示符后输入消息：

```text
You>
```

退出程序：

```text
exit
```

或：

```text
quit
```

## 自循环模式

使用 `--auto` 启动自循环模式：

```powershell
cargo run -- --auto "检查项目，发现问题，修复并验证"
```

如果目标里有空格，请使用引号。程序读取 `--auto` 后面的第一个参数作为目标；不加引号时，目标只会取第一个词。

如果不提供目标，会使用默认目标：

```text
Inspect the project and improve it safely.
```

自循环模式每一轮会要求 agent 按以下步骤工作：

1. Observe current state.
2. Decide the next concrete action.
3. Use tools if needed.
4. Verify the result.
5. State whether progress was made.
6. End with one of: `CONTINUE`, `DONE`, `BLOCKED`, `NO_PROGRESS`.

停止条件：

- 输出包含 `DONE`。
- 输出包含 `BLOCKED`。
- 连续 3 轮输出包含 `NO_PROGRESS`。
- 达到 50 轮循环上限。

## 配置项

程序会读取当前工作目录下的 `.env`：

```dotenv
API_KEY=sk-your-api-key
BASE_URL=https://api.deepseek.com
MODEL=deepseek-v4-pro
```

变量说明：

- `API_KEY`：API key。优先使用这个通用变量。
- `BASE_URL`：OpenAI-compatible base URL。默认值为 `https://api.deepseek.com`。
- `MODEL`：模型名称。默认值为 `deepseek-v4-pro`。

兼容变量：

- 如果没有 `API_KEY`，会依次读取 `DEEPSEEK_API_KEY`、`OPENAI_API_KEY`。
- 如果没有 `BASE_URL`，会依次读取 `OPENAI_BASE_URL`、`DEEPSEEK_BASE_URL`。

## Provider 示例

DeepSeek：

```dotenv
API_KEY=sk-your-deepseek-key
BASE_URL=https://api.deepseek.com
MODEL=deepseek-v4-pro
```

当 `BASE_URL` 包含 `deepseek` 时，程序会使用 native DeepSeek provider。这个路径可以在 tool calling 历史中保留并回传 `reasoning_content`，便于使用 DeepSeek thinking 模式。

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

Ollama：

```dotenv
API_KEY=ollama
BASE_URL=http://localhost:11434/v1
MODEL=qwen2.5-coder:7b
```

LM Studio：

```dotenv
API_KEY=lm-studio
BASE_URL=http://localhost:1234/v1
MODEL=local-model
```

## 工具

### `list_files`

列出目录下的文件和文件夹，最多递归 5 层，默认跳过：

- `.git`
- `target`
- `node_modules`
- `.env`
- `Cargo.lock`

### `read_file`

读取文件内容，支持 `start_line` 和 `end_line`。单个文件最大读取 128 KiB。

### `edit_file`

编辑文件内容：

- 如果提供 `old_content`，只替换第一次匹配。
- 如果不提供 `old_content`，会替换整个文件。

### `run_command`

执行受限命令，超时时间为 60 秒。

允许的命令：

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

## 构建

开发检查：

```powershell
cargo check
```

发布构建：

```powershell
cargo build --release
```

生成的可执行文件：

```text
target\release\rig-agent.exe
```

如果直接运行 exe，请把 `.env` 放在运行命令所在的工作目录，或先切换到项目目录再运行。

## 注意事项

- `.env` 不要提交到 Git。
- 自循环模式会持续消耗 API token，请给目标写清楚边界。
- 当前上下文只保存在进程内，程序退出后不会持久化。
- 长时间运行时，history 会增长，token 成本和响应时间也会增加。

## 编码

`README.md` 使用 UTF-8 编码保存。Windows PowerShell 5.1 中查看中文时，建议使用：

```powershell
Get-Content README.md -Encoding UTF8
```
