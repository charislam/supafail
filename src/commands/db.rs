use anyhow::Result;

use crate::utils::print_response_info;

pub mod setup;

pub async fn select_nonexistent_table(project_ref: &str, anon_key: &str) -> Result<()> {
    println!("Attempting to select from nonexistent table (expected: 404 Not Found)");

    let client = reqwest::Client::new();
    let url = format!(
        "https://{}.supabase.co/rest/v1/table_dne?select=id",
        project_ref
    );

    let response = client
        .get(&url)
        .header("apikey", anon_key)
        .header("Authorization", format!("Bearer {}", anon_key))
        .send()
        .await?;
    print_response_info(response).await?;

    Ok(())
}

pub async fn incomplete_anon_key(project_ref: &str, anon_key: &str) -> Result<()> {
    println!("Attempting API call with incomplete ANON KEY (expected: 401 Unauthorized)");

    let truncated_key = anon_key
        .chars()
        .take(anon_key.len() / 2)
        .collect::<String>();
    println!("Using truncated key: {}...", &truncated_key[0..10]);

    let client = reqwest::Client::new();
    let url = format!(
        "https://{}.supabase.co/rest/v1/table_dne?select=id",
        project_ref
    );

    let response = client
        .get(&url)
        .header("apikey", anon_key)
        .header("Authorization", format!("Bearer {}", truncated_key))
        .send()
        .await?;
    print_response_info(response).await?;

    Ok(())
}
