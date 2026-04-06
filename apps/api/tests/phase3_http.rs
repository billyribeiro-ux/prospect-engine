//! Phase 3 domain endpoints (audit, map, discovery jobs, email stub).

use axum::body::{to_bytes, Body};
use axum::http::{Request, StatusCode};
use serde_json::{json, Value};
use sqlx::any::{install_default_drivers, AnyPoolOptions};
use tower::ServiceExt;

use api::build_http_app;
use api::config::AppConfig;
use api::state::AppState;

async fn test_pool() -> sqlx::AnyPool {
    install_default_drivers();
    let pool = AnyPoolOptions::new()
        .max_connections(1)
        .connect("sqlite::memory:?cache=shared")
        .await
        .expect("pool");
    sqlx::migrate!("./migrations")
        .run(&pool)
        .await
        .expect("migrate");
    pool
}

#[tokio::test]
async fn post_audit_run_scores_html() {
    let pool = test_pool().await;
    let secret = "integration-test-jwt-secret-32chars!!";
    let state = AppState::for_tests(pool, secret);
    let cfg = AppConfig::for_tests(secret);
    let app = build_http_app(state, &cfg);

    let body = json!({
        "html": "<html><head><title>T</title></head><body><h1>x</h1><p>word word word</p></body></html>"
    });
    let req = Request::builder()
        .method("POST")
        .uri("/api/v1/audit/run")
        .header("content-type", "application/json")
        .body(Body::from(body.to_string()))
        .expect("request");
    let res = app.oneshot(req).await.expect("response");
    assert_eq!(res.status(), StatusCode::OK);
    let bytes = to_bytes(res.into_body(), usize::MAX).await.expect("body");
    let val: Value = serde_json::from_slice(&bytes).expect("json");
    assert_eq!(val["status"], "complete");
    assert!(val["composite"].as_u64().is_some());
    assert!(val["dimensions"].as_array().is_some());
}

#[tokio::test]
async fn post_discovery_job_accepted() {
    let pool = test_pool().await;
    let secret = "integration-test-jwt-secret-32chars!!";
    let state = AppState::for_tests(pool, secret);
    let cfg = AppConfig::for_tests(secret);
    let app = build_http_app(state, &cfg);

    let body = json!({ "url": "https://example.com" });
    let req = Request::builder()
        .method("POST")
        .uri("/api/v1/discovery/jobs")
        .header("content-type", "application/json")
        .body(Body::from(body.to_string()))
        .expect("request");
    let res = app.oneshot(req).await.expect("response");
    assert_eq!(res.status(), StatusCode::ACCEPTED);
}

#[tokio::test]
async fn map_markers_after_lead_with_coords() {
    let pool = test_pool().await;
    let secret = "integration-test-jwt-secret-32chars!!";
    let state = AppState::for_tests(pool, secret);
    let cfg = AppConfig::for_tests(secret);
    let app = build_http_app(state, &cfg);

    let body = json!({
        "name": "Cafe",
        "latitude": 40.7,
        "longitude": -74.0
    });
    let req = Request::builder()
        .method("POST")
        .uri("/api/v1/leads")
        .header("content-type", "application/json")
        .body(Body::from(body.to_string()))
        .expect("request");
    let _ = app.clone().oneshot(req).await.expect("lead");

    let req = Request::builder()
        .method("GET")
        .uri("/api/v1/map")
        .body(Body::empty())
        .expect("map");
    let res = app.oneshot(req).await.expect("map res");
    assert_eq!(res.status(), StatusCode::OK);
    let bytes = to_bytes(res.into_body(), usize::MAX).await.expect("body");
    let val: Value = serde_json::from_slice(&bytes).expect("json");
    assert!(val["markers"].as_array().is_some_and(|a| a.len() == 1));
}

#[tokio::test]
async fn email_send_stub_persists_event() {
    let pool = test_pool().await;
    let secret = "integration-test-jwt-secret-32chars!!";
    let state = AppState::for_tests(pool, secret);
    let cfg = AppConfig::for_tests(secret);
    let app = build_http_app(state, &cfg);

    let body = json!({
        "to": "ops@example.com",
        "subject": "Hello",
        "body": "Test body"
    });
    let req = Request::builder()
        .method("POST")
        .uri("/api/v1/email/send")
        .header("content-type", "application/json")
        .body(Body::from(body.to_string()))
        .expect("request");
    let res = app.oneshot(req).await.expect("response");
    assert_eq!(res.status(), StatusCode::ACCEPTED);
    let bytes = to_bytes(res.into_body(), usize::MAX).await.expect("body");
    let val: Value = serde_json::from_slice(&bytes).expect("json");
    assert_eq!(val["mode"], "stub");
}
