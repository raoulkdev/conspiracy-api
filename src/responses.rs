use std::fmt::Debug;
use std::fs;
use axum::extract::{Path, Query};
use axum::http::{StatusCode};
use axum::Json;
use axum::response::IntoResponse;
use serde::{Deserialize, Serialize};
use serde_json::json;

pub async fn all_theories() -> impl IntoResponse {
    let conspiracies_json = fs::read("src/conspiracies.json");
    match conspiracies_json { 
        Ok(conspiracies) => (StatusCode::OK, conspiracies).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, Json(json!(format!("{e}")))).into_response()
    }
}

pub async fn theory_by_id(Path(id): Path<u32>) -> impl IntoResponse {
    let conspiracies_json = fs::read_to_string("src/conspiracies.json").unwrap();
    let theories: Vec<Theory> = serde_json::from_str(&conspiracies_json).unwrap();
    
    let theory = theories.iter().find(|t| t.id == id);
    
    match theory { 
        Some(t) => (StatusCode::OK, Json(t)).into_response(),
        None => (StatusCode::NOT_FOUND, Json(json!({ "error" : "theory not found"}))).into_response()
    }
}

pub async fn theories_by_category(Query(params): Query<CategoryQuery>) -> impl IntoResponse {
    let conspiracies_json = fs::read_to_string("src/conspiracies.json").unwrap();
    let conspiracies: Vec<Theory> = serde_json::from_str(&conspiracies_json).unwrap();
    let filtered: Vec<Theory> = conspiracies.into_iter().filter(|t| t.category == params.category).collect();
    
    if !filtered.is_empty() {
        (StatusCode::OK, Json(filtered)).into_response()
    } else {
        (StatusCode::NOT_FOUND, Json(json!({ "error" : "Could not find category!"}))).into_response()
    }
}

pub async fn all_categories() -> impl IntoResponse {
    let categories_json = fs::read("src/categories.json");
    match categories_json {
        Ok(categories) => (StatusCode::OK, categories).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, Json(json!(format!("{e}")))).into_response()
    }
}

#[derive(Deserialize)]
pub struct CategoryQuery {
    category: String
}

#[derive(Serialize,Deserialize ,Debug)]
pub struct Theory {
    id: u32,
    title: String,
    category: String,
    description: String
}