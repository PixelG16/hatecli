//! `hatecli` is a command-line version of [Hatebin](https://hatebin.com).

pub mod cli;
mod hatebin;

use std::io::Read;

use cli::App;
use reqwest::blocking;

/// Primary logic of program.
pub fn run(app: &mut App)
{
    // Text to upload.
    if let Some(text) = app.text()
    {
        // If user does not care about 
        // issues such as truncation, don't check.
        if !app.force()
        {
            if text.chars().count() > hatebin::CHARACTER_LIMIT
            {
                eprintln!("Unfortunately, your text exceeds the 50000 character limit. If you are okay with truncation, use '-F'!");
                return;
            }
        }

        // Upload text.
        match hatebin::upload(text)
        {
            Ok(code) => println!("https://hatebin.com/{}", code),
            Err(err) =>
            {
                if err.is_body() { println!("Failed to make a POST request or receive response because the body."); }
                else if err.is_builder() { println!("Failed to build a correct request or response."); }
                else if err.is_connect() { println!("Failed to make a connection."); }
                else if err.is_decode() { println!("Failed to decode the response body."); }
                else if err.is_redirect() { println!("Failed to receive desired response because a redirect policy."); }
                else if err.is_request() { println!("Failed to make request because there was an issue with the request itself."); }
                else if err.is_status() { println!("Failed to receive desired response. Status code: {}", err.status().unwrap()); }
                else if err.is_timeout() { println!("Failed to make request because it timed out."); }
            }
        }
    }
    else if let Some(file) = app.file_mut()
    {
        // Buffer for file content.
        let mut text = String::new();
        if let Err(_) = file.read_to_string(&mut text)
        {
            eprintln!("Failed to read from file.");
            return;
        }

        // If user does not care about 
        // issues such as truncation, don't check.
        if !app.force()
        {
            if text.chars().count() > hatebin::CHARACTER_LIMIT
            {
                eprintln!("Unfortunately, your text exceeds the 50000 character limit. If you are okay with truncation, use '-F'!");
                return;
            }
        }

        // Upload text.
        match hatebin::upload(&text)
        {
            Ok(code) => println!("https://hatebin.com/{}", code),
            Err(err) =>
            {
                if err.is_body() { println!("Failed to make a POST request or receive response because the body."); }
                else if err.is_builder() { println!("Failed to build a correct request or response."); }
                else if err.is_connect() { println!("Failed to make a connection."); }
                else if err.is_decode() { println!("Failed to decode the response body."); }
                else if err.is_redirect() { println!("Failed to receive desired response because a redirect policy."); }
                else if err.is_request() { println!("Failed to make request because there was an issue with the request itself."); }
                else if err.is_status() { println!("Failed to receive desired response. Status code: {}", err.status().unwrap()); }
                else if err.is_timeout() { println!("Failed to make request because it timed out."); }
            }
        }
    }
    else if let Some(url) = app.url()
    {
        // Get body at URL.
        let body = match blocking::get(url.as_str())
        {
            Ok(res) => if let Ok(body) = res.text()
            {
                body
            }
            else
            {
                eprintln!("It does not appear as though there is any text at that URL?");
                return;
            },
            Err(err) => 
            {
                if err.is_body() { println!("Failed to make a POST request or receive response because the body."); }
                else if err.is_builder() { println!("Failed to build a correct request or response."); }
                else if err.is_connect() { println!("Failed to make a connection."); }
                else if err.is_decode() { println!("Failed to decode the response body."); }
                else if err.is_redirect() { println!("Failed to receive desired response because a redirect policy."); }
                else if err.is_request() { println!("Failed to make request because there was an issue with the request itself."); }
                else if err.is_status() { println!("Failed to receive desired response. Status code: {}", err.status().unwrap()); }
                else if err.is_timeout() { println!("Failed to make request because it timed out."); }
                return;
            }
        };

        // If user does not care about 
        // issues such as truncation, don't check.
        if !app.force()
        {
            if body.chars().count() > hatebin::CHARACTER_LIMIT
            {
                eprintln!("Unfortunately, your text exceeds the 50000 character limit. If you are okay with truncation, use '-F'!");
                return;
            }
        }

        // Upload text.
        match hatebin::upload(&body)
        {
            Ok(code) => println!("https://hatebin.com/{}", code),
            Err(err) =>
            {
                if err.is_body() { println!("Failed to make a POST request or receive response because the body."); }
                else if err.is_builder() { println!("Failed to build a correct request or response."); }
                else if err.is_connect() { println!("Failed to make a connection."); }
                else if err.is_decode() { println!("Failed to decode the response body."); }
                else if err.is_redirect() { println!("Failed to receive desired response because a redirect policy."); }
                else if err.is_request() { println!("Failed to make request because there was an issue with the request itself."); }
                else if err.is_status() { println!("Failed to receive desired response. Status code: {}", err.status().unwrap()); }
                else if err.is_timeout() { println!("Failed to make request because it timed out."); }
            }
        }
    }
    else
    {
        // If reached, I fucked up.
        unreachable!();
    }
}