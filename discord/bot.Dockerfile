FROM python:3.10-slim

WORKDIR ./bot

ADD . .

RUN pip install -r requirements-bot.txt

CMD ["python3", "src/bot/main.py"]