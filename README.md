# OllamaServe 🦙🛜

OllamaServe is an open-source HTTP server built with Rust and Axum, designed to integrate with the Ollama AI engine. It allows users to send prompts via HTTP POST requests and receive AI-generated responses. The server leverages asynchronous processing for high performance and supports thread-safe state sharing using Rust's `Arc` for concurrent handling.

## Table of Contents
1. [Installation](#installation)
2. [Usage](#usage)
3. [License](#license)

## Installation

1. **Clone repo:**
    ```bash
    git clone https://github.com/vovakalinins/ollamaserve.git
    ```

2. **Cd in the directory:**
    ```bash
    cd server
    ```

3. **Install dependencies:**
    Ensure you have Rust installed. If not, install it from [here](https://www.rust-lang.org/tools/install).

    Then run:
    ```bash
    cargo build
    ```

4. **Run the server:**
    ```bash
    cargo run
    ```

    The server will start on `http://127.0.0.1:14434`. You can change it in the program, that's just the Ollama Default.

## Usage

Once the server is running, you can send HTTP POST requests to the `/prompt` endpoint. The payload should be a JSON object containing the `prompt` string field.

### Example Request (using `curl`):

```bash
curl -X POST http://127.0.0.1:14434/prompt \
     -H "Content-Type: application/json" \
     -d '{"prompt": "What is up bro?"}'
```

### Example Response:
```bash
{
  "response": "Hello, How Can I Assist You..."
}
```

### License
This project is licensed under the MIT License. See the LICENSE file for details.
Feel free to adjust the project name and update the repository URL with your actual GitHub account details when publishing this project. Let me know if you need further modifications!
