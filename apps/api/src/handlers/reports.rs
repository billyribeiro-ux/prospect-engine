use axum::body::Bytes;
use axum::http::{header, StatusCode};
use axum::response::IntoResponse;
use axum::Json;
use printpdf::{BuiltinFont, Mm, PdfDocument};
use serde_json::{json, Value};

pub async fn get_reports() -> Json<Value> {
    Json(json!({ "status": "stub", "reports": [] }))
}

/// Returns a minimal single-page PDF (server-side export path).
pub async fn get_report_pdf() -> Result<impl IntoResponse, StatusCode> {
    let (doc, page1, layer1) = PdfDocument::new("ProspectEngine", Mm(210.0), Mm(297.0), "Layer");
    let layer = doc.get_page(page1).get_layer(layer1);
    let font = doc
        .add_builtin_font(BuiltinFont::Helvetica)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    layer.use_text(
        "ProspectEngine report (stub)",
        14.0,
        Mm(20.0),
        Mm(270.0),
        &font,
    );
    let buf = doc
        .save_to_bytes()
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok((
        StatusCode::OK,
        [
            (header::CONTENT_TYPE, "application/pdf"),
            (
                header::CONTENT_DISPOSITION,
                "inline; filename=\"report.pdf\"",
            ),
        ],
        Bytes::from(buf),
    ))
}
