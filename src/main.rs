use async_std::io::{stdin, stdout};
use async_std::prelude::*;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader, BufWriter};
use lsp_types::{DidOpenTextDocumentParams, InitializeParams, InitializeResult, ServerCapabilities, TextDocumentItem};
use lsp_types::ResponseError;
use lsp_types::request::Request;
use serde_json::Value;
use serde_json::json;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(tag = "method", content = "params")]
enum SupportedRequest {
    Initialize(InitializeParams),
    // Add other supported requests here
}

async fn async_main() {
    let stdin = stdin();
    let stdout = stdout();
    let (reader, writer) = (BufReader::new(stdin), BufWriter::new(stdout));

    while let Some(message) = read_message(&mut reader).await {
        // JSON-RPCメッセージを処理する関数を呼び出します。
        process_message(&mut writer, message).await;
    }
}

async fn read_message(reader: &mut (impl AsyncBufReadExt + Unpin)) -> Option<Value> {
    let mut content_length = None;
    let mut buf = String::new();

    while content_length.is_none() {
        buf.clear();
        let _ = reader.read_line(&mut buf).await;
        if let Some(len) = buf.strip_prefix("Content-Length: ") {
            content_length = usize::from_str_radix(len.trim(), 10).ok();
        }
    }

    let content_length = content_length?;
    let mut message = vec![0; content_length];
    reader.read_exact(&mut message).await;

    serde_json::from_slice(&message).ok()
}

async fn process_message(writer: &mut impl AsyncWriteExt, message: Value) {
    let request: SupportedRequest = serde_json::from_value(message).unwrap();

    match request {
        SupportedRequest::Initialize(params) => {
            let params: InitializeParams = serde_json::from_value(request.params).unwrap();
            // ここで、サーバーの初期化処理を行います。
            let server_capabilities = ServerCapabilities {
                text_document_sync: None, // ここに適切なオプションを設定してください。
                hover_provider: None,
                completion_provider: None,
                signature_help_provider: None,
                definition_provider: None,
                references_provider: None,
                document_highlight_provider: None,
                document_symbol_provider: None,
                workspace_symbol_provider: None,
                code_action_provider: None,
                code_lens_provider: None,
                document_formatting_provider: None,
                document_range_formatting_provider: None,
                document_on_type_formatting_provider: None,
                rename_provider: None,
                document_link_provider: None,
                color_provider: None,
                folding_range_provider: None,
                execute_command_provider: None,
                workspace: None,
                semantic_tokens_provider: None,
                call_hierarchy_provider: None,
                selection_range_provider: None,
                linked_editing_range_provider: None,
                moniker_provider: None,
                experimental: None,
            };
            let result = InitializeResult {
                capabilities: server_capabilities,
                server_info: None,
            };

            let response = serde_json::to_value(&result).unwrap();
            write_message(writer, request.id, response).await;
        }
        "textDocument/didOpen" => {
            let params: DidOpenTextDocumentParams = serde_json::from_value(message["params"].clone()).unwrap();
            let document: TextDocumentItem = params.text_document;
            // ここで、ドキュメントが開かれたときの処理を行います。
        }
        _ => {
            let error = ResponseError {
                code: -32601, // Method not found
                message: "Method not found".to_string(),
                data: None,
            };
            let response = serde_json::to_value(&error).unwrap();
            write_message(writer, request.id, response).await;
        }
    }
}

async fn write_message(writer: &mut (impl AsyncWriteExt + Unpin), id: Value, result: Value) {
    let message = json!({
        "jsonrpc": "2.0",
        "id": id,
        "result": result,
    });

    let message_str = serde_json::to_string(&message).unwrap();
    let content_length = message_str.len();
    writer
        .write_all(format!("Content-Length: {}\r\n\r\n{}", content_length, message_str).as_bytes())
        .await;
}

fn main() {
    use tokio::runtime::Builder;
    let rt = Builder::new_multi_thread()
        .worker_threads(2)
        .enable_all()
        .build()
        .unwrap();
    rt.block_on(async_main());
}
