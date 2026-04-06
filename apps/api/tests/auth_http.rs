//! HTTP-level auth flow: register → session with Bearer token.

use axum::body::{to_bytes, Body};
use axum::http::{Request, StatusCode};
use serde_json::{json, Value};
use sqlx::sqlite::SqlitePool;
use tower::ServiceExt;

use api::router::api_router;
use api::state::AppState;

#[tokio::test]
async fn register_then_session_with_bearer() {
    let pool = SqlitePool::connect("sqlite::memory:?cache=shared")
        .await
        .expect("pool");
    sqlx::migrate!("./migrations")
        .run(&pool)
        .await
        .expect("migrate");
    let secret = "integration-test-jwt-secret-32chars!!";
    let state = AppState {
        pool,
        jwt_secret: secret.to_string(),
    };
    let app = api_router(state);

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
    let pool = SqlitePool::connect("sqlite::memory:?cache=shared")
        .await
        .expect("pool");
    sqlx::migrate!("./migrations")
        .run(&pool)
        .await
        .expect("migrate");
    let secret = "integration-test-jwt-secret-32chars!!";
    let state = AppState {
        pool,
        jwt_secret: secret.to_string(),
    };
    let app = api_router(state);

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
