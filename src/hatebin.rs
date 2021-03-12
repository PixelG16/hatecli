//! Hatebin-related functionality.

use reqwest::Error;
use reqwest::blocking::Client;

/// Maximum amount of characters that can be uploaded before they're truncated.
pub const CHARACTER_LIMIT: usize = 50000;

/// Upload text to [Hatebin](https://hatebin.com).
pub fn upload(text: &str) -> Result<String, Error>
{
    // Create HTTP Client.
    let client = Client::new();

    // Form parameters to be posted.
    let params = [("text", text)];

    // Post text.
    let resp = client.post("https://hatebin.com/index.php").form(&params).send()?;

    // Return code.
    Ok(resp.text()?.trim().to_string())
}