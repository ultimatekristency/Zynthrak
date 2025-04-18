use actix_web::{
    web::{self, Data, Json, Path, ServiceConfig},
    App, HttpRequest, HttpResponse, Responder,
};
use serde::{Deserialize, Serialize};
use shuttle_actix_web::ShuttleActixWeb;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

#[derive(Clone)]
struct AppState {
    nodes: Arc<Mutex<HashMap<String, Node>>>,
}

#[derive(Serialize, Deserialize, Clone)]
struct Node {
    id: String,
    ip: String,
    status: String,
}

async fn register_node(
    new_node: Json<Node>,
    state: Data<AppState>,
    _req: HttpRequest,
) -> impl Responder {
    let mut nodes = state.nodes.lock().unwrap();
    nodes.insert(new_node.id.clone(), new_node.into_inner());

    HttpResponse::Ok().body("Node registered")
}

async fn get_nodes(state: Data<AppState>, _req: HttpRequest) -> impl Responder {
    let nodes = state.nodes.lock().unwrap();
    let node_list: Vec<_> = nodes.values().cloned().collect();
    HttpResponse::Ok().json(node_list)
}

async fn update_node(
    path: Path<String>,
    updated_node: Json<Node>,
    state: Data<AppState>,
    _req: HttpRequest,
) -> impl Responder {
    let mut nodes = state.nodes.lock().unwrap();
    let id = path.into_inner();
    if nodes.contains_key(&id) {
        nodes.insert(id.clone(), updated_node.into_inner());
        HttpResponse::Ok().body("Node updated")
    } else {
        HttpResponse::NotFound().body("Node not found")
    }
}

async fn remove_node(
    path: Path<String>,
    state: Data<AppState>,
    _req: HttpRequest,
) -> impl Responder {
    let mut nodes = state.nodes.lock().unwrap();
    let id = path.into_inner();
    if nodes.remove(&id).is_some() {
        HttpResponse::Ok().body("Node removed")
    } else {
        HttpResponse::NotFound().body("Node not found")
    }
}

async fn block_node(
    path: Path<String>,
    state: Data<AppState>,
    _req: HttpRequest,
) -> impl Responder {
    let mut nodes = state.nodes.lock().unwrap();
    let id = path.into_inner();
    if let Some(node) = nodes.get_mut(&id) {
        node.status = "blocked".into();
        HttpResponse::Ok().body("Node blocked")
    } else {
        HttpResponse::NotFound().body("Node not found")
    }
}

#[shuttle_runtime::main]
async fn main() -> ShuttleActixWeb<impl FnOnce(&mut ServiceConfig) + Send + Clone + 'static> {
    let state = AppState {
        nodes: Arc::new(Mutex::new(HashMap::new())),
    };

    let state_data = Data::new(state);

    let config = move |cfg: &mut ServiceConfig| {
        cfg.app_data(state_data.clone())
            .route("/api/v1/register", web::post().to(register_node))
            .route("/api/v1/nodes", web::get().to(get_nodes))
            .route("/api/v1/nodes/{id}", web::put().to(update_node))
            .route("/api/v1/nodes/{id}", web::delete().to(remove_node))
            .route("/api/v1/nodes/{id}/block", web::put().to(block_node));
    };

    Ok(config.into())
}
