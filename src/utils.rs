use anyhow::Result;
use reqwest::Response;
use serde_json::Value;

pub async fn print_response_info(response: Response) -> Result<()> {
    let status = response.status();
    let status_code = status.as_u16();
    println!(
        "Status: {} ({})",
        status_code,
        status.canonical_reason().unwrap_or("Unknown")
    );

    println!("Headers:");
    for (key, value) in response.headers() {
        println!("    {}: {}", key, value.to_str().unwrap_or("[binary data]"));
    }

    let response_text = response.text().await?;
    println!("Response body:");
    if let Ok(json) = serde_json::from_str::<Value>(&response_text) {
        let pretty_json = serde_json::to_string_pretty(&json)?;
        for line in pretty_json.lines() {
            println!("    {}", line);
        }
    } else {
        for line in response_text.lines() {
            println!("    {}", line);
        }
    }

    Ok(())
}
