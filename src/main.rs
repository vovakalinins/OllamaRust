use ollama_rs::generation::completion::request::GenerationRequest;
use ollama_rs::Ollama;

#[tokio::main]
async fn main() {
    println!("Starting server on http://localhost:11434");
    let ollama = Ollama::default();

    println!("Server should be running");

    prompt("Hello How Are You?".to_string(), ollama).await;
}

async fn prompt(prompt: String, ollama: Ollama) {
    let model = "llama3.1:latest".to_string();

    let res = ollama.generate(GenerationRequest::new(model, prompt)).await;

    if let Ok(res) = res {
        println!("{}", res.response);
    } else {
        eprintln!("Failed to generate response.");
    }
}
