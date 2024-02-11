pub(crate) mod response;

use axum::http::StatusCode;
use axum::http::Uri;
use axum::response::Html;
use axum::response::IntoResponse;
use axum::response::Response;
use axum::Json;
use serde_json::json;

#[tracing::instrument(level = "debug", ret)]
pub(crate) async fn not_found_api_handler(uri: Uri) -> Response {
    let full_uri = uri.to_string();
    let uri = full_uri
        .split_once("/api")
        .unwrap_or(("", full_uri.as_str()))
        .1;

    (
        StatusCode::NOT_FOUND,
        Json(json!({
            "http_response": {
                "code": 404,
                "message": "We could not find the resource you requested."
            },
            "code": 4,
            "description": "",
            "details": {},
            "message": format!("Relative path ({uri}) is not a known endpoint resource. Please refer to documentation for list of endpoint resources.")
        })),
    ).into_response()
}

#[tracing::instrument(level = "debug", ret)]
pub(crate) async fn not_found_handler() -> Response {
    (
        StatusCode::OK,
        Html(
            r#"


            <html>
                <head>
                    <title>System Exception</title>
                    <style type="text/css">
                        body {
                            font-family: "IBM Helvetica Neue", "Helvetica Neue", Helvetica, Arial, Roboto, sans-serif;
                            margin: 0;
                            height: 100%;
                            width: 100%;
                            display: table;
                            text-align: center;
                            font-size: 1rem;
                            line-height: 1.5;
                        }
                        div {
                            vertical-align: middle;
                            display: table-cell;
                        }
                        p {
                            word-break: break-word;
                        }
                        .warningIcon{
                            font-size: 6rem;
                            color: #dc0000;
                        }
                        .sy-h1 {
                            font-size: 2rem;
                            margin-top: -2rem;
                            margin-bottom: 1.875rem;
                            line-height: 1.11111;
                            letter-spacing: .5px;
                            -webkit-font-smoothing: antialiased;
                            -moz-osx-font-smoothing: grayscale;
                            font-weight: 600;
                        }
                    </style>
                </head>
                <body>
                    <div>
                        <span class="warningIcon">&#x26a0;</span>
                        <h1 class="sy-h1">Application error</h1>
                        <p> An error has occurred. Return and attempt the action again.</br><b>If the problem persists, please contact customer support for assistance.</b></p>
                    </div>
                </body>
            </html>"#,
        ),
    ).into_response()
}

#[tracing::instrument(level = "debug", ret)]
pub(crate) async fn global_not_found_handler(uri: Uri) -> Response {
    // I hate this solution. Didn't find anything better.
    if uri.path().starts_with("/api/") {
        not_found_api_handler(uri).await
    } else {
        not_found_handler().await
    }
}
