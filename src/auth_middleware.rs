use axum::{extract::{Request, State}, http::StatusCode, middleware::Next, response::IntoResponse};
use tracing::error;

use crate::user_service::token_provider::TokenProvider;

#[derive(Debug, Clone)]
pub struct ClientInfo {
    pub client_name: String
}

pub async fn auth_middleware(
    State(jwt_secret): State<String>,
    mut request: Request,
    next: Next
) -> Result<impl IntoResponse, StatusCode> {

    let token_provider = TokenProvider::new(jwt_secret);

    let jwt_token = request.headers().get("bearer")
        .ok_or(StatusCode::UNAUTHORIZED)?;

    let jwt_token_string = jwt_token.to_str().map_err(|err| {
        error!("Error mapping the jwt token to string: {}", err);
        StatusCode::BAD_REQUEST
    })?;

    let claims = token_provider.verify_token(jwt_token_string)
        .map_err(|err| {
            error!("Error in the token verification process: {}", err);
            StatusCode::UNAUTHORIZED
        })?;

    let client_name = claims.claims.sub;

    request.extensions_mut().insert(ClientInfo { client_name });

    let response = next.run(request).await;

    Ok(response)
}
