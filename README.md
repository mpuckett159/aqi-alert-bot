aqi-alert-bot

Environment Variables:
 * DISCORD_WEBHOOK_URL - required, webhook URL for the channel. Found in the channel settings
 * AQI_API_TOKEN - token acquired from https://aqicn.org/api/ to access their API
 * LAT_LONG - latitude and longitude to monitor AQI of in the format lat;long, e.g. 33.6772973;-106.477862, notice the semicolon as the delimiter, not a comma.
 * AQI_THRESHOLD - AQI threshold you want to monitor the crossing of, see https://www.airnow.gov/aqi/aqi-basics/ for more detials
 * MAX_ERRORS - max number of consecutive errors before the bot sends an error message to the designated discord chat, no tuning done here no idea what a good number is I'm useing 5
 * POLL_INTERVAL - number of seconds between API requests, minimum of 60 is likely best due to frequency of station updates