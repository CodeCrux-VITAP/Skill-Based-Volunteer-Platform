use axum::{
    Router,
    routing::*
};
use crate::handler;
use surrealdb::engine::any::Any;
use surrealdb::Surreal;
use std::sync::Arc;
pub fn app(db:Arc<Surreal<Any>>)->Router
{
    Router::new().route("/check",post(handler::check))
    .route("/otp",post(handler::Otps))
    .route("/verify",post(handler::Verify))
    .route("/deleteotp",post(handler::Deleteotp))
    .route("/updateotp",post(handler::Updateotp))
    .route("/signin",post(handler::Signin))
    .with_state(db.clone())
}