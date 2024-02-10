mod spellchecker;

use log::{info, debug, error};
use tera::Tera;

const LISTEN_ADDRESS: &str = "0.0.0.0:8080";

#[derive(Debug, serde::Serialize, serde::Deserialize)]
struct Spellchecked {
    original: String,
    spellchecked: String,
}

#[async_std::main]
async fn main() -> tide::Result<()> {
    env_logger::builder()
        .filter_level(log::LevelFilter::Debug)
        .format_target(false)
        .format_timestamp(None)
        .init();

    let mut tera = Tera::new("templates/**/*").expect("Failed to initialize Tera!");
    tera.autoescape_on(vec!["html"]);

    let mut app = tide::with_state(tera);
    app.at("/").get(handle_index).post(handle_index);
    app.listen(LISTEN_ADDRESS).await?;
    info!("Server listening on {}...", LISTEN_ADDRESS);

    Ok(())
}

async fn handle_index(mut req: tide::Request<Tera>) -> tide::Result {
    let body_string = req.body_string().await?;
    let tera = req.state();
    let mut context = tera::Context::new();

    if !body_string.is_empty() {
        let decoded_form_data: Vec<(String, String)> = form_urlencoded::parse(body_string.as_bytes())
                                                                    .into_owned()
                                                                    .collect();
        let body_string_decoded =  decoded_form_data
                                        .iter()
                                        .find(|(key, _)| key == "textInput")
                                        .map(|(_, value)| value.clone())
                                        .unwrap_or_default();
        debug!("Received body: {}", body_string_decoded);
        let spellchecked_sentence = spellcheck(&body_string_decoded).await;
        context.insert("spellchecked_sentences", &spellchecked_sentence);
    }

    let rendered_content = tera
                                    .render("index.html", &context)
                                    .map_err(|e| {
                                            error!("Failed to render template: {}", format!("{:?}", e));
                                            tide::Error::new(tide::StatusCode::InternalServerError, e)
                                        }
                                    )?;
    let mut response = tide::Response::new(tide::StatusCode::Ok);
    response.set_body(rendered_content);
    response.insert_header(tide::http::headers::CONTENT_TYPE, "text/html");

    Ok(response)
}

async fn spellcheck(to_spellchek: &str) -> Vec<Spellchecked> {
    let mut spellchecked_sentence = Vec::new();
    let spellchecker = spellchecker::Spellchecker::new("dictionary.txt");
    match spellchecker {
        Some(spellchecker) => {
            for original_word in to_spellchek.split_whitespace() {
                let result = spellchecker.spellcheck(&original_word.to_lowercase());
                let mut spellchecked_word = match result {
                    Some(word) => word,
                    None => original_word.to_string(),
                };
                capitalize_if_needed(&original_word, &mut spellchecked_word);
                spellchecked_sentence.push(Spellchecked {
                    original: original_word.to_string(),
                    spellchecked: spellchecked_word,
                });
            }
        }
        None => error!("Could not create spellchecker!"),
    }
    spellchecked_sentence
}

fn capitalize_if_needed(original_word: &str, spellchecked_word: &mut String) {
    if original_word.chars().next().unwrap().is_uppercase() {
        let first_char = spellchecked_word.chars().next().unwrap().to_uppercase().to_string();
        spellchecked_word.replace_range(..1, &first_char);
    }
}