use crate::models::{Toezegging, Onderwerp};

use genai::chat::{ChatMessage, ChatRequest};
use genai::Client;
use log::info;

// pub mod prompt_llamacpp;
    
pub fn build_prompt(toezeggingen: Vec<Toezegging>) -> String {
    // You can feed anything that implements `AsRef<[u8]>` into the model's context.
    let prompt_context = r#"spreek alleen nederlands. Je bent een expert in het herkennen van onderwerpen. Bekijk de volgende toezeggingen, (geformatteerd als volgt pub struct TOEZEGGING_ID: TOEZEGGING_TEKST) bepaal per toezegging het onderwerp in maximaal 5 woorden. Formatteer het antwoord in een json array geformatteerd als volgt: [{"onderwerp":"onderwerp","toezegging_ids":["id1","id2"]}]"#;

    let concatenated_toezeggingen = toezeggingen
        .iter()
        .map(|toezegging| format!("{}: {}", toezegging.nummer, toezegging.tekst.clone()))
        .collect::<Vec<String>>()
        .join("\n");

    info!("concatenated {}:", concatenated_toezeggingen);
    let prompt_affix = "document met gecategoriseerde toezeggingen: ";

    let prompt = [prompt_context,
        &concatenated_toezeggingen,
        prompt_affix].join("\n");

    //info!("prompt length: {}, prompt {} ", &prompt.len(), &prompt);
    prompt
}

async fn retry_chat_execution(
    client: &Client,
    model: &str,
    chat_req: ChatRequest,
) -> Result<Vec<Onderwerp>, Box<dyn std::error::Error>> {
    let max_retries = 5;
    let mut attempt = 0;
    
    loop {
        attempt += 1;
        
        match async {
            let chat_res = client.exec_chat(model, chat_req.clone(), None).await?;
            info!("{}", chat_res.content_text_as_str().unwrap_or("NO ANSWER"));
            
            let json_str = chat_res.content_text_as_str().unwrap_or("[]");
            let onderwerpen: Vec<Onderwerp> = serde_json::from_str(json_str)?;
            Ok(onderwerpen)
        }.await {
            Ok(result) => return Ok(result),
            Err(e) => {
                if attempt >= max_retries {
                    return Err(e);
                }
                info!("Attempt {} failed, retrying...", attempt);
            }
        }
    }
}

pub async fn ai_translate_toezeggingen_to_onderwerpen(prompt: String) -> Result<Vec<Onderwerp>, Box<dyn std::error::Error>> {
    const MODEL_OLLAMA: &str = "llama3.2:latest";

    let chat_req = ChatRequest::new(vec![
        ChatMessage::system("spreek alleen nederlands"),
        ChatMessage::user(prompt),
    ]);

    let client = Client::default();
    //let print_options = PrintChatStreamOptions::from_print_events(false);

    let adapter_kind = client.resolve_model_iden(MODEL_OLLAMA)?.adapter_kind;
    info!("\n===== MODEL: ({adapter_kind}) =====");

    let onderwerpen = retry_chat_execution(&client, MODEL_OLLAMA, chat_req.clone()).await?;
    info!("Parsed onderwerpen: {:?}", onderwerpen);

    Ok(onderwerpen)
}
