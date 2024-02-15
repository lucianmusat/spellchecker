mod spellchecker;
mod spellchecked;
mod spellcheck_parser;

use log::{info, debug, error};
use tera::Tera;


const LISTEN_ADDRESS: &str = "0.0.0.0:8080";


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
    let spellcheck_parser = spellcheck_parser::SpellcheckParser::new();
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
        let spellchecked_sentence = spellcheck_parser.spellcheck_all(&body_string_decoded);
        context.insert("spellchecked_sentences", &spellchecked_sentence);
    }
    // Because this is both backend and frontend, we render the template for every request
    Ok(render_frontend(tera, context).await?)
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
