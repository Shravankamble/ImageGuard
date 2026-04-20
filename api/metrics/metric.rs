pub mod metric {
    pub async fn metric_middleware(
    State(metrics): State<Arc<Mutex<Metrics>>>,
    req: Request,
    next: Next,
) -> Response<Body> {
    let resp = next.run(req).await;
    let (parts, body) = resp.into_parts();
    let buffer = match to_bytes(body, 3_000_000).await {
        Ok(bytes) => bytes,
        Err(_err) => return StatusCode::BAD_REQUEST.into_response(),
    };

    let data = serde_json::from_slice::<Value>(&buffer).unwrap();

    let original_resp = Response::from_parts(parts, Body::from(buffer));

    let allowed = &data["response"]["allowed"];
    // println!("allowed : {:?}", allowedlowed);

    if allowed == true {
        metrics
            .lock()
            .await
            .counter
            .get_or_create(&Labels {
                method: Method::POST,
                response: "allowed".to_string(),
            })
            .inc();
        return original_resp.into_response();
    } else {
        metrics
            .lock()
            .await
            .counter
            .get_or_create(&Labels {
                method: Method::POST,
                response: "denied".to_string(),
            })
            .inc();
        return original_resp.into_response();
    }
}
}