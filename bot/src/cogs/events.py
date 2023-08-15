import logging

import discord
from discord.ext import commands

logger = logging.getLogger('BOT')


class Events(commands.Cog):
    def __init__(self, bot):
        self.bot = bot

    @commands.Cog.listener()
    async def on_ready(self):
        await self.bot.change_presence(
            activity=discord.Game(
                name="Reportando Phishing"
            )
        )

        logger.info("Bot en marcha!")


def setup(bot):
    bot.add_cog(Events(bot))
