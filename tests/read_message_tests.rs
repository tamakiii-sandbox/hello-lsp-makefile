use tokio_test::io::Builder;
use bytes::Bytes;
use makefile_lsp::hello_lsp_makefile::read_message; // Import the read_message function from your module

#[tokio::test]
async fn test_read_message() {
    let input = b"Content-Length: 19\n\n{\"key\":\"value\"}";
    let input = Bytes::copy_from_slice(input);

    let reader = Builder::new().read(&input).build();
    let mut reader = tokio::io::BufReader::new(reader);

    let result = read_message(&mut reader).await;

    let mut expected_map = serde_json::Map::new();
    expected_map.insert("key".to_string(), serde_json::Value::String("value".to_string()));

    assert_eq!(result, Some(serde_json::Value::Object(expected_map)));
}

// use tokio::io::{AsyncBufReadExt, AsyncReadExt};
// use futures::io::Cursor;
// use serde_json::Value;

// #[tokio::test]
// async fn test_read_message() {
//     let input = b"Content-Length: 19\n\n{\"key\":\"value\"}";
//     let input = Bytes::copy_from_slice(input);

//     let mut reader = Builder::new().read(input).build();
//     let mut reader = tokio::io::BufReader::new(reader);

//     let result = read_message(&mut reader).await;

//     let mut expected_map = serde_json::Map::new();
//     expected_map.insert("key".to_string(), serde_json::Value::String("value".to_string()));

//     assert_eq!(result, Some(serde_json::Value::Object(expected_map)));
// }

// #[tokio::test]
// async fn test_read_message() {
//     let input = b"Content-Length: 19\n\n{\"key\":\"value\"}";
//     let bytes = Bytes::from_static(input);
//     let framed = FramedRead::new(bytes.reader(), BytesCodec::new());
//     let mut reader = framed.into_inner();

//     let result = read_message(&mut reader).await;

//     let expected_value = serde_json::json!({
//         "key": "value"
//     });

//     assert_eq!(result, Some(expected_value));
// }

// #[tokio::test]
// async fn test_read_message() {
//     let input = b"Content-Length: 19\n\n{\"key\":\"value\"}";
//     let mut reader = Cursor::new(&input[..]);

//     let result = read_message(&mut reader).await;

//     let expected_value = serde_json::json!({
//         "key": "value"
//     });

//     assert_eq!(result, Some(expected_value));
// }

// #[tokio::test]
// async fn test_read_message() {
//     let input = b"Content-Length: 19\n\n{\"key\":\"value\"}";
//     let mut reader = Cursor::new(&input[..]);

//     let result = read_message(&mut reader).await;

//     let expected_value = serde_json::json!({
//         "key": "value"
//     });

//     assert_eq!(result, Some(expected_value));
// }

// async fn test_read_message() {
//     let input = b"Content-Length: 19\n\n{\"key\":\"value\"}";
//     let mut reader = Cursor::new(&input[..]);

//     let result = read_message(&mut reader).await;

//     let mut expected_map = Map::new();
//     expected_map.insert("key".to_string(), Value::String("value".to_string()));

//     assert_eq!(result, Some(Value::Object(expected_map)));
// }


// #[tokio::test]
// async fn test_read_message() {
//     let input = b"Content-Length: 19\n\n{\"key\":\"value\"}";
//     let mut reader = Cursor::new(&input[..]);

//     let result = read_message(&mut reader).await;

//     let mut expected_map = Map::new();
//     expected_map.insert("key".to_string(), Value::String("value".to_string()));

//     assert_eq!(result, Some(Value::Object(expected_map)));
// }

// #[tokio::test]
// async fn test_read_message() {
//     let input = b"Content-Length: 19\n\n{\"key\":\"value\"}";
//     let mut reader = Cursor::new(&input[..]);

//     let result = read_message(&mut reader).await;

//     assert_eq!(
//         result,
//         Some(Value::Object(
//             vec![("key", Value::String("value".to_string()))]
//                 .into_iter()
//                 .collect()
//         ))
//     );
// }

// #[tokio::test]
// async fn test_read_message() {
//     let test_data = b"Content-Length: 13\n{\"foo\": \"bar\"}";

//     // Use a Cursor to provide an AsyncBufReadExt and Unpin implementation
//     let mut reader = Cursor::new(&test_data[..]);

//     let result = read_message(&mut reader).await;

//     let expected = Some(Value::Object(
//         vec![("foo", Value::String("bar".to_string()))]
//             .into_iter()
//             .collect(),
//     ));

//     assert_eq!(result, expected);
// }

// #[tokio::test]
// async fn test_read_message_with_invalid_json() {
//     let test_data = b"Content-Length: 10\ninvalidjson";

//     let mut reader = Cursor::new(&test_data[..]);

//     let result = read_message(&mut reader).await;

//     assert_eq!(result, None);
// }

// #[tokio::test]
// async fn test_read_message_with_missing_content_length() {
//     let test_data = b"Content-Length: \n{\"foo\": \"bar\"}";

//     let mut reader = Cursor::new(&test_data[..]);

//     let result = read_message(&mut reader).await;

//     assert_eq!(result, None);
// }

// #[tokio::test]
// async fn test_read_message_with_incorrect_content_length() {
//     let test_data = b"Content-Length: 5\n{\"foo\": \"bar\"}";

//     let mut reader = Cursor::new(&test_data[..]);

//     let result = read_message(&mut reader).await;

//     assert_eq!(result, None);
// }
