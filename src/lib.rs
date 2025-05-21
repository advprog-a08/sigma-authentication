mod proto {
    tonic::include_proto!("authentication");
}

pub mod app;
pub mod database;

pub mod admin;
pub mod table_session;
pub mod token;
