FROM python:3.9-slim

WORKDIR /app
COPY requirements.txt .
COPY aqi-alert-bot/main.py .
RUN pip install -r requirements.txt

CMD ["python", "-u", "main.py"]

