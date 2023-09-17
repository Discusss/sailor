import json
import os

import requests
from discord import Color, Embed, InputTextStyle, Interaction
from discord.ui import InputText, Modal

from constants import MALICIOUS_CATEGORIES


class ConfirmationModal(Modal):
    def __init__(
        self,
        domain_id: int,
        link: str,
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

        # if category == "Sin categoría.":
        #     category = ""

        if priority == "Sin prioridad.":
            priority = ""

        # self.add_item(
        #     InputText(
        #         label="Categoría",
        #         style=InputTextStyle.short,
        #         value=category,
        #         required=True
        #     )
        # )
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
        # self.add_item(
        #     InputText(
        #         label="Nota del Revisor",
        #         style=InputTextStyle.long,
        #         value="",
        #         required=False,
        #     )
        # )

        self._id = domain_id
        self._reason = reason
        self._category = category
        self._original_priority = priority
        self._original_interaction = original_interaction

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
            url=os.getenv("API_BASE_URL") + "/domain",
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
                description="El bot está en la blacklist.",
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
        await self._original_interaction.delete_original_response()
