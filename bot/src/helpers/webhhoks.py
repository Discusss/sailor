import datetime
import json
import os
import re
from threading import Thread
from urllib.parse import urlparse

import src.helpers.url_analyzer as url_analyzer
import requests
from discord import Bot, Color
from flask import Flask, jsonify, request


def _get_last_element_or_string(data):
    if isinstance(data, list):
        if data:
            return data[-1]
        else:
            return "List is empty"
    elif isinstance(data, datetime.datetime):
        return data
    else:
        return "Unsupported data type"


class WebhookReceiver:
    def __init__(self, bot: Bot, review_channel_id: str, route="/webhook", port=5001):
        self._bot = bot
        self._route = route
        self._port = port
        self._app = Flask(__name__)
        self._app.add_url_rule(self._route, "webhook", self.receive_webhook, methods=["POST"])
        self._review_channel_id = review_channel_id

    def start(self):
        Thread(target=self._run_app).start()

    def _run_app(self):
        self._app.run(debug=False, port=self._port)

    def receive_webhook(self):
        data = request.json

        link: str = data["link"]
        category: str = data["category"]
        priority: int = data["priority"]
        note: str = data["note"]

        if link is None:
            return jsonify({"message": "URL de phishing no proporcionada."}), 400

        url = re.search(r"(?:(?:https?|ftp)://)?[\w/\-?=%.]+\.[\w/\-&?=%.]+", link)

        if url is None:
            return jsonify({"message": "URL de phishing no válida."}), 400

        final_url = url[0]
        parsed_url = urlparse(final_url)

        ssl_cert = url_analyzer.check_ssl_certificate(parsed_url.netloc)
        registrar = url_analyzer.get_domain_registration_info(parsed_url.netloc)

        requests.post(
            url=f"https://discord.com/api/v10/channels/{self._review_channel_id}/messages",
            headers={
                "Authorization": f"Bot {os.getenv('BOT_TOKEN')}",
                "Content-Type": "application/json"
            },
            data=json.dumps({
                "embeds": [
                    {
                        "title": "¡Nuevo Enlace a Revisar!",
                        "description": f"{parsed_url.netloc}",
                        "color": int(Color.gold()),
                        "fields": [
                            {
                                "name": "Categoría",
                                "value": category,
                                "inline": True
                            },
                            {
                                "name": "Priority",
                                "value": priority,
                                "inline": True
                            },
                            {
                                "name": "Certificado SSL",
                                "value": f"Válido ({ssl_cert[1]})" if ssl_cert[0] else "Inválido"
                            },
                            {
                                "name": "Registrar",
                                "value": registrar["registrar"] if registrar["is_registered"] else "No encontrado"
                            },
                            {
                                "name": "Creación",
                                "value": _get_last_element_or_string(registrar["creation_date"])
                                .strftime("%a %d %b %Y %Z")
                                if registrar["creation_date"] is not None
                                else "No encontrado",
                                "inline": True
                            },
                            {
                                "name": "Última Actualización",
                                "value": _get_last_element_or_string(registrar["updated_date"])
                                .strftime("%a %d %b %Y %Z")
                                if registrar["updated_date"] is not None
                                else "No encontrado",
                                "inline": True
                            },
                            {
                                "name": "Caducidad",
                                "value": _get_last_element_or_string(registrar["expiration_date"])
                                .strftime("%a %d %b %Y %Z")
                                if registrar["expiration_date"] is not None
                                else "No encontrado",
                                "inline": True
                            },
                            {
                                "name": "Notas del Usuario",
                                "value": note if note != "" else "Sin nota."
                            }
                        ]
                    }
                ],
                "components": [
                    {
                        "type": 1,
                        "components": [
                            {
                                "type": 2,
                                "label": "Aprobar",
                                "style": 3,  # Success
                                "custom_id": "approved-link"
                            },
                            {
                                "type": 2,
                                "label": "Rechazar",
                                "style": 4,  # Danger
                                "custom_id": "rejected-link"
                            }
                        ]
                    }
                ]
            })
        )

        return jsonify({"message": "Petición recibida."}), 200

