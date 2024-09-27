use llm_chain::executor;
use llm_chain::{parameters, prompt};
use llm_chain::options::*;
use llm_chain::options;

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let opts = options!(
        Model: ModelRef::from_path("/Users/peter/llms/llama-2-7b.Q8_0.gguf"), // Notice that we reference the model binary path
        ModelType: "llama",
        MaxContextSize: 512_usize,
        NThreads: 4_usize,
        MaxTokens: 0_usize,
        TopK: 40_i32,
        TopP: 0.95,
        TfsZ: 1.0,
        TypicalP: 1.0,
        Temperature: 0.8,
        RepeatPenalty: 1.1,
        RepeatPenaltyLastN: 64_usize,
        FrequencyPenalty: 0.0,
        PresencePenalty: 0.0,
        Mirostat: 0_i32,
        MirostatTau: 5.0,
        MirostatEta: 0.1,
        PenalizeNl: true,
        StopSequence: vec!["\n".to_string()]
    );

    let exec = executor!(llama, opts)?;
    let res = prompt!("i rust because")
        .run(
            &parameters!(),
            &exec,
        )
        .await?;
    println!("{}", res.to_immediate().await?);
    Ok(())
}
