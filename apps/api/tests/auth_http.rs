//! HTTP-level auth flow and API contract checks (full middleware stack).

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
    // Single connection: SQLite in-memory DB is per-connection unless shared cache is honored
    // consistently by the driver; one connection avoids empty-schema races in integration tests.
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
async fn register_then_session_with_bearer_and_request_id() {
    let pool = test_pool().await;
    let secret = "integration-test-jwt-secret-32chars!!";
    let state = AppState::for_tests(pool, secret);
    let cfg = AppConfig::for_tests(secret);
    let app = build_http_app(state, &cfg);

    let body = json!({
        "email": "user@example.com",
        "password": "correct-horse-battery-staple-unique"
    });
    let req = Request::builder()
        .method("POST")
        .uri("/api/v1/auth/register")
        .header("content-type", "application/json")
        .body(Body::from(body.to_string()))
        .expect("request");
    let res = app.clone().oneshot(req).await.expect("response");
    assert_eq!(res.status(), StatusCode::OK);
    assert!(
        res.headers().get("x-request-id").is_some(),
        "responses should include x-request-id for correlation"
    );
    let bytes = to_bytes(res.into_body(), usize::MAX).await.expect("body");
    let val: Value = serde_json::from_slice(&bytes).expect("json");
    let token = val["token"].as_str().expect("token");

    let session_req = Request::builder()
        .method("GET")
        .uri("/api/v1/auth/session")
        .header("authorization", format!("Bearer {token}"))
        .body(Body::empty())
        .expect("session request");
    let res2 = app.oneshot(session_req).await.expect("session response");
    assert_eq!(res2.status(), StatusCode::OK);
    let bytes2 = to_bytes(res2.into_body(), usize::MAX).await.expect("body");
    let session: Value = serde_json::from_slice(&bytes2).expect("session json");
    assert_eq!(session["authenticated"], true);
    assert_eq!(session["user"]["email"], "user@example.com");
}

#[tokio::test]
async fn login_returns_token_for_existing_user() {
    let pool = test_pool().await;
    let secret = "integration-test-jwt-secret-32chars!!";
    let state = AppState::for_tests(pool, secret);
    let cfg = AppConfig::for_tests(secret);
    let app = build_http_app(state, &cfg);

    let reg = json!({
        "email": "login@example.com",
        "password": "correct-horse-battery-staple-unique"
    });
    let req = Request::builder()
        .method("POST")
        .uri("/api/v1/auth/register")
        .header("content-type", "application/json")
        .body(Body::from(reg.to_string()))
        .expect("request");
    let res = app.clone().oneshot(req).await.expect("response");
    assert_eq!(res.status(), StatusCode::OK);

    let login = json!({
        "email": "login@example.com",
        "password": "correct-horse-battery-staple-unique"
    });
    let req = Request::builder()
        .method("POST")
        .uri("/api/v1/auth/login")
        .header("content-type", "application/json")
        .body(Body::from(login.to_string()))
        .expect("login request");
    let res = app.oneshot(req).await.expect("login response");
    assert_eq!(res.status(), StatusCode::OK);
    let bytes = to_bytes(res.into_body(), usize::MAX).await.expect("body");
    let val: Value = serde_json::from_slice(&bytes).expect("json");
    assert!(val["token"].as_str().is_some());
    assert_eq!(val["user"]["email"], "login@example.com");
}

#[tokio::test]
async fn register_duplicate_email_returns_conflict_with_code() {
    let pool = test_pool().await;
    let secret = "integration-test-jwt-secret-32chars!!";
    let state = AppState::for_tests(pool, secret);
    let cfg = AppConfig::for_tests(secret);
    let app = build_http_app(state, &cfg);

    let body = json!({
        "email": "dup@example.com",
        "password": "correct-horse-battery-staple-unique"
    });
    let req = Request::builder()
        .method("POST")
        .uri("/api/v1/auth/register")
        .header("content-type", "application/json")
        .body(Body::from(body.to_string()))
        .expect("request");
    let res = app.clone().oneshot(req).await.expect("response");
    assert_eq!(res.status(), StatusCode::OK);

    let req = Request::builder()
        .method("POST")
        .uri("/api/v1/auth/register")
        .header("content-type", "application/json")
        .body(Body::from(body.to_string()))
        .expect("request");
    let res = app.oneshot(req).await.expect("response");
    assert_eq!(res.status(), StatusCode::CONFLICT);
    let bytes = to_bytes(res.into_body(), usize::MAX).await.expect("body");
    let val: Value = serde_json::from_slice(&bytes).expect("json");
    assert_eq!(val["code"], "conflict");
    assert!(val["error"].as_str().is_some());
}

