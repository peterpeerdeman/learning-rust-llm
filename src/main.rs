mod prompt;
mod toezegging;

use crate::prompt::*;
use crate::toezegging::Toezegging;

use rusqlite::{Connection, Result};

fn retrieve_toezeggingen(conn: Connection) -> Vec<Toezegging> {
    let mut stmt = conn.prepare(
        "SELECT tekst, datum FROM Toezegging  ORDER BY datum DESC"
    ).expect("err");

    let toezegging_iter = stmt.query_map([], |row| {
        Ok(Toezegging {
            tekst: row.get(0)?,
            datum: row.get(1)?,
        })
    }).expect("error");

    let mut result = Vec::new();
    for toezegging in toezegging_iter.take(7) {
        let tz = toezegging.unwrap();
        println!("toezegging {}: {}", tz.datum, tz.tekst);
        result.push(tz);
    }
    result
}

#[tokio::main]
async fn main() -> Result<()> {
    let conn = Connection::open("/Users/peter/tkconv-data/tk.sqlite3")?;

    let toezeggingen = retrieve_toezeggingen(conn);
    let prompt = build_prompt(toezeggingen);

    let res = print_completions(prompt).await;
    dbg!(res.unwrap());

    Ok(())
}
