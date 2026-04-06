#![deny(clippy::all, clippy::pedantic)]
#![allow(clippy::cast_possible_truncation)]
#![allow(clippy::cast_sign_loss)]
#![allow(clippy::cast_precision_loss)]

//! Heuristic **audit scoring** over raw HTML (no headless browser). Scores are **indicative** for Phase 3;
//! replace with deeper analysis or ML as the product matures.

use serde::Serialize;
use serde_json::{json, Value};

/// Top-level composite (0–100) plus per-dimension breakdown (matches `@pe/types` audit shapes).
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AuditScore {
    pub composite: u8,
    pub dimensions: Vec<DimensionScore>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DimensionScore {
    pub dimension: &'static str,
    /// 0–100
    pub score: f64,
    pub weight: f64,
    pub signals: Value,
}

#[allow(clippy::struct_excessive_bools)]
struct HtmlSignals {
    words: usize,
    script_tags: u32,
    style_tags: u32,
    link_tags: u32,
    img_tags: u32,
    imgs_with_alt: u32,
    h1_count: u32,
    has_title: bool,
    has_viewport: bool,
    has_meta_desc: bool,
    mixed_http: bool,
    legacy_tags: u32,
}

fn analyze_html(html: &str) -> HtmlSignals {
    let lower = html.to_lowercase();
    HtmlSignals {
        words: visible_text_word_count(html),
        script_tags: count_substring(&lower, "<script"),
        style_tags: count_substring(&lower, "<style"),
        link_tags: count_substring(&lower, "<link"),
        img_tags: count_substring(&lower, "<img"),
        imgs_with_alt: count_imgs_with_alt(&lower),
        h1_count: count_substring(&lower, "<h1"),
        has_title: lower.contains("<title") && lower.contains("</title>"),
        has_viewport: lower.contains("viewport"),
        has_meta_desc: lower.contains("name=\"description\"")
            || lower.contains("name='description'"),
        mixed_http: lower.contains("http://") && lower.contains("https://"),
        legacy_tags: count_substring(&lower, "<font")
            + count_substring(&lower, "<center")
            + count_substring(&lower, "<marquee"),
    }
}

/// Run all heuristics and return a weighted composite.
#[must_use]
pub fn score_html(html: &str) -> AuditScore {
    if html.trim().is_empty() {
        return empty_audit();
    }
    let s = analyze_html(html);
    build_audit_score(&s)
}

fn empty_audit() -> AuditScore {
    let z = |dim: &'static str| DimensionScore {
        dimension: dim,
        score: 0.0,
        weight: 1.0,
        signals: json!({ "reason": "empty_input" }),
    };
    AuditScore {
        composite: 0,
        dimensions: vec![
            z("performance"),
            z("seo"),
            z("content"),
            z("design"),
            z("accessibility"),
            z("security"),
            z("tech_debt"),
        ],
    }
}

fn build_audit_score(s: &HtmlSignals) -> AuditScore {
    let perf_penalty = f64::from((s.script_tags * 6 + s.style_tags * 4 + s.link_tags).min(70));
    let performance = (100.0 - perf_penalty).clamp(0.0, 100.0);

    let mut seo: f64 = 0.0;
    if s.has_title {
        seo += 35.0;
    }
    if s.has_meta_desc {
        seo += 30.0;
    }
    seo += match s.h1_count {
        0 => 10.0,
        1 => 25.0,
        _ => 15.0,
    };
    seo = seo.min(100.0);

    let wc = s.words.min(10_000_000);
    let content = (f64::from(u32::try_from(wc).unwrap_or(u32::MAX)) / 4.0).min(100.0);

    let design = if s.has_viewport { 75.0 } else { 45.0 };

    let accessibility = if s.img_tags == 0 {
        85.0
    } else {
        let ratio = f64::from(s.imgs_with_alt) / f64::from(s.img_tags);
        (ratio * 100.0).min(100.0)
    };

    let security = if s.mixed_http { 55.0 } else { 88.0 };

    let tech_debt = (100.0 - f64::from(s.legacy_tags * 12)).clamp(0.0, 100.0);

    let dimensions = vec![
        DimensionScore {
            dimension: "performance",
            score: performance,
            weight: 1.0,
            signals: json!({
                "scriptTags": s.script_tags,
                "styleTags": s.style_tags,
                "linkTags": s.link_tags,
                "visibleWordCount": s.words,
            }),
        },
        DimensionScore {
            dimension: "seo",
            score: seo,
            weight: 1.0,
            signals: json!({
                "hasTitle": s.has_title,
                "hasMetaDescription": s.has_meta_desc,
                "h1Count": s.h1_count,
            }),
        },
        DimensionScore {
            dimension: "content",
            score: content,
            weight: 1.0,
            signals: json!({ "visibleWordCount": s.words }),
        },
        DimensionScore {
            dimension: "design",
            score: design,
            weight: 1.0,
            signals: json!({ "hasViewportMeta": s.has_viewport }),
        },
        DimensionScore {
            dimension: "accessibility",
            score: accessibility,
            weight: 1.0,
            signals: json!({
                "imgTags": s.img_tags,
                "imgsWithAlt": s.imgs_with_alt,
            }),
        },
        DimensionScore {
            dimension: "security",
            score: security,
            weight: 1.0,
            signals: json!({ "possibleMixedContent": s.mixed_http }),
        },
        DimensionScore {
            dimension: "tech_debt",
            score: tech_debt,
            weight: 1.0,
            signals: json!({ "legacyTagHints": s.legacy_tags }),
        },
    ];

    let sum_w: f64 = dimensions.iter().map(|d| d.weight).sum();
    let composite_f: f64 = dimensions.iter().map(|d| d.score * d.weight).sum::<f64>() / sum_w;
    let composite = composite_f.round().clamp(0.0, 100.0) as u8;

    AuditScore {
        composite,
        dimensions,
    }
}

fn count_substring(haystack_lower: &str, needle_lower: &str) -> u32 {
    let mut n = 0u32;
    let mut start = 0;
    while let Some(i) = haystack_lower
        .get(start..)
        .and_then(|s| s.find(needle_lower))
    {
        n += 1;
        start += i + needle_lower.len();
    }
    n
}

fn visible_text_word_count(html: &str) -> usize {
    let mut in_tag = false;
    let mut buf = String::new();
    for c in html.chars() {
        match c {
            '<' => in_tag = true,
            '>' => in_tag = false,
            _ if !in_tag => buf.push(c),
            _ => {}
        }
    }
    buf.split_whitespace().filter(|w| !w.is_empty()).count()
}

fn count_imgs_with_alt(html_lower: &str) -> u32 {
    let mut count = 0;
    for seg in html_lower.split("<img") {
        if seg.is_empty() {
            continue;
        }
        let head = seg.split('>').next().unwrap_or(seg);
        if head.contains("alt=") {
            count += 1;
        }
    }
    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_html_scores_low_content() {
        let sc = score_html("");
        assert!(sc.composite < 50);
    }

    #[test]
    fn reasonable_landing_page_scores_mid_high() {
        let html = r#"<!DOCTYPE html><html><head>
            <title>Local Plumber</title>
            <meta name="description" content="24/7 drains">
            <meta name="viewport" content="width=device-width">
            </head><body>
            <h1>Drain cleaning</h1>
            <p>We fix sinks fast with warranty and friendly service in your neighborhood.</p>
            <img src="/a.png" alt="truck">
            </body></html>"#;
        let sc = score_html(html);
        assert!(sc.composite >= 40);
        assert_eq!(sc.dimensions.len(), 7);
    }
}
