use anyhow::Result;
use secrecy::{ExposeSecret as _, SecretBox};

use crate::utils::print_response_info;

pub async fn create_timeout_function(
    project_ref: &str,
    active_jwt: &str,
    conn_string: &SecretBox<String>,
) -> Result<()> {
    println!("Creating timeout function in the database...");

    let client = reqwest::Client::new();
    let url = format!(
        "https://api.supabase.com/platform/pg-meta/{}/query",
        project_ref
    );
    let sql = r#"
        CREATE OR REPLACE FUNCTION timeout_function()
        RETURNS VOID AS $$
        BEGIN
            SET statement_timeout = '1s';
            PERFORM pg_sleep(20);
        END;
        $$ LANGUAGE plpgsql;
    "#;
    let payload = serde_json::json!({
        "query": sql,
    });

    let response = client
        .post(&url)
        .header("Authorization", format!("Bearer {}", active_jwt))
        .header("Content-Type", "application/json")
        .header("X-Connection-Encrypted", conn_string.expose_secret())
        .json(&payload)
        .send()
        .await?;
    print_response_info(response).await?;

    Ok(())
}
