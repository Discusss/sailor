import json
import logging
import os

import constants as constants
import discord
import requests
from discord import Embed
from discord.ext import commands

from views.confirmation_modal import ConfirmationModal

logger = logging.getLogger("BOT")


class Events(commands.Cog):
    def __init__(self, bot: discord.Bot):
        self._bot = bot

    @commands.Cog.listener()
    async def on_ready(self):
        await self._bot.change_presence(
            activity=discord.Game(name="Reportando Phishing")
        )

        logger.info("Gateway connected!")

    @commands.Cog.listener()
    async def on_interaction(self, interaction: discord.Interaction):
        if interaction.type is discord.InteractionType.component:
            if interaction.custom_id not in constants.VALID_CUSTOM_IDS:
                await interaction.response.send_message(
                    "¡Ha habido un error!", ephemeral=True
                )
                return

            if len(interaction.message.embeds) == 0:
                await interaction.response.send_message(
                    "Parece que alguien ha borrado el embed. Por favor, pide que lo manden de nuevo."
                )
                return

            domain_id = int(interaction.message.embeds[0].footer.text)

            if interaction.custom_id == "rejected-link":
                response = requests.delete(
                    url=os.getenv("BASE_API_URL") + "/domain",
                    params={"id": domain_id},

                    headers={'Content-Type': 'application/json', "Authorization": os.getenv("API_AUTH_KEY")}
                )

                if response.status_code == 200:
                    embed = Embed(
                        color=discord.Color.red(),
                        title="Gracias por la valoración",
                        description="Enlace rechazado.",
                    )
                elif response.status_code == 401:
                    embed = Embed(
                        color=discord.Color.yellow(),
                        title="ERROR",
                        description="La clave de autorización no es válida.",
                    )
                elif response.status_code == 403:
                    embed = Embed(
                        color=discord.Color.yellow(),
                        title="ERROR",
                        description="El bot está en la blacklist.",
                    )
                elif response.status_code == 404:
                    embed = Embed(
                        color=discord.Color.yellow(),
                        title="ERROR",
                        description="Enlace no encontrado.",
                    )
                else:
                    embed = Embed(
                        color=discord.Color.yellow(),
                        title="ERROR",
                        description="Ha ocurrido un error desconocido.",
                    )

                await interaction.response.send_message(embed=embed, ephemeral=True)
                await interaction.delete_original_response()
                return
            elif interaction.custom_id == "approved-link":
                # Get information of the report based on the embed.
                link = interaction.message.embeds[0].description
                category = interaction.data.get("values", ["Other"])[0]
                priority = interaction.message.embeds[0].fields[1].value
                reason = interaction.message.embeds[0].fields[-2].value
                user_note = interaction.message.embeds[0].fields[-1].value

                # Allows reviewers to modify any values of the report if needed.
                await interaction.response.send_modal(
                    ConfirmationModal(
                        domain_id,
                        link,
                        category,
                        priority,
                        reason,
                        user_note,
                        interaction,
                        title="Review Values",
                    )
                )


def setup(bot):
    bot.add_cog(Events(bot))
