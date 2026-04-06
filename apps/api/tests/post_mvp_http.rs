//! Post-MVP HTTP coverage: durable job inspect, map heatmap/route, email tracking, rate limits.

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
async fn get_durable_job_reflects_inserted_row() {
    let pool = test_pool().await;
    let secret = "integration-test-jwt-secret-32chars!!";
    let state = AppState::for_tests(pool, secret);
    let cfg = AppConfig::for_tests(secret);
    let app = build_http_app(state, &cfg);

    let body = json!({ "job_id": "  job-99  " });
    let req = Request::builder()
        .method("POST")
        .uri("/api/v1/jobs")
        .header("content-type", "application/json")
        .body(Body::from(body.to_string()))
        .expect("request");
    let res = app.clone().oneshot(req).await.expect("response");
    assert_eq!(res.status(), StatusCode::ACCEPTED);
    let bytes = to_bytes(res.into_body(), usize::MAX).await.expect("body");
    let val: Value = serde_json::from_slice(&bytes).expect("json");
    let durable_id = val["durable_job_id"].as_str().expect("durable_job_id");

    let req = Request::builder()
        .method("GET")
        .uri(format!("/api/v1/jobs/durable/{durable_id}"))
        .body(Body::empty())
        .expect("get durable");
    let res = app.oneshot(req).await.expect("response");
    assert_eq!(res.status(), StatusCode::OK);
    let bytes = to_bytes(res.into_body(), usize::MAX).await.expect("body");
    let row: Value = serde_json::from_slice(&bytes).expect("json");
    assert_eq!(row["kind"], "generic");
    assert_eq!(row["status"], "pending");
}

