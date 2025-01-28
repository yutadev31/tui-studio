use std::process::Stdio;

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use tokio::{
    io::{AsyncBufReadExt, AsyncWriteExt, BufReader, BufWriter},
    process::Command,
};

// LSPメッセージの構造
#[derive(Serialize, Deserialize, Debug)]
struct LspRequest {
    jsonrpc: String,
    id: u64,
    method: String,
    params: Value,
}

#[derive(Serialize, Deserialize, Debug)]
struct LspResponse {
    jsonrpc: String,
    id: u64,
    result: Option<Value>,
    error: Option<LspError>,
}

#[derive(Serialize, Deserialize, Debug)]
struct LspError {
    code: i32,
    message: String,
}

pub struct LSPClient {
    server_cmd: String,
}

impl LSPClient {
    pub fn new(server_cmd: String) -> Self {
        Self { server_cmd }
    }

    pub async fn run(&self) -> Result<()> {
        let mut server = Command::new(self.server_cmd.clone())
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn()?;

        let stdin = server.stdin.take().expect("Failed to open stdin");
        let stdout = server.stdout.take().expect("Failed to open stdout");

        let mut writer = BufWriter::new(stdin);
        let mut reader = BufReader::new(stdout).lines();

        // サーバーからのレスポンスを受信するタスク
        let response_task = tokio::spawn(async move {
            let mut response = String::new();
            while let Ok(line) = reader.next_line().await {
                match line {
                    Some(line) => {
                        response.push_str(&line);
                        // LSPレスポンスの処理（JSONパースなど）
                        if let Ok(parsed_response) = serde_json::from_str::<LspResponse>(&response)
                        {
                            println!("Received response: {:?}", parsed_response);
                        }
                        response.clear();
                    }
                    None => break,
                }
            }
        });

        // LSPリクエストを送信
        let request = LspRequest {
            jsonrpc: "2.0".to_string(),
            id: 1,
            method: "initialize".to_string(),
            params: json!({
                "processId": std::process::id(),
                "rootUri": "file:///path/to/your/project",
                "capabilities": {},
            }),
        };

        let request_str = serde_json::to_string(&request).context("Failed to serialize request")?;

        // リクエストをサーバーに送信
        writer
            .write_all(request_str.as_bytes())
            .await
            .context("Failed to send request")?;
        writer.flush().await.context("Failed to flush writer")?;

        // レスポンスタスクが完了するのを待つ
        response_task
            .await
            .context("Response handling task failed")?;

        Ok(())
    }
}
