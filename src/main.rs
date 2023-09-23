use std::env;

use serenity::http::Http;
use serenity::model::webhook::Webhook;
use serde_json::Value;
use tokio::time::{sleep, Duration};

#[tokio::main]
async fn main() {
    // Set necessary env vars
    let aqi_api_token = env::var("AQI_API_TOKEN")
        .expect("Expected an AQI_API_TOKEN in the environment");

    let lat_long = env::var("LAT_LONG")
        .expect("Expected a LAT_LONG in the environment");

    let discord_webhook = env::var("DISCORD_WEBHOOK_URL")
        .expect("Expected a DISCORD_WEBHOOK_URL in the environment");

    let aqi_threshold = env::var("AQI_THRESHOLD")
        .expect("Expected an AQI_THRESHOLD in the environment")
        .parse::<u64>()
        .expect("Expected AQI_THRESHOLD to be a parsable interger (u64)");

    let poll_interval = env::var("POLL_INTERVAL")
        .expect("Expected a POLL_INTERVAL in the environment")
        .parse::<u64>()
        .expect("Expected POLL_INTERVAL to be a parsable interger (u64)");

    // Initialize mutable tracker for AQI messurements
    let mut hist_aqi = 0;

    // Declare new http to use for webhook calls
    let http = Http::new("");

    // Store formatted url for AQI API calls
    let aqi_url = format!("https://api.waqi.info/feed/geo:{lat_long}/?token={aqi_api_token}");
    
    // Configure webhook endpoint from above env var
    let webhook = Webhook::from_url(&http, &discord_webhook).await.expect("Error creating webhook object from url.");

    loop {
        // Get json resp from aqi tracker site
        let body = reqwest::get(&aqi_url)
            .await
            .unwrap()
            .text()
            .await
            .unwrap();
        
        // Get current AQI from response JSON payload
        let aqi_json: Value = serde_json::from_str(&body).expect("Could not deserialize json.");
        let aqi = aqi_json["data"]["aqi"]
            .to_string()
            .parse::<u64>()
            .expect("Expected value that can be parsed to u64 at .data.aqi from AQI API JSON body.");
        
        // Send message via webhook if necessary
        if aqi > aqi_threshold && hist_aqi < aqi_threshold {
            webhook.execute(&http, 
                false, 
                |w| 
                w.content(format!("@everyone AQI is now above {aqi_threshold}\nCurrent AQI = {aqi}")))
                .await
                .expect("Error executing webhook");
        } else if aqi < aqi_threshold && hist_aqi > aqi_threshold {
            webhook.execute(&http, 
                false, 
                |w| 
                w.content(format!("@everyone AQI is now below {aqi_threshold}\nCurrent AQI = {aqi}")))
                .await
                .expect("Error executing webhook");
        } else {
            println!("AQI is {aqi}, previous AQI was {hist_aqi}, no need to send message");
        }

        // Set update AQI tracker to most recent AQI
        hist_aqi = aqi;

        // So as not to overload the API sleep for set poll interval
        sleep(Duration::from_secs(poll_interval)).await;
    }
}