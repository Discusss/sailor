import logging

import constants as constants
import discord
from discord import Embed
from discord.ext import commands

from views.confirmation_modal import ConfirmationModal

logger = logging.getLogger('BOT')


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

            if interaction.custom_id == "rejected-link":
                embed = Embed(
                    color=discord.Color.red(),
                    title="Gracias por la valoración",
                    description="Enlace rechazado."
                )

                # TODO: Make API call to invalidate the report
                await interaction.response.send_message(embed=embed, ephemeral=True)
                await interaction.delete_original_response()
                return

            # Get information of the report based on the embed.
            if len(interaction.message.embeds) == 0:
                await interaction.response.send_message(
                    "Parece que alguien ha borrado el embed. Por favor, pide que lo manden de nuevo."
                )
                return

            link = interaction.message.embeds[0].description
            category = interaction.message.embeds[0].fields[0].value
            priority = interaction.message.embeds[0].fields[1].value
            user_note = interaction.message.embeds[0].fields[-1].value

            # Allows reviewers to modify any values of the report if needed.
            await interaction.response.send_modal(
                ConfirmationModal(link, category, int(priority), user_note, interaction, title="Review Values")
            )


def setup(bot):
    bot.add_cog(Events(bot))
