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

        if category == "Sin categoría.":
            category = ""

        if priority == "Sin prioridad.":
            priority = ""

        self.add_item(InputText(label="Categoría", style=InputTextStyle.short, value=category))
        self.add_item(InputText(label="Prioridad (0-10)", style=InputTextStyle.short, value=str(priority)))
        self.add_item(InputText(label="Nota del Usuario", style=InputTextStyle.long, value=note, required=False))
        self.add_item(InputText(label="Nota del Revisor", style=InputTextStyle.long, value="", required=False))

        self._id = domain_id
        self._reason = reason
        self._original_category = category
        self._original_priority = priority
        self._original_interaction = original_interaction

    async def callback(self, interaction: Interaction):
        category = self.children[0].value
        priority = self.children[1].value
        user_note = self.children[2].value
        reviewer_note = self.children[3].value

        if category not in MALICIOUS_CATEGORIES:
            category = self._original_category

        if priority is not int:
            await interaction.response.send_message("Proporciona una prioridad válida.", ephemeral=True)
            return
        else:
            priority = int(priority)

        if 0 > priority > 10:
            priority = self._original_priority

        requests.patch(
            url=os.getenv("API_BASE_URL") + "/domains",
            params=json.dumps({
                "id": self._id
            }),
            data=json.dumps({
                "category": MALICIOUS_CATEGORIES[category] if category is not None else None,
                "priority": priority,
                "public_notes": user_note,
                "notes": reviewer_note,
                "approved_by": interaction.user.name
            })
        )

        embed = Embed(
            color=Color.green(),
            title="Gracias por la valoración",
            description="Enlace aprobado."
        )
        await interaction.response.send_message(embed=embed, ephemeral=True)
        await self._original_interaction.delete_original_response()
