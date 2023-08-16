import logging

import constants
import discord
from discord import Embed
from discord.ext import commands

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

        logger.info("Bot en marcha!")

    @commands.Cog.listener()
    async def on_interaction(self, interaction: discord.Interaction):
        if interaction.type is discord.InteractionType.component:
            if interaction.custom_id not in constants.VALID_CUSTOM_IDS:
                await interaction.response.send_message("¡Ha habido un error!", ephemeral=True)
                return

            embed = None

            # TODO: Api calls
            match interaction.custom_id:
                case "approved-link":
                    embed = Embed(
                        color=discord.Color.green(),
                        title="Gracias por la valoración",
                        description="Enlace aprobado."
                    )
                    # Api call when approved
                case "rejected-link":
                    embed = Embed(
                        color=discord.Color.red(),
                        title="Gracias por la valoración",
                        description="Enlace rechazado."
                    )
                    # Api call when rejected

            await interaction.response.send_message(embed=embed, ephemeral=True)
            await interaction.delete_original_message()


def setup(bot):
    bot.add_cog(Events(bot))
