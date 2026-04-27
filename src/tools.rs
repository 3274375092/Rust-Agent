use std::str;

use anyhow::Result;
use rig::{completion::ToolDefinition, tool::Tool};
use schemars::{JsonSchema, schema_for};
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use tokio::fs::metadata;
use tokio::io::AsyncBufReadExt;
use tokio::{fs::File, io::BufReader};
use walkdir::WalkDir;

const MAX_READ_BYTES: usize = 128 * 1024;

#[derive(Debug, Deserialize, JsonSchema)]
pub struct ListFilesArgs {
    pub path: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ListFilesTool;

#[derive(Debug, thiserror::Error)]
pub enum ListFilesError {
    #[error("{0} is not a directory")]
    NotDirectory(String),
    #[error("io error: {0}")]
    Io(#[from] std::io::Error),
    #[error("{0}")]
    Other(String),
}

impl Tool for ListFilesTool {
    const NAME: &'static str = "list_files";
    type Error = ListFilesError;
    type Args = ListFilesArgs;
    type Output = Value;

    async fn definition(&self, _prompt: String) -> ToolDefinition {
        ToolDefinition {
            name: Self::NAME.to_string(),
            description: "List files and folders in a directory".to_string(),
            parameters: serde_json::to_value(schema_for!(ListFilesArgs))
                .expect("ListFilesArgs schema should serialize"),
        }
    }

