use axum::response::Html;

pub async fn test() -> Html<String> {
    let html = std::fs::read_to_string("test.html").unwrap();
    Html(html)
}
