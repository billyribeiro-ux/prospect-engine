//! Global HTTP middleware: body limits, CORS, security headers, request IDs, tracing.

use axum::body::Body;
use axum::extract::DefaultBodyLimit;
use axum::http::header::{self, HeaderName, HeaderValue};
use axum::http::{Method, Request};
use axum::Router;

use tower_http::cors::{AllowHeaders, AllowMethods, AllowOrigin, CorsLayer, ExposeHeaders};
use tower_http::request_id::{
    MakeRequestUuid, PropagateRequestIdLayer, RequestId, SetRequestIdLayer,
};
use tower_http::set_header::SetResponseHeaderLayer;
use tower_http::trace::TraceLayer;

use crate::config::AppConfig;
/// Applies production-oriented layers around the API router (see `crate::build_http_app`).
///
/// The router must already have [`crate::state::AppState`] applied via [`axum::Router::with_state`],
/// so the Axum type is [`Router`] (equivalently `Router<()>`).
pub fn apply_global_layers(router: Router, cfg: &AppConfig) -> Router {
    let x_request_id = HeaderName::from_static("x-request-id");
    router
        .layer(DefaultBodyLimit::max(crate::MAX_JSON_BODY_BYTES))
        .layer(cors_layer(cfg))
        .layer(SetResponseHeaderLayer::overriding(
            HeaderName::from_static("x-content-type-options"),
            HeaderValue::from_static("nosniff"),
        ))
        .layer(SetResponseHeaderLayer::overriding(
            HeaderName::from_static("x-frame-options"),
            HeaderValue::from_static("DENY"),
        ))
        .layer(SetResponseHeaderLayer::overriding(
            HeaderName::from_static("referrer-policy"),
            HeaderValue::from_static("strict-origin-when-cross-origin"),
        ))
        .layer(SetResponseHeaderLayer::overriding(
            HeaderName::from_static("permissions-policy"),
            HeaderValue::from_static(
                "accelerometer=(), camera=(), geolocation=(), microphone=(), usb=()",
            ),
        ))
        .layer(
            TraceLayer::new_for_http().make_span_with(|req: &Request<Body>| {
                let rid = req
                    .extensions()
                    .get::<RequestId>()
                    .and_then(|id| id.header_value().to_str().ok())
                    .unwrap_or("-");
                tracing::info_span!(
                    "http_request",
                    method = %req.method(),
                    uri = %req.uri(),
                    request_id = %rid,
                )
            }),
        )
        .layer(PropagateRequestIdLayer::new(x_request_id.clone()))
        .layer(SetRequestIdLayer::new(x_request_id, MakeRequestUuid))
}

fn cors_layer(cfg: &AppConfig) -> CorsLayer {
    if cfg.cors_allow_origins.is_empty() {
        return CorsLayer::permissive();
    }

    let origins: Vec<HeaderValue> = cfg
        .cors_allow_origins
        .iter()
        .filter_map(|o| HeaderValue::from_str(o.trim()).ok())
        .collect();

    if origins.is_empty() {
        tracing::warn!(
            "PE_CORS_ALLOW_ORIGINS had no valid entries; falling back to permissive CORS"
        );
        return CorsLayer::permissive();
    }

    CorsLayer::new()
        .allow_origin(AllowOrigin::list(origins))
        .allow_methods(AllowMethods::list([
            Method::GET,
            Method::POST,
            Method::PUT,
            Method::PATCH,
            Method::DELETE,
            Method::OPTIONS,
        ]))
        .allow_headers(AllowHeaders::list([
            header::AUTHORIZATION,
            header::CONTENT_TYPE,
            header::ACCEPT,
            HeaderName::from_static("x-request-id"),
        ]))
        .expose_headers(ExposeHeaders::list([HeaderName::from_static(
            "x-request-id",
        )]))
}
