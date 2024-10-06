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

    // Initialize the Ollama instance with default settings
    // Wrap it in an Arc to allow shared ownership across threads
    let ollama = Arc::new(Ollama::default());

    // Create the application state by encapsulating the Ollama instance
    // Using Arc for thread-safe shared access
    let app_state = Arc::new(AppState {
        ollama: ollama.clone(),
    });

    // Build the Axum router
    // - Define a POST route at "/prompt" that uses the `handle_prompt` handler
    // - Attach the shared application state to the router
    let app = Router::new()
        .route("/prompt", post(handle_prompt))
        .with_state(app_state.clone());

    // Define the socket address where the server will listen for incoming requests
    let addr = SocketAddr::from(([127, 0, 0, 1], 14434));
    // Print a message indicating where the server is listening
    println!("Listening for Prompts on http://{}/prompt", addr);

    // Start the Axum server
    // - Bind to the specified address
    // - Serve the router as a make_service
    // - Await the server to run indefinitely, handling incoming requests
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap(); // Panic if the server fails to start
}

struct AppState {
    ollama: Arc<Ollama>, // Shared Ollama instance for handling requests
}

#[derive(Deserialize)]
struct PromptRequest {
    prompt: String, // The user's prompt as a string
}

#[derive(Serialize)]
struct PromptResponse {
    response: String, // The generated response from Ollama
}

// Handler function for the "/prompt" route
// - Extracts the shared application state
// - Parses the incoming JSON payload into a PromptRequest
// - Calls the `prompt` function to generate a response
// - Returns the response wrapped in JSON
async fn handle_prompt(
    State(state): State<Arc<AppState>>, // Extract shared state
    Json(payload): Json<PromptRequest>, // Extract and parse JSON payload
) -> Json<PromptResponse> {
    // Call the `prompt` function with the user's prompt and the shared Ollama instance
    let response = prompt(payload.prompt, state.ollama.clone()).await;

    // Return the generated response as a JSON payload
    Json(PromptResponse { response })
}

// Asynchronous function to generate a response based on the user's prompt
// - Accepts the user's prompt and a shared Ollama instance
// - Constructs a system prompt by embedding the user's prompt
// - Sends the prompt to Ollama for generation
// - Returns the generated response or an error message
async fn prompt(prompt: String, ollama: Arc<Ollama>) -> String {
    // Specify the model to use for generation
    let model = "llama3.1:latest".to_string(); // Modify this to use a different model if desired

    // Create a system prompt by formatting the user's prompt within a predefined template
    let sysprompt = format!(
        "
        (SYSTEM)
        SYSTEM PROMPT

        (USER)
        Here's a prompt you need to answer:
        {}
        ",
        prompt
    );

    // Send the generation request to Ollama asynchronously
    let res = ollama
        .generate(GenerationRequest::new(model, sysprompt))
        .await;

    // Handle the response from Ollama
    match res {
        Ok(res) => res.response, // Return the generated response on success
        Err(_) => "Failed to generate response.".to_string(), // Return an error message on failure
    }
}