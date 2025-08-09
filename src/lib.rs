// Survey library - Expose modules for reuse

pub mod controllers;
pub mod database;
pub mod dto;
pub mod errors;
pub mod middleware;
pub mod models;
pub mod repository;
pub mod routes;
pub mod services;
pub mod utils;

use sqlx::PgPool;

#[derive(Clone)]
pub struct AppState {
    pub db: PgPool,
}
