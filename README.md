# Rust Agent

一个基于 Rust、Tokio 和 `rig-core` 的命令行 AI Agent 示例项目。当前默认使用 DeepSeek OpenAI-compatible API，并提供文件列表、文件读取、文件编辑和受限命令执行工具。

## 功能

- 通过 DeepSeek API 创建可多轮对话的 Agent。
- 支持 `.env` 配置 API Key、Base URL 和模型名。
- 内置文件工具：
  - `list_files`：递归列出目录内容，并跳过 `.git`、`target`、`node_modules`、`.env`、`Cargo.lock`。
  - `read_file`：读取文件内容，支持 `start_line` / `end_line` 指定行范围。
  - `edit_file`：替换整个文件，或替换文件中第一次出现的指定内容。
  - `run_command`：执行白名单内的项目命令并返回输出。

## 环境要求

- Rust stable，建议使用最新版。
- 一个可用的 DeepSeek API Key。

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

编辑 `.env`：

```dotenv
DEEPSEEK_API_KEY=sk-your-deepseek-api-key
BASE_URL=https://api.deepseek.com
MODEL=deepseek-v4-pro
```

配置项说明：

- `DEEPSEEK_API_KEY`：DeepSeek API Key，优先读取。
- `OPENAI_API_KEY`：当 `DEEPSEEK_API_KEY` 不存在时作为备用 Key。
- `BASE_URL`：OpenAI-compatible API 地址，默认 `https://api.deepseek.com`。
- `MODEL`：Agent 使用的模型，默认 `deepseek-v4-pro`。

`.env` 已被 `.gitignore` 忽略，请不要提交真实密钥。

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

