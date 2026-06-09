pub mod utils {
    use crate::cache::cache::ImageCache;
    use crate::schema;
    use axum::{
        Json,
        body::Body,
        extract::{Request, State},
        http::Response,
        middleware::Next,
        response::IntoResponse,
    };
    use futures::lock::Mutex;
    use log::{info, warn};
    use prometheus_client::encoding::text::encode;
    use schema::configs::{
        AdmissionResponse, AdmissionReview, Decider, Labels, Method, Metrics, Register, Status,
    };
    // use serde::{Deserialize, Serialize};
    use serde_json::Value;
    use std::sync::Arc;
    use tokio::sync::RwLock;
    // use tower::{ServiceBuilder, buffer};

    pub async fn validate(
        State(cache): State<Arc<RwLock<ImageCache>>>,
        payload: Json<Value>,
    ) -> impl IntoResponse {
        let resp = |allow: bool, uid: Value, code: i16, message: String| AdmissionReview {
            apiVersion: "admission.k8s.io/v1".to_string(),
            kind: "AdmissionReview".to_string(),
            response: AdmissionResponse {
                uid: (uid),
                allowed: (allow),
                status: Status {
                    code: (code),
                    message: (message),
                },
            },
        };

        /* admission review request image field stored as
           Some(String("image_name"))
        */

        let path = "/request/object/spec/containers/0/image";

        let image = payload
            .pointer(path)
            .and_then(|s| s.as_str())
            .unwrap_or_else(|| "default");

        let uid = payload["request"]["uid"].clone();

        log::info!("uid : {}, image name : {}", &uid, &image);

        let image_value: Option<String> = cache.read().await.cache.get(image).await;

        if image_value.is_some() {
            warn!(
                "Invalid request by {}",
                &payload["request"]["userInfo"]["username"]
            );
            Response::builder()
                .header("Content-Type", "application/json")
                .extension(Decider { state: false })
                .body(Body::from(
                    serde_json::to_vec(&resp(false, uid, 403, "Invalid request".to_string()))
                        .unwrap(),
                ))
                .unwrap()
        } else {
            info!("valid request");
            Response::builder()
                .header("Content-Type", "application/json")
                .extension(Decider { state: true })
                .body(Body::from(
                    serde_json::to_vec(&resp(true, uid, 200, "valid Request!".to_string()))
                        .unwrap(),
                ))
                .unwrap()
        }
    }

    // middleware for pormetheus metrics which parses the outgoing response and increments counter based on the response
    pub async fn metric_middleware(
        State(metrics): State<Arc<Mutex<Metrics>>>,
        req: Request,
        next: Next,
    ) -> Response<Body> {
        let resp = next.run(req).await;

        let result = match &resp.extensions().get::<Decider>() {
            Some(val) if val.state == true => true,
            Some(_) => false,
            None => false,
        };

        if result == true {
            metrics
                .lock()
                .await
                .counter
                .get_or_create(&Labels {
                    method: Method::POST,
                    response: "allowed".to_string(),
                })
                .inc();
            return resp;
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
            return resp;
        }
    }

    pub async fn metrics_handler(State(metrics): State<Arc<Mutex<Register>>>) -> impl IntoResponse {
        let state = metrics.lock().await;
        let mut buffer = String::new();
        encode(&mut buffer, &state.registry).unwrap();
        buffer
    }

    // pub async fn _schema_validation_middleware(req: Request, next: Next) -> impl IntoResponse {
    //     let (parts, body) = req.into_parts();
    //     let buffer = match to_bytes(body, 5_000_000).await {
    //         Ok(bytes) => bytes,
    //         Err(_err) => return StatusCode::BAD_REQUEST.into_response(),
    //     };

    //     let data = serde_json::from_slice::<Value>(&buffer).unwrap();

    //     let uid = &data["request"]["uid"];
    //     println!("uid is : {}", uid);

    //     let constructed_request = Request::from_parts(parts, Body::from(buffer));
    //     let resp = next.run(constructed_request).await;
    //     resp
    // }
}
