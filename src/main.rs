mod prompt;
mod models;
mod telegram;

use crate::prompt::*;
use crate::models::Toezegging;

use rusqlite::{Connection, Result};

use dotenv::dotenv;
use std::env;

fn retrieve_toezeggingen(conn: Connection) -> Vec<Toezegging> {
    let mut stmt = conn.prepare(
        "SELECT tekst, nummer FROM Toezegging  ORDER BY datum DESC"
    ).expect("err");

    let toezegging_iter = stmt.query_map([], |row| {
        Ok(Toezegging {
            tekst: row.get(0)?,
            nummer: row.get(1)?,
        })
    }).expect("error");

    let mut result = Vec::new();
    for toezegging in toezegging_iter.take(10) {
        let tz = toezegging.unwrap();
        result.push(tz);
    }
    result
}

async fn process_and_send_onderwerpen(
    res: Result<Vec<models::Onderwerp>, Box<dyn std::error::Error>>,
    telegram_bot: &telegram::TelegramBot,
) -> Result<(), Box<dyn std::error::Error>> {
    match res {
        Ok(onderwerpen) => {
            let mut message = String::from("🔍 *Gevonden onderwerpen:*\n\n");
            
            for onderwerp in onderwerpen {
                message.push_str(&format!("📌 <b>{}</b>\n", onderwerp.onderwerp));
                for id in onderwerp.toezegging_ids {
                    message.push_str(&format!("   • {}\n", id));
                }
                message.push('\n');
            }

            telegram_bot.send_message(&message).await?;
            Ok(())
        },
        Err(e) => Err(e)
    }
}


#[tokio::main]
async fn main() -> Result<()> {
    env_logger::init();
    dotenv().ok();

    let conn = Connection::open("/Users/peter/tkconv-data/tk.sqlite3")?;

    let telegram_bot = crate::telegram::TelegramBot::new(
        std::env::var("TELEGRAM_BOT_TOKEN").expect("TELEGRAM_BOT_TOKEN not set"),
        std::env::var("TELEGRAM_CHAT_ID").expect("TELEGRAM_CHAT_ID not set")
    );

    let toezeggingen = retrieve_toezeggingen(conn);
    let prompt = build_prompt(toezeggingen);

    let res = ai_translate_toezeggingen_to_onderwerpen(prompt).await;

    if let Err(e) = process_and_send_onderwerpen(res, &telegram_bot).await {
        eprintln!("Error processing onderwerpen: {}", e);
    }

    Ok(())
}
