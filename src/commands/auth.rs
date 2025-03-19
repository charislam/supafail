use anyhow::Result;

use crate::utils::print_response_info;

pub async fn sign_up_no_password(project_ref: &str, anon_key: &str) -> Result<()> {
    println!("Attempting to sign up without a password (expected: 400 Bad Request)");

    let client = reqwest::Client::new();
    let url = format!("https://{}.supabase.co/auth/v1/signup", project_ref);
    let payload = serde_json::json!({
        "email": "not_real@fake_email.com",
        "password": ""
    });

    let response = client
        .post(&url)
        .header("apikey", anon_key)
        .header("Content-Type", "application/json")
        .json(&payload)
        .send()
        .await?;
    print_response_info(response).await?;

    Ok(())
}

pub async fn sign_in_fake_user(project_ref: &str, anon_key: &str) -> Result<()> {
    println!("Attempting to sign in with a fake user (expected: 400 Bad Request)");

    let client = reqwest::Client::new();
    let url = format!("https://{}.supabase.co/auth/v1/signup", project_ref);
    let payload = serde_json::json!({
        "email": "not_real@fake_email.com",
        "password": "dsahui429978"
    });

    let response = client
        .post(&url)
        .header("apikey", anon_key)
        .header("Content-Type", "application/json")
        .json(&payload)
        .send()
        .await?;
    print_response_info(response).await?;

    Ok(())
}

pub async fn call_admin_with_anon(project_ref: &str, anon_key: &str) -> Result<()> {
    println!("Attempting to call auth admin endpoint with ANON KEY (expected: 403 Forbidden)");

    let client = reqwest::Client::new();
    let url = format!("https://{}.supabase.co/auth/v1/admin/users", project_ref);
    let payload = serde_json::json!({
        "email": "user@email.com",
        "email_confirm": true
    });

    let response = client
        .post(&url)
        .header("apikey", anon_key)
        .header("Authorization", format!("Bearer {}", anon_key))
        .header("Content-Type", "application/json")
        .json(&payload)
        .send()
        .await?;
    print_response_info(response).await?;

    Ok(())
}