    async fn call(&self, args: Self::Args) -> Result<Self::Output, Self::Error> {
        let path = std::path::Path::new(&args.path);
        if !path.is_dir() {
            return Err(ListFilesError::NotDirectory(args.path));
        }
        let mut files = Vec::new();
        for entry in WalkDir::new(path)
            .min_depth(1)
            .max_depth(5)
            .sort_by_file_name()
            .into_iter()
            .filter_entry(|e| {
                let file_name = e.file_name().to_string_lossy();
                !matches!(
                    file_name.as_ref(),
                    ".git" | "target" | "node_modules" | ".env" | "Cargo.lock"
                )
            })
            .filter_map(|e| e.ok())
            .filter(|e| e.file_type().is_dir() || e.file_type().is_file())
        {
            let entry_path = entry
                .path()
                .strip_prefix(path)
                .map_err(|e| ListFilesError::Other(e.to_string()))?;
            let mut entry_path = entry_path.to_string_lossy().replace('\\', "/");
            if entry.file_type().is_dir() {
                entry_path.push('/');
            }
            files.push(entry_path);
        }

        Ok(json!({
            "root": path.display().to_string().replace('\\', "/"),
            "files": files
        }))
    }
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct ReadFileArgs {
    pub path: String,
    pub start_line: Option<usize>,
    pub end_line: Option<usize>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ReadFileTool;

#[derive(Debug, thiserror::Error)]
pub enum ReadFileError {
    #[error("{0} is not a file")]
    NotFile(String),
    #[error("io error: {0}")]
    Io(#[from] std::io::Error),
    #[error("{0}")]
    Other(String),
}

impl Tool for ReadFileTool {
    const NAME: &'static str = "read_file";
    type Args = ReadFileArgs;
    type Output = Value;
    type Error = ReadFileError;

    async fn definition(&self, _prompt: String) -> ToolDefinition {
        ToolDefinition {
            name: Self::NAME.to_string(),
            description: "Read the content of a file".to_string(),
            parameters: serde_json::to_value(schema_for!(ReadFileArgs))
                .expect("ReadFileArgs schema should serialize"),
        }
    }
    async fn call(&self, args: Self::Args) -> Result<Self::Output, Self::Error> {
        let path = std::path::Path::new(&args.path);
        let start_line = args.start_line.unwrap_or(0);
        let end_line = args.end_line.unwrap_or(usize::MAX);
        let metadata = metadata(path).await?;
        let file_size = metadata.len();
        if !path.is_file() {
            return Err(ReadFileError::NotFile(args.path));
        }
        if file_size > MAX_READ_BYTES as u64 {
            return Err(ReadFileError::Other(
                "file size exceeds max_bytes".to_string(),
            ));
        }
        if end_line < start_line {
            return Err(ReadFileError::Other(
                "end_line must be greater than or equal to start_line".to_string(),
            ));
        }
        let file = File::open(path).await?;
        let reader = BufReader::new(file);
        let mut lines = reader.lines();
        let mut content = String::new();
        let mut line_count = 0usize;
        while let Some(line) = lines.next_line().await? {
            line_count += 1;

            if line_count < start_line {
                continue;
            }

            if line_count > end_line {
                break;
            }

            content.push_str(&line);
            content.push('\n');
        }

        let total_chars = content.chars().count();
        Ok(json!({
            "path": path.display().to_string().replace('\\', "/"),
            "content": content,
            "total_chars": total_chars }))
    }
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct EditFileArgs {
    pub path: String,
    pub new_content: String,
    pub old_content: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EditFileTool;

impl Tool for EditFileTool {
    const NAME: &'static str = "edit_file";
    type Args = EditFileArgs;
    type Output = Value;
    type Error = std::io::Error;

    async fn definition(&self, _prompt: String) -> ToolDefinition {
        ToolDefinition {
            name: Self::NAME.to_string(),
            description: "Edit the content of a file. If old_content is provided, only the first occurrence of that text will be replaced with new_content. Otherwise, the entire file will be replaced with new_content.".to_string(),
            parameters: serde_json::to_value(schema_for!(EditFileArgs))
                .expect("EditFileArgs schema should serialize"),
        }
    }

    async fn call(&self, args: Self::Args) -> Result<Self::Output, Self::Error> {
        let path = std::path::Path::new(&args.path);
        if !path.is_file() {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                format!("{} is not a file", args.path),
            ));
        }
        let content = std::fs::read_to_string(path)?;
        if let Some(old_content) = &args.old_content {
            if !content.contains(old_content) {
                return Err(std::io::Error::new(
                    std::io::ErrorKind::InvalidInput,
                    format!(
                        "The specified old_content was not found in the file '{}'.\n\
                         Please ensure the text matches exactly (including whitespace and line breaks).",
                        args.path
                    ),
                ));
            }

            let new_content = content.replacen(old_content, &args.new_content, 1);
            std::fs::write(path, new_content)?;
        } else {
            std::fs::write(path, args.new_content)?;
        }

        Ok(json!({
            "path": args.path,
            "operation": if args.old_content.is_some() { "replace_first_occurrence" } else { "replace_entire_file" }
        }))
    }
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct RunCommandToolArgs {
    pub command: String,
    pub args: Vec<String>,
}
#[derive(Debug, thiserror::Error)]
pub enum RunCommandError {
    #[error("command not allowed")]
    NotAllowed,
    #[error("io error: {0}")]
    Io(#[from] std::io::Error),
    #[error("command timed out")]
    Timeout,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RunCommandTool;

impl Tool for RunCommandTool {
    const NAME: &'static str = "run_command";
    type Args = RunCommandToolArgs;
    type Output = Value;
    type Error = RunCommandError;
    async fn definition(&self, _prompt: String) -> ToolDefinition {
        ToolDefinition {
            name: Self::NAME.to_string(),
            description: "Run a shell command with optional arguments".to_string(),
            parameters: serde_json::to_value(schema_for!(RunCommandToolArgs))
                .expect("错误的命令参数"),
        }
    }
    async fn call(&self, args: Self::Args) -> Result<Self::Output, Self::Error> {
        if !is_allowed_command(&args.command, &args.args) {
            return Err(RunCommandError::NotAllowed);
        }
        println!("$ {} {}", args.command, args.args.join(" "));
        let output = tokio::time::timeout(
            std::time::Duration::from_secs(60),
            tokio::process::Command::new(&args.command)
                .args(&args.args)
                .current_dir(std::env::current_dir()?)
                .output(),
        )
        .await
        .map_err(|_| RunCommandError::Timeout)??;
        let stdout = String::from_utf8_lossy(&output.stdout).to_string();
        let stderr = String::from_utf8_lossy(&output.stderr).to_string();
        Ok(serde_json::json!({ "exit_code": output.status.code(),
            "stdout": stdout,
            "stderr": stderr,
            "success": output.status.success() }))
    }
}
fn is_allowed_command(command: &str, args: &[String]) -> bool {
    match command {
        "cargo" => matches!(
            args.first().map(String::as_str),
            Some("check" | "test" | "clippy" | "fmt" | "build")
        ),
        "git" => matches!(
            args.first().map(String::as_str),
            Some("status" | "diff" | "log" | "show")
        ),
        "rustc" => args.first().map(String::as_str) == Some("--version"),
        _ => false,
    }
}
