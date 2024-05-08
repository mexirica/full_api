use crate::__path_health;
use super::{routes,health};
use utoipa::{Modify, OpenApi};
use utoipa::openapi::security::{HttpAuthScheme, HttpBuilder, SecurityScheme};

#[derive(OpenApi)]
#[openapi(
paths(
health,
routes::auth::login
),
)]
pub(crate) struct ApiDoc;

struct SecurityAddon;

impl Modify for SecurityAddon {
    fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
        let components = openapi.components.as_mut().unwrap();
        components.add_security_scheme(
            "bearerAuth", SecurityScheme::Http(HttpBuilder::new()
                .scheme(HttpAuthScheme::Bearer)
                .bearer_format("JWT")
                .build()),
        );
        components.add_security_scheme(
            "basicAuth", SecurityScheme::Http(HttpBuilder::new()
                .scheme(HttpAuthScheme::Basic)
                .build()),
        );
    }
}