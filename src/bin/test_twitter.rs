use std::fs;
use toml::Value;
use the_coalitions_war_of_empires::systems::twitter_client::TwitterClient;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🐦 Testing Twitter API Configuration...\n");

    // Try to load configuration
    let config_content = match fs::read_to_string("twitter_config.toml") {
        Ok(content) => content,
        Err(e) => {
            eprintln!("❌ Error reading twitter_config.toml: {}", e);
            eprintln!("📝 Please create twitter_config.toml with your API credentials:");
            eprintln!("
[twitter]
consumer_key = \"your_api_key_here\"
consumer_secret = \"your_api_secret_here\"
access_token = \"your_access_token_here\"
access_token_secret = \"your_access_token_secret_here\"
");
            return Err(e.into());
        }
    };

    let config: Value = toml::from_str(&config_content)?;
    
    let twitter_section = config.get("twitter")
        .ok_or("Missing [twitter] section in config")?;

    let consumer_key = twitter_section.get("consumer_key")
        .and_then(|v| v.as_str())
        .ok_or("Missing consumer_key in config")?;
    
    let consumer_secret = twitter_section.get("consumer_secret")
        .and_then(|v| v.as_str())
        .ok_or("Missing consumer_secret in config")?;
    
    let access_token = twitter_section.get("access_token")
        .and_then(|v| v.as_str())
        .ok_or("Missing access_token in config")?;
    
    let access_token_secret = twitter_section.get("access_token_secret")
        .and_then(|v| v.as_str())
        .ok_or("Missing access_token_secret in config")?;

    println!("✅ Configuration loaded successfully!");
    println!("🔑 Consumer Key: {}...", &consumer_key[..std::cmp::min(10, consumer_key.len())]);
    println!("🔑 Access Token: {}...", &access_token[..std::cmp::min(10, access_token.len())]);
    println!();

    // Create Twitter client
    let client = TwitterClient::new(
        consumer_key,
        consumer_secret,
        access_token,
        access_token_secret,
    );

    println!("🧪 Testing Twitter API connection...");
    
    // Test the connection
    match client.test_twitter_integration_blocking() {
        Ok(_) => {
            println!("🎉 SUCCESS! Twitter API is working correctly!");
            println!("✅ OAuth signatures are valid");
            println!("✅ API credentials are correct");
            println!("✅ App permissions are sufficient");
        }
        Err(e) => {
            eprintln!("❌ FAILED! Twitter API test failed: {}", e);
            eprintln!();
            eprintln!("🔍 Common issues to check:");
            eprintln!("1. App Configuration (in Twitter Developer Portal):");
            eprintln!("   - Set a valid Callback URL (e.g., https://example.com/callback)");
            eprintln!("   - Set a valid Website URL (e.g., https://example.com)");
            eprintln!("   - Enable 'User authentication settings'");
            eprintln!("   - Set App permissions to 'Read and write'");
            eprintln!();
            eprintln!("2. API Keys:");
            eprintln!("   - Make sure all 4 keys are complete (not truncated)");
            eprintln!("   - Regenerate keys if they might be compromised");
            eprintln!();
            eprintln!("3. OAuth Settings:");
            eprintln!("   - Enable OAuth 1.0a");
            eprintln!("   - Set proper callback URLs");
            return Err(e);
        }
    }

    Ok(())
}
