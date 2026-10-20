#![cfg(feature = "ocaf-integration")]
use ocaf_runtime::runtime::{OcafRuntime, RuntimeError, ToolCallRequest};
use std::io::{Cursor, Write};
use zip::{write::FileOptions, ZipWriter};
use zip_warden_agent::zenoh::{manifest, AgentTool};
fn args() -> serde_json::Value {
    let mut out = Cursor::new(Vec::<u8>::new());
    {
        let mut writer = ZipWriter::new(&mut out);
        writer
            .start_file("personal/file.txt", FileOptions::default())
            .unwrap();
        writer.write_all(b"private example").unwrap();
        writer.finish().unwrap();
    }
    serde_json::json!({"archive_hex":hex::encode(out.into_inner())})
}
#[tokio::test]
async fn report_never_leaks_name_or_claims_semantic_review() {
    let rt = OcafRuntime::new(0);
    rt.register(manifest(), AgentTool).await.unwrap();
    let res = rt
        .execute(ToolCallRequest {
            call_id: "zip010".into(),
            tool: "archive.audit_zip".into(),
            args: args(),
            auth: None,
        })
        .await
        .unwrap();
    assert_eq!(res.output["privacy_names_included"], false);
    assert_eq!(res.output["archive_extracted"], false);
    assert_eq!(res.output["semantic_review_performed"], false);
    assert_eq!(res.output["files"][0]["path"], serde_json::Value::Null);
}
#[tokio::test]
async fn model_cannot_opt_into_private_names() {
    let rt = OcafRuntime::new(0);
    rt.register(manifest(), AgentTool).await.unwrap();
    let mut a = args();
    a["include_names"] = serde_json::json!(true);
    let result = rt
        .execute(ToolCallRequest {
            call_id: "leak010".into(),
            tool: "archive.audit_zip".into(),
            args: a,
            auth: None,
        })
        .await;
    assert!(result.is_err());
}
#[tokio::test]
async fn unsigned_peer_denied() {
    let rt = OcafRuntime::new(0)
        .with_tenant_namespace("tenant1")
        .unwrap();
    rt.register(manifest(), AgentTool).await.unwrap();
    let result = rt
        .execute_authenticated_peer(
            ToolCallRequest {
                call_id: "unsigned010".into(),
                tool: "archive.audit_zip".into(),
                args: args(),
                auth: None,
            },
            "peer",
        )
        .await;
    assert!(matches!(result, Err(RuntimeError::AuthRequired(_))));
}
