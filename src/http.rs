use std::sync::LazyLock;

use anyhow::anyhow;

static HTTP_CLIENT: LazyLock<ureq::Agent> = LazyLock::new(ureq::agent);

pub fn get(url: &str) -> anyhow::Result<String> {
    let request = HTTP_CLIENT.get(url);

    request
        .call()
        .map_err(|e| anyhow!("HTTP request failed: {}", e))?
        .into_body()
        .read_to_string()
        .map_err(|e| anyhow!("Failed to read response body: {}", e))
}
