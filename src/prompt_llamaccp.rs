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