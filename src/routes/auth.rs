use utoipa_actix_web::service_config::ServiceConfig;

use crate::controllers::auth::{signin, signup};

pub fn auth_routes(cfg: &mut ServiceConfig) {
    cfg.service(
        utoipa_actix_web::scope("/auth")
            .service(signup)
            .service(signin),
    );
}