#[tokio::test]
async fn map_heatmap_returns_points_for_leads_with_coords() {
    let pool = test_pool().await;
    let secret = "integration-test-jwt-secret-32chars!!";
    let state = AppState::for_tests(pool, secret);
    let cfg = AppConfig::for_tests(secret);
    let app = build_http_app(state, &cfg);

    let body = json!({
        "name": "A",
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
        .uri("/api/v1/map/heatmap")
        .body(Body::empty())
        .expect("heatmap");
    let res = app.oneshot(req).await.expect("response");
    assert_eq!(res.status(), StatusCode::OK);
    let bytes = to_bytes(res.into_body(), usize::MAX).await.expect("body");
    let val: Value = serde_json::from_slice(&bytes).expect("json");
    assert_eq!(val["status"], "ok");
    assert!(val["points"].as_array().is_some_and(|a| a.len() == 1));
}

#[tokio::test]
async fn map_route_returns_linestring_between_two_leads() {
    let pool = test_pool().await;
    let secret = "integration-test-jwt-secret-32chars!!";
    let state = AppState::for_tests(pool, secret);
    let cfg = AppConfig::for_tests(secret);
    let app = build_http_app(state, &cfg);

    let mut ids = Vec::new();
    for (name, lat, lon) in [("A", 40.7, -74.0), ("B", 40.8, -74.1)] {
        let body = json!({ "name": name, "latitude": lat, "longitude": lon });
        let req = Request::builder()
            .method("POST")
            .uri("/api/v1/leads")
            .header("content-type", "application/json")
            .body(Body::from(body.to_string()))
            .expect("request");
        let res = app.clone().oneshot(req).await.expect("lead");
        let bytes = to_bytes(res.into_body(), usize::MAX).await.expect("body");
        let v: Value = serde_json::from_slice(&bytes).expect("json");
        ids.push(v["id"].as_str().expect("id").to_string());
    }

    let q = format!("/api/v1/map/route?from_id={}&to_id={}", ids[0], ids[1]);
    let req = Request::builder()
        .method("GET")
        .uri(q)
        .body(Body::empty())
        .expect("route");
    let res = app.oneshot(req).await.expect("response");
    assert_eq!(res.status(), StatusCode::OK);
    let bytes = to_bytes(res.into_body(), usize::MAX).await.expect("body");
    let val: Value = serde_json::from_slice(&bytes).expect("json");
    assert_eq!(val["status"], "ok");
    assert!(val["distance_meters"].as_f64().is_some_and(|d| d > 0.0));
    assert_eq!(val["geojson"]["type"], "LineString");
}

#[tokio::test]
async fn email_open_pixel_increments_opens() {
    let pool = test_pool().await;
    let secret = "integration-test-jwt-secret-32chars!!";
    let pool_assert = pool.clone();
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
    let res = app.clone().oneshot(req).await.expect("response");
    assert_eq!(res.status(), StatusCode::ACCEPTED);
    let bytes = to_bytes(res.into_body(), usize::MAX).await.expect("body");
    let val: Value = serde_json::from_slice(&bytes).expect("json");
    let open = val["tracking"]["openPixelUrl"]
        .as_str()
        .expect("openPixelUrl");
    let token = open
        .strip_prefix("/api/v1/email/track/open/")
        .expect("open path");

    let req = Request::builder()
        .method("GET")
        .uri(open)
        .body(Body::empty())
        .expect("pixel");
    let res = app.oneshot(req).await.expect("pixel res");
    assert_eq!(res.status(), StatusCode::OK);
    assert_eq!(
        res.headers()
            .get("content-type")
            .and_then(|v| v.to_str().ok()),
        Some("image/gif")
    );

    let row: (i64,) = sqlx::query_as("SELECT opens FROM email_events WHERE tracking_token = ?")
        .bind(token)
        .fetch_one(&pool_assert)
        .await
        .expect("opens row");
    assert_eq!(row.0, 1);
}

#[tokio::test]
async fn email_click_redirect_counts_and_location() {
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
    let res = app.clone().oneshot(req).await.expect("response");
    let bytes = to_bytes(res.into_body(), usize::MAX).await.expect("body");
    let val: Value = serde_json::from_slice(&bytes).expect("json");
    let click = val["tracking"]["clickUrlTemplate"]
        .as_str()
        .expect("clickUrlTemplate");
    assert!(
        click.ends_with("?u="),
        "clickUrlTemplate should end with ?u="
    );
    let target = "https://example.com/path";
    let uri = format!("{click}{}", urlencoding::encode(target));

    let req = Request::builder()
        .method("GET")
        .uri(&uri)
        .body(Body::empty())
        .expect("click");
    let res = app.oneshot(req).await.expect("click res");
    assert_eq!(res.status(), StatusCode::TEMPORARY_REDIRECT);
    assert_eq!(
        res.headers().get("location").and_then(|v| v.to_str().ok()),
        Some(target)
    );
}

#[tokio::test]
async fn email_send_rate_limited_after_burst() {
    let pool = test_pool().await;
    let secret = "integration-test-jwt-secret-32chars!!";
    let state = AppState::for_tests_with_email_rate(pool, secret, 2);
    let cfg = AppConfig::for_tests_with_email_rate(secret, 2);
    let app = build_http_app(state, &cfg);

    let body = json!({
        "to": "burst@example.com",
        "subject": "Hello",
        "body": "x"
    });
    let body_str = body.to_string();

    for _ in 0..2 {
        let req = Request::builder()
            .method("POST")
            .uri("/api/v1/email/send")
            .header("content-type", "application/json")
            .header("x-forwarded-for", "203.0.113.50")
            .body(Body::from(body_str.clone()))
            .expect("request");
        let res = app.clone().oneshot(req).await.expect("response");
        assert_eq!(res.status(), StatusCode::ACCEPTED);
    }

    let req = Request::builder()
        .method("POST")
        .uri("/api/v1/email/send")
        .header("content-type", "application/json")
        .header("x-forwarded-for", "203.0.113.50")
        .body(Body::from(body_str))
        .expect("request");
    let res = app.oneshot(req).await.expect("response");
    assert_eq!(res.status(), StatusCode::TOO_MANY_REQUESTS);
    let bytes = to_bytes(res.into_body(), usize::MAX).await.expect("body");
    let val: Value = serde_json::from_slice(&bytes).expect("json");
    assert_eq!(val["code"], "rate_limited");
}
