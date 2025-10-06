/// http rest api server, used to operate and maintain internal channels, groups, and topics
use axum::{
    Json, Router,
    http::{StatusCode, header},
    response::IntoResponse,
    routing::{get, post},
};
use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::config::HTTP_LISTENER_PORT;

/// order book http service, HTTP service for handling order books
pub struct LuminMQHtppService;
impl LuminMQHtppService {
    pub async fn enable() {
        // build our application with a route
        let app = Router::new()
            // groups related operations
            .route(
                "/groups/getGroupList",
                post(move |body| Self::groups_get_group_list(body)),
            )
            // group related operations
            .route("/group/create", post(move |body| Self::group_create(body)))
            .route("/group/remove", post(move |body| Self::group_remove(body)))
            .route(
                "/group/setMode",
                post(move |body| Self::group_set_mode(body)),
            )
            .route(
                "/group/getInfoById",
                post(move |body| Self::group_get_info_by_id(body)),
            )
            .route(
                "/group/getTopicListByGroupId",
                post(move |body| Self::group_get_topic_list_by_group_id(body)),
            )
            .route(
                "/group/getMessageNum",
                post(move |body| Self::group_get_message_num(body)),
            )
            // topic related operations
            .route("/topic/create", get(move |path| Self::topic_create(path)))
            .route(
                "/topic/setMode",
                get(move |path| Self::topic_set_mode(path)),
            )
            .route(
                "/topic/getInfoById",
                get(move |path| Self::topic_get_info_by_id(path)),
            )
            .route(
                "/topic/getMessageNum",
                post(move |body| Self::topic_get_message_num(body)),
            );
        let addr = HTTP_LISTENER_PORT.lock().unwrap().clone();
        let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
        axum::serve(listener, app).await.unwrap();
    }

    async fn groups_get_group_list(Json(vo): Json<GroupsVO>) -> impl IntoResponse {
        (
            StatusCode::OK,
            [(header::CONTENT_TYPE, "application/json")],
            Json(json!({ "message": "successfully"})),
        )
    }
    async fn group_create(Json(vo): Json<GroupVO>) -> impl IntoResponse {
        (
            StatusCode::OK,
            [(header::CONTENT_TYPE, "application/json")],
            Json(json!({ "message": "successfully"})),
        )
    }
    async fn group_remove(Json(vo): Json<GroupVO>) -> impl IntoResponse {
        (
            StatusCode::OK,
            [(header::CONTENT_TYPE, "application/json")],
            Json(json!({ "message": "successfully"})),
        )
    }
    async fn group_set_mode(Json(vo): Json<GroupVO>) -> impl IntoResponse {
        (
            StatusCode::OK,
            [(header::CONTENT_TYPE, "application/json")],
            Json(json!({ "message": "successfully"})),
        )
    }
    async fn group_get_info_by_id(Json(vo): Json<GroupVO>) -> impl IntoResponse {
        (
            StatusCode::OK,
            [(header::CONTENT_TYPE, "application/json")],
            Json(json!({ "message": "successfully"})),
        )
    }
    async fn group_get_topic_list_by_group_id(Json(vo): Json<GroupVO>) -> impl IntoResponse {
        (
            StatusCode::OK,
            [(header::CONTENT_TYPE, "application/json")],
            Json(json!({ "message": "successfully"})),
        )
    }
    async fn group_get_message_num(Json(vo): Json<GroupVO>) -> impl IntoResponse {
        (
            StatusCode::OK,
            [(header::CONTENT_TYPE, "application/json")],
            Json(json!({ "message": "successfully"})),
        )
    }
    async fn topic_create(Json(vo): Json<TopicVO>) -> impl IntoResponse {
        (
            StatusCode::OK,
            [(header::CONTENT_TYPE, "application/json")],
            Json(json!({ "message": "successfully"})),
        )
    }
    async fn topic_set_mode(Json(vo): Json<TopicVO>) -> impl IntoResponse {
        (
            StatusCode::OK,
            [(header::CONTENT_TYPE, "application/json")],
            Json(json!({ "message": "successfully"})),
        )
    }
    async fn topic_get_info_by_id(Json(vo): Json<TopicVO>) -> impl IntoResponse {
        (
            StatusCode::OK,
            [(header::CONTENT_TYPE, "application/json")],
            Json(json!({ "message": "successfully"})),
        )
    }
    async fn topic_get_message_num(Json(vo): Json<TopicVO>) -> impl IntoResponse {
        (
            StatusCode::OK,
            [(header::CONTENT_TYPE, "application/json")],
            Json(json!({ "message": "successfully"})),
        )
    }
}

// create order book view object
#[derive(Deserialize, Serialize, Debug, Clone)]
struct GroupsVO {
    pub symbol: String,
}
impl GroupsVO {}

// order view object
#[derive(Deserialize, Serialize, Debug, Clone)]
struct GroupVO {
    pub symbol: String,
    pub price: f64,
}

impl GroupVO {}

// order view object
#[derive(Deserialize, Serialize, Debug, Clone)]
struct TopicVO {
    pub symbol: String,
    pub price: f64,
}

impl TopicVO {}
