use std::collections::HashMap;

#[tauri::command]
pub async fn fetch_url(url: String) -> Result<Vec<u8>, String> {
    let response = reqwest::get(&url)
        .await
        .map_err(|e| format!("fetch failed: {}", e))?;

    let bytes = response
        .bytes()
        .await
        .map_err(|e| format!("read failed: {}", e))?;

    Ok(bytes.to_vec())
}

#[tauri::command]
pub async fn fetch_url_text(url: String) -> Result<String, String> {
    let response = reqwest::get(&url)
        .await
        .map_err(|e| format!("fetch failed: {}", e))?;

    response
        .text()
        .await
        .map_err(|e| format!("read failed: {}", e))
}

#[tauri::command]
pub async fn proxy_request(
    url: String,
    method: String,
    headers: HashMap<String, String>,
    body: Option<String>,
) -> Result<String, String> {
    let client = reqwest::Client::new();
    
    let mut req = match method.to_uppercase().as_str() {
        "POST" => client.post(&url),
        "PUT" => client.put(&url),
        "DELETE" => client.delete(&url),
        "PATCH" => client.patch(&url),
        _ => client.get(&url),
    };

    for (key, value) in &headers {
        req = req.header(key.as_str(), value.as_str());
    }

    if let Some(b) = body {
        req = req.body(b);
    }

    let response = req
        .send()
        .await
        .map_err(|e| format!("proxy request failed: {}", e))?;

    response
        .text()
        .await
        .map_err(|e| format!("proxy read failed: {}", e))
}
