use reqwest;
use serde_json::json;

pub struct TelegramBot {
    bot_token: String,
    chat_id: String,
}

impl TelegramBot {
    pub fn new(bot_token: String, chat_id: String) -> Self {
        TelegramBot {
            bot_token,
            chat_id,
        }
    }

    pub async fn send_message(&self, message: &str) -> Result<(), Box<dyn std::error::Error>> {
        let url = format!(
            "https://api.telegram.org/bot{}/sendMessage",
            self.bot_token
        );

        let client = reqwest::Client::new();
        let response = client
            .post(&url)
            .json(&json!({
                "chat_id": self.chat_id,
                "text": message,
                "parse_mode": "HTML"
            }))
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(format!("Failed to send message: {}", response.status()).into());
        }

        Ok(())
    }
}
