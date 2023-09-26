from dotenv import load_dotenv
from flask import Flask

from src.server.routes.webhook import webhook

load_dotenv()

app = Flask(__name__)

app.add_url_rule(
    "/webhook", "webhook", webhook, methods=["POST"]
)
