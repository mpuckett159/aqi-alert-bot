import os
from textwrap import dedent
import time
from typing import Any

import requests
from discord_webhook import DiscordWebhook


def _env(
    key: str,
    fail: bool = True,
    default: Any = None,
) -> Any:
    """
    Function used to read container/OS environmnet variables in and return the
    values to be stored in global Python variables.
    """
    value = os.environ.get(key)
    if value is None:
        if fail and default is None:
            raise KeyError(f"Key '{key}' is not present in environment!")
        value = default
    return value


DISCORD_WEBHOOK_URL = _env(key="DISCORD_WEBHOOK_URL")


def send_discord_message(url: str, message: str) -> None:
    print("sending update message to Discord channel.")
    webhook = DiscordWebhook(url=url, content=message)
    webhook.execute()


def above_threshold(aqi: int) -> str:
    return dedent(
        f"""\
    @here AQI is now above 100
    Current AQI = {aqi}
    """
    )


def below_threshold(aqi: int) -> str:
    return dedent(
        f"""\
    @everyone AQI is now below 100
    Current AQI = {aqi}
    """
    )


def error(error_message: str) -> str:
    return dedent(
        f"""\
    @everyone error requesting AQI
    {error_message}
    """
    )


def main():
    """
    Top level function for running the twitter-to-signal loop.
    """

    # Load Environment Variables
    print("Loading environment variables")
    aqi_api_token = _env("AQI_API_TOKEN")
    lat_long = _env("LAT_LONG")
    discord_webhook = _env("DISCORD_WEBHOOK_URL")
    print(f"AQI API Token = {aqi_api_token}")
    print(f"Latitude and Longitude = {lat_long}")
    print(f"Discord Webhook URL = {discord_webhook}")

    # track prior AQI variable so we don't alert constantly only when
    # the threshold is crossed
    hist_aqi = 0

    # track errors so we only alert after so many failures
    error_max = 10
    
    while True:
        # make request
        r = requests.get(f'https://api.waqi.info/feed/geo:{lat_long}/?token={aqi_api_token}')

        if r.status_code != 200:
            print("error making request, please see error message below")
            print(f'Status Code = {r.status_code}')
            print(r.text)
            error_max -= 1

            if error_max < 0:
                send_discord_message(url=DISCORD_WEBHOOK_URL, message=error(error_message=r.text))
        else:
            aqi = int(r.json()['data']['aqi'])
            if aqi > 100 and hist_aqi < 100:
                send_discord_message(url=DISCORD_WEBHOOK_URL, message=above_threshold(aqi=aqi))
            elif aqi < 100 and hist_aqi > 100:
                send_discord_message(url=DISCORD_WEBHOOK_URL, message=below_threshold(aqi=aqi))
            else:
                print(f"AQI is {aqi}, previous AQI was {hist_aqi}, no need to send message")

            hist_aqi = aqi
            error_max = 10


        # wait 1 minute before next poll
        time.sleep(60)


if __name__ == "__main__":
    main()