from flask import Flask
from server.routes.webhook import webhook

from dotenv import load_dotenv

load_dotenv()

app = Flask(__name__)

app.add_url_rule(
    "/webhook", "webhook", webhook, methods=["POST"]
)
