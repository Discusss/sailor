from discord import Color, Embed, InputTextStyle, Interaction
from discord.ui import InputText, Modal

from src.constants import MALICIOUS_CATEGORIES


class ConfirmationModal(Modal):
    def __init__(
            self,
            link: str,
            category: str,
            priority: int,
            note: str,
            original_interaction: Interaction,
            *args,
            **kwargs
    ) -> None:
        super().__init__(*args, **kwargs)

        if note == "Sin nota.":
            note = ""

        self.add_item(InputText(label="Categoría", style=InputTextStyle.short, value=category))
        self.add_item(InputText(label="Prioridad", style=InputTextStyle.short, value=str(priority)))
        self.add_item(InputText(label="Nota del Usuario", style=InputTextStyle.long, value=note, required=False))
        self.add_item(InputText(label="Nota del Revisor", style=InputTextStyle.long, value="", required=False))

        self._link = link
        self._original_category = category
        self._original_priority = priority
        self._original_interaction = original_interaction

    async def callback(self, interaction: Interaction):
        category = self.children[0].value
        priority = int(self.children[1].value)
        user_note = self.children[2].value
        reviewer_note = self.children[3].value

        if category not in MALICIOUS_CATEGORIES:
            category = self._original_category

        if 0 > priority > 10:
            priority = self._original_priority

        # TODO: Make an API call to approve the link and send any modifications.
        print(self._link, category, priority, user_note, reviewer_note, sep="\n")


        embed = Embed(
            color=Color.green(),
            title="Gracias por la valoración",
            description="Enlace aprobado."
        )
        await interaction.response.send_message(embed=embed, ephemeral=True)
        await self._original_interaction.delete_original_response()
