mod spellchecker;

use log::{info, debug, error};

const LISTEN_ADDRESS: &str = "0.0.0.0:8080";

#[async_std::main]
async fn main() -> tide::Result<()> {
    env_logger::builder()
        .filter_level(log::LevelFilter::Debug)
        .format_target(false)
        .format_timestamp(None)
        .init();

    let mut app = tide::new();
    app.at("/").get(root);
    app.at("/").post(handle_request);

    app.listen(LISTEN_ADDRESS).await?;
    info!("Server listening on {}...", LISTEN_ADDRESS);
    Ok(())
}

async fn root(_req: tide::Request<()>) -> tide::Result<String> {
    Ok("<html> <h1>Main Page</h1> <p>Awesome html skills!</p> </html>".to_string())
}

async fn handle_request(mut req: tide::Request<()>) -> tide::Result<String> {
    let body_string = req.body_string().await?;
    debug!("Received body: {}", body_string);
    let spellchecked_sentence = spellcheck(&body_string).await;
    Ok(spellchecked_sentence)
}

async fn spellcheck(to_spellchek: &str) -> String {
    let mut spellchecked_sentence = String::new();
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
                spellchecked_sentence.push_str(&spellchecked_word);
                spellchecked_sentence.push_str(" ");
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