import datetime
import json
import os
import re

import src.helpers.url_analyzer as url_analyzer
import requests
from flask import jsonify, request
import hashlib

from src.constants import MALICIOUS_CATEGORIES


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


def webhook():
    raw_data = request.data.decode('utf-8')
    data = request.json
    signature = request.headers.get("X-LACABRA-Signature")

    if not signature:
        return jsonify({"message": "Provide a valid signature."}), 401

    md5_hash = hashlib.md5()
    md5_hash.update((str(raw_data) + os.getenv("WEBHOOK_HASH_KEY")).encode("utf-8"))
    if md5_hash.hexdigest() != signature:
        return jsonify({"message": "Provide a valid signature."}), 401

    domain_id: int = data.get("id")
    link: str = data.get("domain")
    category: str = data.get("category")
    priority: int = data.get("priority")
    reason: str = data.get("submitted_reason")
    note: str = data.get("public_notes")

    if link is None:
        return jsonify({"message": "URL de phishing no proporcionada."}), 400

    url = re.search(r"(?:(?:https?|ftp)://)?[\w/\-?=%.]+\.[\w/\-&?=%.]+", link)

    if url is None:
        return jsonify({"message": "URL de phishing no válida."}), 400

    final_url = url[0]

    ssl_cert = url_analyzer.check_ssl_certificate(final_url)
    registrar = url_analyzer.get_domain_registration_info(final_url)

    select_menu_options = []
    for malicious_category in MALICIOUS_CATEGORIES:
        select_menu_options.append({
            "label": malicious_category,
            "value": malicious_category
        })

    requests.post(
        url=f"https://discord.com/api/v10/channels/{os.getenv('REVIEWING_CHANNEL_ID')}/messages",
        headers={
            "Authorization": f"Bot {os.getenv('BOT_TOKEN')}",
            "Content-Type": "application/json",
        },
        data=json.dumps(
            {
                "embeds": [
                    {
                        "title": "¡Nuevo Enlace a Revisar!",
                        "description": f"{final_url}",
                        "color": 16761095,
                        "fields": [
                            {
                                "name": "Categoría",
                                "value": category
                                if category is not None
                                else "Sin categoría.",
                                "inline": True,
                            },
                            {
                                "name": "Priority",
                                "value": priority
                                if priority is not None
                                else "Sin prioridad.",
                                "inline": True,
                            },
                            {
                                "name": "Certificado SSL",
                                "value": f"Válido ({ssl_cert[1]})"
                                if ssl_cert[0]
                                else "Inválido",
                            },
                            {
                                "name": "Registrar",
                                "value": registrar["registrar"]
                                if registrar["is_registered"]
                                else "No encontrado",
                            },
                            {
                                "name": "Creación",
                                "value": _get_last_element_or_string(
                                    registrar["creation_date"]
                                ).strftime("%a %d %b %Y %Z")
                                if registrar["creation_date"] is not None
                                else "No encontrado",
                                "inline": True,
                            },
                            {
                                "name": "Última Actualización",
                                "value": _get_last_element_or_string(
                                    registrar["updated_date"]
                                ).strftime("%a %d %b %Y %Z")
                                if registrar["updated_date"] is not None
                                else "No encontrado",
                                "inline": True,
                            },
                            {
                                "name": "Caducidad",
                                "value": _get_last_element_or_string(
                                    registrar["expiration_date"]
                                ).strftime("%a %d %b %Y %Z")
                                if registrar["expiration_date"] is not None
                                else "No encontrado",
                                "inline": True,
                            },
                            {"name": "Razón", "value": reason},
                            {
                                "name": "Notas del Usuario",
                                "value": note if note is not None else "Sin nota.",
                            },
                        ],
                        "footer": {"text": domain_id},
                    }
                ],
                "components": [
                    {
                        "type": 1,
                        "components": [
                            {
                                "type": 3,
                                "label": "Aprobar",
                                "placeholder": "Seleccionar categoría",
                                "custom_id": "approved-link",
                                "options": select_menu_options
                            }

                        ]
                    },
                    {
                        "type": 1,
                        "components": [{
                            "type": 2,
                            "label": "Rechazar",
                            "style": 4,  # Danger
                            "custom_id": "rejected-link",
                        }]
                    }
                ],
            }
        ),
    )

    return jsonify({"message": "Petición recibida."}), 200