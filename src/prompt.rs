use crate::toezegging::Toezegging;

use genai::chat::printer::PrintChatStreamOptions;
use genai::chat::{ChatMessage, ChatRequest};
use genai::Client;

use llama_cpp::standard_sampler::StandardSampler;
use llama_cpp::{LlamaModel, LlamaParams, SessionParams};

pub fn prompt_llama_cpp(toezeggingen: Vec<Toezegging>) {

    let model = LlamaModel::load_from_file(
        "/Users/peter/llms/llama-2-7b.Q8_0.gguf",
        LlamaParams::default(),
    )
    .expect("Couldn't load file");

    // A `LlamaModel` holds the weights shared across many _sessions_; while your model may be
    // several gigabytes large, a session is typically a few dozen to a hundred megabytes!
    let mut ctx = model
        .create_session(SessionParams::default())
        .expect("Failed to create session");

    let prompt = "the sky is";

    ctx.advance_context(prompt)
        .unwrap();

    // LLMs are typically used to predict the next word in a sequence. Let's generate some tokens!
    let max_tokens = 1024;
    let mut decoded_tokens = 0;

    // `ctx.start_completing_with` creates a worker thread that generates tokens. When the completion
    // handle is dropped, tokens stop generating!

    let completions = ctx
        .start_completing_with(StandardSampler::default(), 1024)
        .expect("error")
        .into_strings();

    for completion in completions {
        print!("{completion}");
        //let _ = io::stdout().flush();

        decoded_tokens += 1;

        if decoded_tokens > max_tokens {
            break;
        }
    }
}
    
pub fn build_prompt(toezeggingen: Vec<Toezegging>) -> String {
    // You can feed anything that implements `AsRef<[u8]>` into the model's context.
    let prompt_context = "spreek alleen nederlands. Je bent een expert in het categoriseren van toezeggingen, bekijk de volgende toezeggingen, verzin bijpassende categorieen, en maak per categorie een lijst met toezeggingen";

    let concatenated_toezeggingen = toezeggingen
        .iter()
        .take(5)
        .map(|toezegging| toezegging.tekst.clone()).collect::<Vec<String>>()
        .join("\n");

    let prompt_affix = "document met gecategoriseerde toezeggingen: ";

    let prompt = vec![
        prompt_context,
        &concatenated_toezeggingen,
        prompt_affix
    ].join("\n");

    //println!("prompt length: {}, prompt {} ", &prompt.len(), &prompt);
    prompt
}

pub async fn print_completions(prompt: String) -> Result<(), Box<dyn std::error::Error>> {
    const MODEL_OLLAMA: &str = "llama3.2:latest";

    let chat_req = ChatRequest::new(vec![
        ChatMessage::system("spreek alleen nederlands"),
        ChatMessage::user(prompt),
    ]);

    let client = Client::default();
    //let print_options = PrintChatStreamOptions::from_print_events(false);

    let adapter_kind = client.resolve_model_iden(MODEL_OLLAMA)?.adapter_kind;
    println!("\n===== MODEL: ({adapter_kind}) =====");

    let chat_res = client.exec_chat(MODEL_OLLAMA, chat_req.clone(), None).await?;

    println!("{}", chat_res.content_text_as_str().unwrap_or("NO ANSWER"));

    Ok(())
}
