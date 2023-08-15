import re

import discord
from discord.ext import commands


class Commands(commands.Cog):
    def __init__(self, bot):
        self.bot = bot

    @discord.slash_command(
        description="Reporta un enlace sospechoso de ser phishing y lo revisaremos.",
        guild_only=True
    )
    async def reportar(
            self,
            ctx: discord.ApplicationContext,
            link: discord.Option(name="enlace", type=str, required=True)
    ):
        urls = re.search(r"(?:(?:https?|ftp)://)?[\w/\-?=%.]+\.[\w/\-&?=%.]+", link)

        if urls is None:
            embed = discord.Embed(
                title="Reporte de Enlaces",
                color=discord.Color.red(),
                description="No se ha detectado ningún enlace."
            )
        else:
            embed = discord.Embed(
                title="Reporte de Enlaces",
                color=discord.Color.green(),
                description="Enlace mandado a revisar, gracias por la ayuda."
            )

        await ctx.respond(embed=embed)


def setup(bot):
    bot.add_cog(Commands(bot))
