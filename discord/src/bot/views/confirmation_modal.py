import json
import logging
import os

import requests
from discord import Color, Embed, InputTextStyle, Interaction
from discord.ui import InputText, Modal

from src.constants import MALICIOUS_CATEGORIES


class ConfirmationModal(Modal):
    def __init__(
        self,
        domain_id: int,
        category: str,
        priority: int,
        reason: str,
        note: str,
        original_interaction: Interaction,
        *args,
        **kwargs
    ) -> None:
        super().__init__(*args, **kwargs)

        if note == "Sin nota.":
            note = ""

        if priority == "Sin prioridad.":
            priority = ""

        self.add_item(
            InputText(
                label="Prioridad (0-10)",
                style=InputTextStyle.short,
                value=str(priority),
                required=True
            )
        )
        self.add_item(
            InputText(
                label="Nota del Usuario",
                style=InputTextStyle.long,
                value=note,
                required=False,
            )
        )

        self._id = domain_id
        self._reason = reason
        self._category = category
        self._original_priority = priority
        self._original_interaction = original_interaction
        self._logger = logging.getLogger("REVIEWS")

    async def callback(self, interaction: Interaction):
        priority = self.children[0].value
        user_note = self.children[1].value

        if not priority.isnumeric():
            await interaction.response.send_message(
                "Proporciona una prioridad válida.", ephemeral=True
            )
            return
        else:
            priority = int(priority)

        if 0 > priority > 10:
            await interaction.response.send_message(
                "Proporciona una prioridad válida.", ephemeral=True
            )
            return

        response = requests.patch(
            url=os.getenv("API_BASE_URL") + "/api/domain",
            params={"id": self._id},
            headers={'Content-Type': 'application/json', "Authorization": os.getenv("API_AUTH_KEY")},
            data=json.dumps(
                {
                    "category": MALICIOUS_CATEGORIES[self._category],
                    "priority": priority,
                    "public_notes": user_note,
                    "approved_by": interaction.user.name,
                }
            ),
        )

        if response.status_code == 200:
            embed = Embed(
                color=Color.green(),
                title="Gracias por la valoración",
                description="Enlace aprobado.",
            )
            self._logger.info(f"{self._id} has been approved by {interaction.user.name}")
        elif response.status_code == 400:
            embed = Embed(
                color=Color.yellow(),
                title="ERROR",
                description="La petición no es válida (400).",
            )
        elif response.status_code == 401:
            embed = Embed(
                color=Color.yellow(),
                title="ERROR",
                description="La clave de autorización no es válida.",
            )
        elif response.status_code == 403:
            embed = Embed(
                color=Color.yellow(),
                title="ERROR",
                description="El discord está en la blacklist.",
            )
        elif response.status_code == 404:
            embed = Embed(
                color=Color.yellow(), title="ERROR", description="Enlace no encontrado."
            )
        else:
            embed = Embed(
                color=Color.yellow(),
                title="ERROR",
                description="Ha ocurrido un error desconocido.",
            )

        await interaction.response.send_message(embed=embed, ephemeral=True)
        embeds = self._original_interaction.message.embeds
        if len(embeds) > 0:
            embed = embeds[0].to_dict()
            embed["title"] = "Enlace aprobado"
            embed["color"] = int(Color.green())
            embed["fields"][0]["value"] = self._category
            embed["fields"][1]["value"] = str(priority)
            embed["fields"][-1]["value"] = user_note

            await interaction.message.edit(
                content=interaction.message.content,
                embeds=[Embed.from_dict(embed)],
                view=None
            )
