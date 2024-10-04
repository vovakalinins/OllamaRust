use axum::extract::State;
use axum::{routing::post, Json, Router};
use ollama_rs::generation::completion::request::GenerationRequest;
use ollama_rs::generation::options::GenerationOptions;
use ollama_rs::Ollama;
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;
use std::sync::Arc;

#[tokio::main]
async fn main() {
    println!("Starting Ollama Rust Server...");

    // Initialize Ollama and wrap it in Arc for shared ownership
    let ollama = Arc::new(Ollama::default());

    // Define generation options
    let options = GenerationOptions::default()
        .temperature(0.2) // Low temperature for deterministic answers
        .repeat_penalty(1.3) // Moderate penalty to avoid repetition
        .top_k(30) // Slightly higher to allow some flexibility
        .top_p(0.3); // Balanced nucleus sampling for relevance

    // Combine shared states into a single AppState struct
    let app_state = Arc::new(AppState {
        ollama: ollama.clone(),
        options,
    });

    // Build the Axum application with the combined state
    let app = Router::new()
        .route("/prompt", post(handle_prompt))
        .with_state(app_state.clone());

    let addr = SocketAddr::from(([127, 0, 0, 1], 14434));
    println!("Listening for Prompts on http://{}/prompt", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

// Define a struct to hold all shared state
struct AppState {
    ollama: Arc<Ollama>,
    options: GenerationOptions,
}

#[derive(Deserialize)]
struct PromptRequest {
    prompt: String,
}

#[derive(Serialize)]
struct PromptResponse {
    response: String,
}

// Update the handler to extract the combined AppState
async fn handle_prompt(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<PromptRequest>,
) -> Json<PromptResponse> {
    let response = prompt(payload.prompt, state.ollama.clone(), &state.options).await;

    Json(PromptResponse { response })
}

// Update the prompt function to accept a reference to GenerationOptions
async fn prompt(prompt: String, ollama: Arc<Ollama>, options: &GenerationOptions) -> String {
    let model = "closex/neuraldaredevil-8b-abliterated".to_string(); // Change the Model based on your liking

    let res = ollama
        .generate(GenerationRequest::new(model, prompt).options(options.clone()))
        .await;

    match res {
        Ok(res) => res.response,
        Err(_) => "Failed to generate response.".to_string(),
    }
}
