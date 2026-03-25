#![allow(dead_code)]

pub mod client;
mod endpoints;
pub mod error;
pub mod models;

pub use client::{OnspringClient, OnspringClientBuilder};
pub use error::{OnspringError, Result};
pub use models::*;
