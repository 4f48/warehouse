use axum::extract::Multipart;

pub(crate) async fn upload(mut multipart: Multipart) {
    // parse multipart using serde
    
    while let Some(field) = multipart.next_field().await.unwrap() {
        let bytes = field.bytes().await.unwrap();
        println!("{:?}", bytes);
    }
    
}
