use anyhow::{Context, Result};
use rig::{
    completion::ToolDefinition,
    tool::Tool,
};
use schemars::{JsonSchema, schema_for};
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};

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

        let entries = std::fs::read_dir(path)
            .with_context(|| format!("failed to read directory {}", args.path))
            .map_err(|err| ListFilesError::Other(err.to_string()))?;

        let files = entries
            .map(|entry| -> Result<Value, ListFilesError> {
                let entry = entry.map_err(ListFilesError::from)?;
                let entry_path = entry.path();
                let file_type = entry.file_type().map_err(ListFilesError::from)?;

                Ok(json!({
                    "name": entry.file_name().to_string_lossy().to_string(),
                    "path": entry_path.display().to_string(),
                    "is_dir": file_type.is_dir(),
                }))
            })
            .collect::<Result<Vec<_>, _>>()?;

        Ok(json!({ "files": files }))
    }
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct ReadFileArgs {
    pub path: String,
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
        if !path.is_file() {
            return Err(ReadFileError::NotFile(args.path));
        }
        let bytes =
            std::fs::read(path)
                .with_context(|| format!("failed to read file {}", args.path))
                .map_err(|err| ReadFileError::Other(err.to_string()))?;
        let content = String::from_utf8_lossy(&bytes).to_string();
        let total_chars = content.chars().count();
        Ok(json!({ 
            "path": args.path,
            "content": content,
            "total_chars": total_chars }))
    }
}
