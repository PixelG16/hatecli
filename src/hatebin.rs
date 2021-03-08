//! Hatebin-related functionality.

use reqwest::Error;
use reqwest::blocking::Client;

/// Maximum amount of characters that can be uploaded before they're truncated.
pub const CHARACTER_LIMIT: u16 = 50000;

/// Upload text to [Hatebin](https://hatebin.com).
pub fn upload(text: String) -> Result<String, Error>
{
    // Create HTTP Client.
    let client = Client::new();

    // Payload to be posted.
    let payload = format!("text={}", text);

    // Post text.
    let resp = client.post("https://hatebin.com").body(payload).send()?;

    // Return code.
    Ok(resp.text()?)
}