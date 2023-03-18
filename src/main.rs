use tokio::runtime::Builder;
use tokio::io::{AsyncBufReadExt, AsyncReadExt, AsyncWriteExt, BufReader, BufWriter, stdin, stdout};
use serde_json::Value;
use std::error::Error;
use std::str::FromStr;

// mod main;

// #[derive(Debug, Serialize)]
// struct ResponseError {
//     code: i32,
//     message: String,
// }

async fn async_main() {
    let mut reader = BufReader::new(stdin());
    let mut writer = BufWriter::new(stdout());

    while let Some(message) = read_message(&mut reader).await {
        process_message(&mut writer, message).await.ok();
    }
}

// struct HoverContext {
//     position: lsp_types::Position,
//     text_document: lsp_types::TextDocumentIdentifier,
// }

async fn process_message(_writer: &mut impl AsyncWriteExt, _message: Value) -> Result<(), Box<dyn Error>> {
    // let content_type = message.get("method").and_then(|m| m.as_str());

    // match content_type {
    //     Some(request @ "initialize") => {
    //         let response = json!({
    //             "jsonrpc": "2.0",
    //             "id": message["id"],
    //             "result": {
    //                 "capilities": {
    //                     "textDocumentSync": lsp_types::TextDocumentSyncCapability::Kind(lsp_types::TextDocumentSyncKind::Full)
    //                 }
    //             }
    //         });
            
    //         send_response(writer, response).await
    //     },
    //     Some(request @ "textDocument/hover") => {
    //         let hover_request: Request<HoverRequest> = serde_json::from_value(message)?;
    //         let hover_args = hover_request.params.expect("missing hover parameter");

    //         let hover_context = HoverContext {
    //             position: hover_args.position.expect("missing hover position"),
    //             text_document: hover_args.text_document.clone(),
    //         };

    //         let document_text = include_str!("test.txt"); // replace this with a call to fetch document text
    //         let contents = lsp_types::TextDocumentContentChangeEvent {
    //             range: None,
    //             range_length: None,
    //             text: document_text.to_string(),
    //         };
            
    //         let response = json!({
    //             "jsonrpc": "2.0",
    //             "id": hover_request.id,
    //             "result": {
    //                 "contents": lsp_types::HoverContents::Markup(lsp_types::MarkupContent {
    //                     kind: lsp_types::MarkupKind::Markdown,
    //                     value: String::from(
    //                         "Hello World"
    //                     )
    //                 }),
    //                 "range": hover_args.position,
    //             }
    //         });

    //         send_response(writer, response).await
    //     },
    //     _ => Ok(()),
    // }
    Ok(())
}

// async fn send_response(writer: &mut impl AsyncWriteExt, response: serde_json::Value) -> Result<(), Box<dyn Error>> {
//     let message = serde_json::to_string(&response)?;
//     let content_length = message.len();
//     writer
//         .write_all(format!("Content-Length: {}\r\n\r\n{}", content_length, message).as_bytes())
//         .await?;
//     writer.flush().await?;

//     Ok(())
// }

async fn read_message(reader: &mut (impl AsyncBufReadExt + std::marker::Unpin)) -> Option<Value> {
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

fn main() {
    let rt = Builder::new_multi_thread()
        .worker_threads(2)
        .enable_all()
        .build()
        .unwrap();
    rt.block_on(async_main());
}