#[tokio::test]
async fn register_invalid_email_returns_validation_with_code() {
    let pool = test_pool().await;
    let secret = "integration-test-jwt-secret-32chars!!";
    let state = AppState::for_tests(pool, secret);
    let cfg = AppConfig::for_tests(secret);
    let app = build_http_app(state, &cfg);

    let body = json!({
        "email": "not-an-email",
        "password": "correct-horse-battery-staple-unique"
    });
    let req = Request::builder()
        .method("POST")
        .uri("/api/v1/auth/register")
        .header("content-type", "application/json")
        .body(Body::from(body.to_string()))
        .expect("request");
    let res = app.oneshot(req).await.expect("response");
    assert_eq!(res.status(), StatusCode::BAD_REQUEST);
    let bytes = to_bytes(res.into_body(), usize::MAX).await.expect("body");
    let val: Value = serde_json::from_slice(&bytes).expect("json");
    assert_eq!(val["code"], "validation");
}

#[tokio::test]
async fn auth_refresh_returns_new_access_and_refresh_tokens() {
    let pool = test_pool().await;
    let secret = "integration-test-jwt-secret-32chars!!";
    let state = AppState::for_tests(pool, secret);
    let cfg = AppConfig::for_tests(secret);
    let app = build_http_app(state, &cfg);

    let reg = json!({
        "email": "refresh@example.com",
        "password": "correct-horse-battery-staple-unique"
    });
    let req = Request::builder()
        .method("POST")
        .uri("/api/v1/auth/register")
        .header("content-type", "application/json")
        .body(Body::from(reg.to_string()))
        .expect("request");
    let res = app.clone().oneshot(req).await.expect("response");
    assert_eq!(res.status(), StatusCode::OK);
    let bytes = to_bytes(res.into_body(), usize::MAX).await.expect("body");
    let first: Value = serde_json::from_slice(&bytes).expect("json");
    let refresh = first["refresh_token"].as_str().expect("refresh_token");

    let refresh_body = json!({ "refresh_token": refresh });
    let req = Request::builder()
        .method("POST")
        .uri("/api/v1/auth/refresh")
        .header("content-type", "application/json")
        .body(Body::from(refresh_body.to_string()))
        .expect("refresh request");
    let res = app.oneshot(req).await.expect("refresh response");
    assert_eq!(res.status(), StatusCode::OK);
    let bytes = to_bytes(res.into_body(), usize::MAX).await.expect("body");
    let second: Value = serde_json::from_slice(&bytes).expect("json");
    assert!(second["token"].as_str().is_some());
    assert!(second["refresh_token"].as_str().is_some());
    assert_ne!(
        second["refresh_token"].as_str().unwrap(),
        refresh,
        "refresh token should rotate"
    );
    assert_eq!(second["expires_in"], 60 * 15);
}

#[tokio::test]
async fn login_unknown_user_returns_unauthorized_without_code_leak() {
    let pool = test_pool().await;
    let secret = "integration-test-jwt-secret-32chars!!";
    let state = AppState::for_tests(pool, secret);
    let cfg = AppConfig::for_tests(secret);
    let app = build_http_app(state, &cfg);

    let body = json!({
        "email": "nobody@example.com",
        "password": "correct-horse-battery-staple-unique"
    });
    let req = Request::builder()
        .method("POST")
        .uri("/api/v1/auth/login")
        .header("content-type", "application/json")
        .body(Body::from(body.to_string()))
        .expect("request");
    let res = app.oneshot(req).await.expect("response");
    assert_eq!(res.status(), StatusCode::UNAUTHORIZED);
    let bytes = to_bytes(res.into_body(), usize::MAX).await.expect("body");
    let val: Value = serde_json::from_slice(&bytes).expect("json");
    assert_eq!(val["code"], "unauthorized");
}

