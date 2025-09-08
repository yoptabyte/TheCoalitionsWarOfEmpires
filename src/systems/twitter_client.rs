use reqwest::Client;
use serde::Serialize;
use std::collections::BTreeMap;
use hmac::{Hmac, Mac};
use sha1::Sha1;
use base64::{Engine as _, engine::general_purpose};
use url::form_urlencoded;

#[derive(Clone)]
pub struct TwitterClient {
    consumer_key: String,
    consumer_secret: String,
    access_token: String,
    access_token_secret: String,
    client: Client,
}

#[derive(Serialize)]
struct TweetData {
    text: String,
}

impl TwitterClient {
    pub fn new(
        consumer_key: &str,
        consumer_secret: &str,
        access_token: &str,
        access_token_secret: &str,
    ) -> Self {
        Self {
            consumer_key: consumer_key.to_string(),
            consumer_secret: consumer_secret.to_string(),
            access_token: access_token.to_string(),
            access_token_secret: access_token_secret.to_string(),
            client: Client::new(),
        }
    }

    pub fn post_game_result_blocking(&self, player_name: &str, victory: bool) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let message = if victory {
            format!("🎉 Victory! {} has conquered all enemy towers in The Coalitions War of Empires! #GameVictory #StrategyGame", player_name)
        } else {
            format!("💀 Defeat! {} fought bravely but all towers were destroyed in The Coalitions War of Empires. #GameDefeat #StrategyGame", player_name)
        };

        let rt = tokio::runtime::Runtime::new()?;
        rt.block_on(self.post_tweet(&message))
    }

    pub fn test_twitter_integration_blocking(&self) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let test_message = "🎮 Testing Twitter integration for The Coalitions War of Empires! #GameTest #StrategyGame";
        let rt = tokio::runtime::Runtime::new()?;
        rt.block_on(self.post_tweet(test_message))
    }

    pub async fn post_game_result(&self, player_name: &str, victory: bool) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let message = if victory {
            format!("🎉 Victory! {} has conquered all enemy towers in The Coalitions War of Empires! #GameVictory #StrategyGame", player_name)
        } else {
            format!("💀 Defeat! {} fought bravely but all towers were destroyed in The Coalitions War of Empires. #GameDefeat #StrategyGame", player_name)
        };

        self.post_tweet(&message).await
    }

    pub async fn test_twitter_integration(&self) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let test_message = "🎮 Testing Twitter integration for The Coalitions War of Empires! #GameTest #StrategyGame";
        self.post_tweet(test_message).await
    }

    async fn post_tweet(&self, text: &str) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        // Generate OAuth 1.0a signature and headers
        let oauth_headers = self.generate_oauth_headers("POST", "https://api.twitter.com/2/tweets", text)?;
        
        let tweet_data = TweetData {
            text: text.to_string(),
        };

        let response = self.client
            .post("https://api.twitter.com/2/tweets")
            .headers(oauth_headers)
            .json(&tweet_data)
            .send()
            .await?;

        if response.status().is_success() {
            println!("✅ Tweet posted successfully!");
            Ok(())
        } else {
            let error_text = response.text().await?;
            eprintln!("❌ Failed to post tweet: {}", error_text);
            Err(format!("Twitter API error: {}", error_text).into())
        }
    }

    fn generate_oauth_headers(
        &self,
        method: &str,
        url: &str,
        _body: &str,
    ) -> Result<reqwest::header::HeaderMap, Box<dyn std::error::Error + Send + Sync>> {
        use std::time::{SystemTime, UNIX_EPOCH};
        
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)?
            .as_secs()
            .to_string();
        
        let nonce = format!("{}{}", timestamp, rand::random::<u32>());

        // OAuth 1.0a parameters
        let mut oauth_params = BTreeMap::new();
        oauth_params.insert("oauth_consumer_key".to_string(), self.consumer_key.clone());
        oauth_params.insert("oauth_nonce".to_string(), nonce);
        oauth_params.insert("oauth_signature_method".to_string(), "HMAC-SHA1".to_string());
        oauth_params.insert("oauth_timestamp".to_string(), timestamp);
        oauth_params.insert("oauth_token".to_string(), self.access_token.clone());
        oauth_params.insert("oauth_version".to_string(), "1.0".to_string());

        // Create parameter string for signature base
        let param_string = oauth_params
            .iter()
            .map(|(k, v)| format!("{}={}", percent_encode(k), percent_encode(v)))
            .collect::<Vec<_>>()
            .join("&");

        // Create signature base string
        let signature_base = format!(
            "{}&{}&{}",
            method,
            percent_encode(url),
            percent_encode(&param_string)
        );

        // Create signing key
        let signing_key = format!(
            "{}&{}",
            percent_encode(&self.consumer_secret),
            percent_encode(&self.access_token_secret)
        );

        // Generate proper HMAC-SHA1 signature
        let signature = self.hmac_sha1(&signing_key, &signature_base)?;
        oauth_params.insert("oauth_signature".to_string(), signature);

        // Create Authorization header
        let auth_header = format!(
            "OAuth {}",
            oauth_params
                .iter()
                .map(|(k, v)| format!("{}=\"{}\"", k, percent_encode(v)))
                .collect::<Vec<_>>()
                .join(", ")
        );

        println!("🔐 Sending Twitter API request...");

        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert("Authorization", auth_header.parse()?);
        headers.insert("Content-Type", "application/json".parse()?);

        Ok(headers)
    }

    fn hmac_sha1(&self, key: &str, data: &str) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
        type HmacSha1 = Hmac<Sha1>;
        let mut mac = HmacSha1::new_from_slice(key.as_bytes())?;
        mac.update(data.as_bytes());
        let result = mac.finalize();
        Ok(general_purpose::STANDARD.encode(result.into_bytes()))
    }
}

// Simple percent encoding for OAuth
fn percent_encode(input: &str) -> String {
    input
        .chars()
        .map(|c| match c {
            'A'..='Z' | 'a'..='z' | '0'..='9' | '-' | '.' | '_' | '~' => c.to_string(),
            _ => format!("%{:02X}", c as u8),
        })
        .collect()
}

// Proper HMAC-SHA1 implementation
fn hmac_sha1(key: &str, data: &str) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
    type HmacSha1 = Hmac<Sha1>;
    let mut mac = HmacSha1::new_from_slice(key.as_bytes())?;
    mac.update(data.as_bytes());
    let result = mac.finalize();
    Ok(general_purpose::STANDARD.encode(result.into_bytes()))
}