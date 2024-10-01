use axum::extract::State;
use axum::{routing::post, Json, Router};
use ollama_rs::generation::completion::request::GenerationRequest;
use ollama_rs::Ollama;
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;
use std::sync::Arc;

#[tokio::main]

async fn main() {
    println!("Starting Ollama Rust Server...");

    let ollama = Arc::new(Ollama::default()); // Arc thread, tread safe shared state

    let app = Router::new().route("/prompt", post(handle_prompt).with_state(ollama.clone()));

    let addr = SocketAddr::from(([127, 0, 0, 1], 14434));
    println!("Listening for Prompts on http://{}/prompt", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

#[derive(Deserialize)]
struct PromptRequest {
    prompt: String,
}

#[derive(Serialize)]
struct PromptResponse {
    response: String,
}

async fn handle_prompt(
    State(ollama): State<Arc<Ollama>>,
    Json(payload): Json<PromptRequest>,
) -> Json<PromptResponse> {
    let response = prompt(payload.prompt, ollama.clone()).await;

    Json(PromptResponse { response })
}

async fn prompt(prompt: String, ollama: Arc<Ollama>) -> String {
    let model = "llama3.1:latest".to_string();

    let res = ollama.generate(GenerationRequest::new(model, prompt)).await;

    match res {
        Ok(res) => res.response,
        Err(_) => "Failed to generate response.".to_string(),
    }
}
