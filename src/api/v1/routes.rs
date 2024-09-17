use crate::logger::log;
use crate::api::v1::{
    types::TranslateForm,
    services::lsf_translate
};
use actix_web::{get, post, web, HttpResponse, Responder};

/**
 * Ping routes for the api v1
 */
#[get("/ping")]
pub async fn ping() -> impl Responder {
    HttpResponse::Ok().body("pong")
}

/**
 * Translation routes that take a text and return its translation
 */
#[post("/translate")]
pub async fn translate(body: web::Json<TranslateForm>) -> impl Responder {
    let translation: String = lsf_translate(format!("{}", body.text));
    let words: Vec<&str> = translation.split(" ").collect();
    let filtered: Vec<&str> = words.iter().filter(|&word| word != &"").map(|&word| word).collect();
    let joined: String = filtered.join(" ");

    log::new().syntax(&format!("words: {:?}", words));
    log::new().syntax(&format!("filtered: {:?}", filtered));
    log::new().syntax(&format!("joined: {}", joined));
    HttpResponse::Ok().body(joined)
}
