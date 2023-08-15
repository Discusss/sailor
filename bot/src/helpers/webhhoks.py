from threading import Thread

from discord import Bot
from flask import Flask, jsonify, request


class WebhookReceiver:
    def __init__(self, bot: Bot, route="/webhook", port=5000):
        self.bot = bot
        self.route = route
        self.port = port
        self.app = Flask(__name__)
        self.app.add_url_rule(self.route, "webhook", self.receive_webhook, methods=["POST"])

    def start(self):
        Thread(target=self._run_app).start()

    def _run_app(self):
        self.app.run(debug=False, port=self.port)

    def receive_webhook(self):
        data = request.json

        # Hacer lo que sea cuando sepa qué lleva el data

        return jsonify({"message": "Webhook received"}), 200

