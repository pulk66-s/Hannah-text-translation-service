mod db;
mod AST;
mod api;
mod context;
mod json;
mod logger;
mod regex;
mod tests;
mod translation;

use logger::log;
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use api::v1::routes::{ping, translate};

/**
 * Main function for rust actix web server. It builds and provides an api
 */
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    log::new().syntax(&format!("Starting server... {}", 42));
    HttpServer::new(|| App::new().service(web::scope("/api/v1").service(ping).service(translate)))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}

// fn main() {
//     let mut db = db::mysql::Mysql::new();
    
//     println!("{:?}", db.verbs.find("aller"));
// }

// use AST::expr::Expr;
// use AST::text::Text;
// use context::Context;

// /**
//  * Testing main function
//  */
// fn main() {
//     let phrase: String = "le chat noir.".to_string();
//     let context = Context::new();
//     let mut text: Text = Text::new(&context);
//     let (input, result) = text.parse(&phrase);

//     if !result {
//         log::new().syntax(&format!("Error: {}", input);
//     } else {
//         log::new().syntax(&format!("{{ \"Success\": {}}}", text);
//     }

//     log::new().syntax(&format!("AST -> LSF");

//     let res = translation::lsf::translate::lsf_translation(text);

//     log::new().syntax(&format!("res: {}", res);
// }
