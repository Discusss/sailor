import logging

import src.constants as constants
import discord
from discord import Embed, InputTextStyle, Interaction
from discord.ext import commands
from discord.ui import Modal, InputText

logger = logging.getLogger('BOT')


class MyModal(Modal):
    def __init__(self, link: str, category: str, priority: int, note: str, *args, **kwargs) -> None:
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

    async def callback(self, interaction: Interaction):
        category = self.children[0].value
        priority = int(self.children[1].value)
        user_note = self.children[2].value
        reviewer_note = self.children[3].value

        if category not in constants.MALICIOUS_CATEGORIES:
            category = self._original_category

        if 0 > priority > 10:
            priority = self._original_priority

        # API CALL
        print(self._link, category, priority, user_note, reviewer_note, sep="\n")
        embed = Embed(
            color=discord.Color.green(),
            title="Gracias por la valoración",
            description="Enlace aprobado."
        )
        await interaction.response.send_message(embed=embed, ephemeral=True)
        # TODO: It does not delete it
        await interaction.delete_original_response()


class Events(commands.Cog):
    def __init__(self, bot: discord.Bot):
        self._bot = bot

    @commands.Cog.listener()
    async def on_ready(self):
        await self._bot.change_presence(
            activity=discord.Game(
                name="Reportando Phishing"
            )
        )

        logger.info("Gateway connected!")

    @commands.Cog.listener()
    async def on_interaction(self, interaction: discord.Interaction):
        if interaction.type is discord.InteractionType.component:
            if interaction.custom_id not in constants.VALID_CUSTOM_IDS:
                await interaction.response.send_message("¡Ha habido un error!", ephemeral=True)
                return

            embed = None

            # TODO: Api calls
            if interaction.custom_id == "rejected-link":
                embed = Embed(
                    color=discord.Color.red(),
                    title="Gracias por la valoración",
                    description="Enlace rechazado."
                )

                # Api call when rejected
                await interaction.response.send_message(embed=embed, ephemeral=True)
                return


            # MODAL
            link = interaction.message.embeds[0].description
            category = interaction.message.embeds[0].fields[0].value
            priority = interaction.message.embeds[0].fields[1].value
            user_note = interaction.message.embeds[0].fields[-1].value

            await interaction.response.send_modal(MyModal(link, category, int(priority), user_note, title="Review Values"))




def setup(bot):
    bot.add_cog(Events(bot))
