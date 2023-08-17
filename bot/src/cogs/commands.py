import re

from discord import ApplicationContext, Option, Embed, Color, slash_command
from discord.ext import commands

from src.constants import MALICIOUS_CATEGORIES


class Commands(commands.Cog):
    def __init__(self, bot):
        self.bot = bot

    @slash_command(
        description="Reporta un enlace sospechoso de ser phishing y lo revisaremos.",
        guild_only=True
    )
    async def reportar(
            self,
            ctx: ApplicationContext,
            link: Option(name="enlace", type=str, required=True),
            category: Option(str, "categoría", required=True, choices=MALICIOUS_CATEGORIES),
            priority: Option(int, "prioridad",  min_value=0, max_value=10, required=True),
            note: Option(name="nota", type=str, default="")
    ):
        urls = re.search(r"(?:(?:https?|ftp)://)?[\w/\-?=%.]+\.[\w/\-&?=%.]+", link)

        if urls is None:
            embed = Embed(
                title="Reporte de Enlaces",
                color=Color.red(),
                description="No se ha detectado ningún enlace."
            )
        # TODO: Elif checking if already added.
        else:
            embed = Embed(
                title="Reporte de Enlaces",
                color=Color.green(),
                description="Enlace mandado a revisar, gracias por la ayuda."
            )

        # TODO: Api call to submit for reviewing.

        await ctx.respond(embed=embed)


def setup(bot):
    bot.add_cog(Commands(bot))
