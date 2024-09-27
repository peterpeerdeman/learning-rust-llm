use anyhow::Result;
use mistralrs::{
    GgufModelBuilder, PagedAttentionMetaBuilder, RequestBuilder, TextMessageRole, TextMessages,
};
use std::path::Path;

#[tokio::main]
async fn main() -> Result<()> {

    let model = GgufModelBuilder::new(
        "llama-2-7b.Q8_0", 
        vec![
            "./llama-2-7b.Q8_0.gguf",
            "./chat_template.json",
        ]
    )
    .with_chat_template(Path::new("./chat_template.json").display().to_string())
    .with_logging()
    .with_paged_attn(|| PagedAttentionMetaBuilder::default().build())?
    .build()
    .await?;

    let messages = TextMessages::new()
        .add_message(
            TextMessageRole::System,
            "You are an AI agent with a specialty in personal knowledge management",
        )
        .add_message(TextMessageRole::User, "please create a template for my daily note");

    let response = model.send_chat_request(messages).await?;

    println!("{}", response.choices[0].message.content.as_ref().unwrap());

    Ok(())
}
