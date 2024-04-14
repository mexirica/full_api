use anyhow::Context;
use argon2::{Algorithm, Argon2, Params, PasswordHash, PasswordHasher, PasswordVerifier, Version};
use argon2::password_hash::SaltString;
use sqlx::SqlitePool;

use crate::models::users::Users;
use crate::telemetry::spawn_blocking_with_tracing;

#[derive(thiserror::Error, Debug)]
pub enum AuthError {
    #[error("Credenciais inválidas.")]
    InvalidCredentials(#[source] anyhow::Error),
    #[error(transparent)]
    UnexpectedError(#[from] anyhow::Error),
}

#[tracing::instrument(name = "Get stored credentials", skip(username, pool))]
async fn get_stored_credentials(
    username: &str,
    pool: &SqlitePool,
) -> Result<Option<(String, String)>, anyhow::Error> {
    let row = sqlx::query!(
        r#"
        SELECT username, password
        FROM users
        WHERE username = $1
        "#,
        username,
    )
        .fetch_optional(pool)
        .await
        .context("Erro ao recuperar os dados do usuário.")?
        .map(|row| (row.username, row.password));
    Ok(row)
}

#[tracing::instrument(name = "Validate credentials", skip(credentials, pool))]
pub async fn validate_credentials(
    credentials: Users,
    pool: &SqlitePool,
) -> Result<String, AuthError> {
    let mut user_id = None;
    let mut expected_password_hash =
        "$argon2id$v=19$m=15000,t=2,p=1$\
        gZiV/M1gPc22ElAH/Jh1Hw$\
        CWOrkoo7oJBQ/iyh7uJ0LO2aLEfrHwTWllSAxT0zRno"
            .to_string()
    ;

    if let Some((stored_user_id, stored_password_hash)) =
        get_stored_credentials(&credentials.username, pool).await?
    {
        user_id = Some(stored_user_id);
        expected_password_hash = stored_password_hash
    }

    spawn_blocking_with_tracing(move || {
        verify_password_hash(expected_password_hash, credentials.password)
    })
        .await
        .context("Failed to spawn blocking task.")??;

    user_id
        .ok_or_else(|| anyhow::anyhow!("Unknown username."))
        .map_err(AuthError::InvalidCredentials)
}

#[tracing::instrument(
name = "Validate credentials",
skip(expected_password_hash, password_candidate)
)]
fn verify_password_hash(
    expected_password_hash: String,
    password_candidate: String,
) -> Result<(), AuthError> {
    let expected_password_hash = PasswordHash::new(&expected_password_hash)
        .context("Erro ao converter hash para string PHC.")?;

    Argon2::default()
        .verify_password(
            password_candidate.as_bytes(),
            &expected_password_hash,
        )
        .context("Invalid password.")
        .map_err(AuthError::InvalidCredentials)
}

#[tracing::instrument(name = "Change password", skip(password, pool))]
pub async fn change_password(
    username: String,
    password: String,
    pool: &SqlitePool,
) -> Result<(), anyhow::Error> {
    let password_hash = spawn_blocking_with_tracing(move || compute_password_hash(&password))
        .await?
        .context("Failed to hash password")?;
    sqlx::query!(
        r#"
        UPDATE users
        SET password = $1
        WHERE username = $2
        "#,
        password_hash,
        username
    )
        .execute(pool)
        .await
        .context("Erro ao alterar a senha.")?;
    Ok(())
}

pub(crate) fn compute_password_hash(password: &String) -> Result<String, anyhow::Error> {
    let salt = SaltString::generate(&mut rand::thread_rng());
    let password_hash = Argon2::new(
        Algorithm::Argon2id,
        Version::V0x13,
        Params::new(15000, 2, 1, None).unwrap(),
    )
        .hash_password(password.as_bytes(), &salt)?
        .to_string();
    Ok(password_hash)
}