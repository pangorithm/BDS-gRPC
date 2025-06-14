pub mod proto {
    tonic::include_proto!("packet");
}

// service.rs 모듈을 가져옴
pub mod repository;
pub mod service;
