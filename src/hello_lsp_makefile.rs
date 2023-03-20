use tokio::io::{AsyncBufReadExt, AsyncReadExt};
use serde_json::Value;
use std::str::FromStr;

pub async fn read_message(reader: &mut (impl AsyncBufReadExt + std::marker::Unpin)) -> Option<Value> {
    let mut content_length = None;
    let mut buf = String::new();

    while content_length.is_none() {
        buf.clear();
        let _ = reader.read_line(&mut buf).await;
        if let Some(len) = buf.strip_prefix("Content-Length: ") {
            content_length = usize::from_str(len.trim()).ok();
        }
    }

    let content_length = content_length?;
    let mut message = vec![0; content_length];
    reader.read_exact(&mut message).await.ok()?;

    serde_json::from_slice(&message).ok()
}