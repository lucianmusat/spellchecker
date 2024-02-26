mod spellchecker;
mod spellchecked;
mod spellcheck_parser;

use log::{info, debug, error};
use tera::Tera;
use crate::spellcheck_parser::SpellcheckParser;

const LISTEN_ADDRESS: &str = "0.0.0.0:8080";
const TEXT_INPUT_NAME: &str = "textInput";
const MAX_TEXT_LENGTH: usize = 150;


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
    app.at("/static").serve_dir("static/")?;
    app.at("/").get(handle_index).post(handle_index);
    app.listen(LISTEN_ADDRESS).await?;
    info!("Server listening on {}...", LISTEN_ADDRESS);
    Ok(())
}

async fn handle_index(mut req: tide::Request<Tera>) -> tide::Result {
    let body_string = req.body_string().await?;
    let tera = req.state();
    let mut context = tera::Context::new();
    let spellcheck_parser = SpellcheckParser::new();
    if spellcheck_parser.is_err() {
        let response = generate_error_response(spellcheck_parser);
        return Ok(response);
    }
    if !body_string.is_empty() {
        let decoded_form_data: Vec<(String, String)> = form_urlencoded::parse(body_string.as_bytes())
                                                                    .into_owned()
                                                                    .collect();
        let body_string_decoded =  decoded_form_data
                                        .iter()
                                        .find(|(key, _)| key == TEXT_INPUT_NAME)
                                        .map(|(_, value)| value.clone())
                                        .unwrap_or_default();
        let body_string_decoded = body_string_decoded[..body_string_decoded.len().min(MAX_TEXT_LENGTH)].trim().to_string();
        debug!("Received body: {}", body_string_decoded);
        let spellchecked_sentence = spellcheck_parser.unwrap().spellcheck_all(&body_string_decoded);
        context.insert("spellchecked_sentences", &spellchecked_sentence);
    }
    // Because this is both backend and frontend, we render the template for every request
    render_frontend(tera, context).await
}

fn generate_error_response(spellcheck_parser: Result<SpellcheckParser, String>) -> tide::Response {
    let error_message = format!("Failed to initialize SpellcheckParser: {}", spellcheck_parser.err().unwrap());
    error!("{}", error_message);
    let response_body = serde_json::json!({
            "error": error_message,
        });
    let mut response = tide::Response::new(tide::StatusCode::InternalServerError);
    response.set_body(serde_json::to_string(&response_body).unwrap());
    response.insert_header(tide::http::headers::CONTENT_TYPE, "application/json");
    response
}

async fn render_frontend(tera: &Tera, context: tera::Context) -> tide::Result {
    let rendered_content = tera.render("index.html", &context)
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