#[tokio::test]
async fn post_job_and_queue_stats_reflect_depth() {
    let pool = test_pool().await;
    let secret = "integration-test-jwt-secret-32chars!!";
    let state = AppState::for_tests(pool, secret);
    let cfg = AppConfig::for_tests(secret);
    let app = build_http_app(state, &cfg);

    let body = json!({ "job_id": "  job-42  " });
    let req = Request::builder()
        .method("POST")
        .uri("/api/v1/jobs")
        .header("content-type", "application/json")
        .body(Body::from(body.to_string()))
        .expect("request");
    let res = app.clone().oneshot(req).await.expect("response");
    assert_eq!(res.status(), StatusCode::ACCEPTED);

    let req = Request::builder()
        .method("GET")
        .uri("/api/v1/queue/stats")
        .body(Body::empty())
        .expect("stats");
    let res = app.oneshot(req).await.expect("stats response");
    assert_eq!(res.status(), StatusCode::OK);
    let bytes = to_bytes(res.into_body(), usize::MAX).await.expect("body");
    let val: Value = serde_json::from_slice(&bytes).expect("json");
    assert_eq!(val["depth"], 1);
}

#[tokio::test]
async fn smtp_settings_get_requires_bearer() {
    let pool = test_pool().await;
    let secret = "integration-test-jwt-secret-32chars!!";
    let state = AppState::for_tests(pool, secret);
    let cfg = AppConfig::for_tests(secret);
    let app = build_http_app(state, &cfg);

    let req = Request::builder()
        .method("GET")
        .uri("/api/v1/settings/smtp")
        .body(Body::empty())
        .expect("request");
    let res = app.oneshot(req).await.expect("response");
    assert_eq!(res.status(), StatusCode::UNAUTHORIZED);
}

#[tokio::test]
async fn smtp_settings_get_put_roundtrip() {
    let pool = test_pool().await;
    let secret = "integration-test-jwt-secret-32chars!!";
    let state = AppState::for_tests(pool, secret);
    let cfg = AppConfig::for_tests(secret);
    let app = build_http_app(state, &cfg);

    let reg = json!({
        "email": "smtp-settings@example.com",
        "password": "correct-horse-battery-staple-unique"
    });
    let req = Request::builder()
        .method("POST")
        .uri("/api/v1/auth/register")
        .header("content-type", "application/json")
        .body(Body::from(reg.to_string()))
        .expect("register");
    let res = app.clone().oneshot(req).await.expect("response");
    assert_eq!(res.status(), StatusCode::OK);
    let bytes = to_bytes(res.into_body(), usize::MAX).await.expect("body");
    let val: Value = serde_json::from_slice(&bytes).expect("json");
    let token = val["token"].as_str().expect("token");

    let req = Request::builder()
        .method("GET")
        .uri("/api/v1/settings/smtp")
        .header("authorization", format!("Bearer {token}"))
        .body(Body::empty())
        .expect("get smtp");
    let res = app.clone().oneshot(req).await.expect("response");
    assert_eq!(res.status(), StatusCode::OK);
    let bytes = to_bytes(res.into_body(), usize::MAX).await.expect("body");
    let get0: Value = serde_json::from_slice(&bytes).expect("json");
    assert_eq!(get0["enabled"], false);
    assert_eq!(get0["active_source"], "none");

    let put = json!({
        "enabled": true,
        "host": "smtp.example.com",
        "port": 587,
        "username": "relay",
        "password": "secret-smtp-pass",
        "from": "PE <noreply@example.com>"
    });
    let req = Request::builder()
        .method("PUT")
        .uri("/api/v1/settings/smtp")
        .header("authorization", format!("Bearer {token}"))
        .header("content-type", "application/json")
        .body(Body::from(put.to_string()))
        .expect("put smtp");
    let res = app.clone().oneshot(req).await.expect("response");
    assert_eq!(res.status(), StatusCode::OK);
    let bytes = to_bytes(res.into_body(), usize::MAX).await.expect("body");
    let saved: Value = serde_json::from_slice(&bytes).expect("json");
    assert_eq!(saved["enabled"], true);
    assert_eq!(saved["host"], "smtp.example.com");
    assert_eq!(saved["has_password"], true);
    assert_eq!(saved["active_source"], "database");
    assert_eq!(saved["username"], "relay");
}
