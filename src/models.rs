
//#[derive(Debug)]
pub struct Toezegging {
    pub tekst: String,
    pub nummer: String,
}


#[derive(Debug, serde::Deserialize)]
pub struct Onderwerp {
    pub onderwerp: String,
    pub toezegging_ids: Vec<String>,
}
