use crate::docs::users::__path_get_user;
use utoipa::OpenApi;
use crate::routes::users;

#[derive(OpenApi)]
#[openapi(
info(title = "User API", version = "1.0.0"),
paths(
/{username})
)]
pub struct OpenApiDoc;
