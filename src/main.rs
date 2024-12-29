mod config;
mod controllers;
mod database;
mod routes;
mod services;
mod types;

use actix_cors::Cors;
use actix_web::{web, App, HttpServer};
use config::load_config;
use dotenv::dotenv;
use routes::auth::auth_routes;
use types::ContextData;
use utoipa::OpenApi;
use utoipa_actix_web::AppExt;
use utoipa_swagger_ui::SwaggerUi;


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    tracing_subscriber::fmt::init();

    #[derive(OpenApi)]
    struct ApiDoc;

    let mut doc = ApiDoc::openapi();
    doc.info.title = String::from("Invee API");
    doc.info.description = Some(String::from("A Stock API magna for inventory management"));

    let config = load_config();
    let db = database::connection::create(&config.database_url)
        .await
        .unwrap();

    let data= ContextData  {
        config: config.clone(),
        db: db.clone(),
    };

    HttpServer::new(move || {
        App::new()
            .wrap(Cors::default().max_age(3600))
            .into_utoipa_app()
            .openapi(doc.clone())
            .app_data(web::Data::new(data.clone()))
            .service(utoipa_actix_web::scope("/api").configure(auth_routes))
            .openapi_service(|api| SwaggerUi::new("/docs/{_:.*}").url("/docs/openapi.json", api))
            .into_app()
    })
    .bind(("0.0.0.0", config.port))?
    .run()
    .await
}
