pub mod configs {
    use prometheus_client::{
        encoding::{EncodeLabelSet, EncodeLabelValue},
        metrics::{counter::Counter, family::Family},
        registry::Registry,
    };
    use serde::Serialize;
    use serde_json::Value;
    #[allow(non_camel_case_types)]
    #[derive(Debug, Serialize)]
    pub struct Status {
        pub code: i16,
        pub message: String,
    }

    #[allow(non_camel_case_types)]
    #[derive(Debug, Serialize)]
    pub struct AdmissionResponse {
        pub uid: Value,
        pub allowed: bool,
        pub status: Status,
    }

    #[derive(Debug, PartialEq, Eq, Hash, EncodeLabelSet, Clone)]
    pub struct Labels {
        pub method: Method,
        pub response: String,
    }

    #[derive(Debug, PartialEq, Eq, Hash, EncodeLabelValue, Clone)]
    pub enum Method {
        GET,
        POST,
    }

    #[allow(non_snake_case)]
    #[derive(Debug, Serialize)]
    pub struct AdmissionReview {
        pub apiVersion: String,
        pub kind: String,
        pub response: AdmissionResponse,
    }

    #[derive(Debug, Clone)]
    pub struct Decider {
        pub state: bool,
    }

    #[derive(Debug, Clone)]
    pub struct Metrics {
        pub counter: Family<Labels, Counter>,
    }

    #[derive(Debug)]
    pub struct Register {
        pub registry: Registry,
    }
}
