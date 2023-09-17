import json
import os
import re

import requests
from discord import ApplicationContext, Option, Embed, Color, slash_command
from discord.ext import commands

from constants import MALICIOUS_CATEGORIES
from src.helpers.url_analyzer import get_netloc


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
            link: Option(str, name="enlace", required=True),
            reason: Option(str, name="razón", required=True),
            category: Option(
                str, name="categoría", choices=MALICIOUS_CATEGORIES, required=False
            ),
            priority: Option(
                int, name="prioridad", min_value=0, max_value=10, required=False
            ),
            notes: Option(str, name="nota", required=False),
    ):
        urls = re.search(r"(?:(?:https?|ftp)://)?[\w/\-?=%.]+\.[\w/\-&?=%.]+", link)

        if urls is None:
            embed = Embed(
                title="Reporte de Enlaces",
                color=Color.red(),
                description="No se ha detectado ningún enlace.",
            )
            await ctx.respond(embed=embed)
            return

        netloc = get_netloc(urls[0])

        response = requests.post(
            url=os.getenv("API_BASE_URL") + "/domain",
            data=json.dumps(
                {
                    "domain": netloc,
                    "category": MALICIOUS_CATEGORIES[category]
                    if category is not None
                    else None,
                    "priority": priority,
                    "submitted_by": ctx.user.name,
                    "reason": reason,
                    "notes": notes,
                }
            ),
        )

        if response.status_code == 200:
            embed = Embed(
                title="Reporte de Enlaces",
                color=Color.green(),
                description="Enlace mandado a revisar, gracias por la ayuda.",
            )
        elif response.status_code == 409:
            embed = Embed(
                title="Reporte de Enlaces",
                color=Color.red(),
                description="Enlace ya registrado en la base de datos.",
            )
        else:
            embed = Embed(
                title="Reporte de Enlaces",
                color=Color.red(),
                description="Ha ocurrido un error inesperado.",
            )

        await ctx.respond(embed=embed)

    @slash_command(
        description="Obtén información de un enlace.",
        guild_only=True
    )
    async def comprobar(
            self,
            ctx: ApplicationContext,
            link: Option(str, name="enlace", required=True)
    ):
        urls = re.search(r"(?:(?:https?|ftp)://)?[\w/\-?=%.]+\.[\w/\-&?=%.]+", link)

        if urls is None:
            embed = Embed(
                title="Comprobación de enlaces",
                color=Color.red(),
                description="No se ha detectado ningún enlace.",
            )
            await ctx.respond(embed=embed)
            return

        netloc = get_netloc(urls[0])

        response = requests.get(
            url=os.getenv("API_BASE_URL") + "/domain",
            params={"domain": netloc}
        )

        body = dict(response.json())["data"]
        if response.status_code == 200:
            embed = Embed(
                title=f"Información de {netloc}",
                color=Color.greyple()
            )
            embed\
                .add_field(name="Categoría", value=body.get("category"), inline=True)\
                .add_field(name="Prioridad", value=body.get("priority"), inline=True)\
                .add_field(name="Notas", value=body.get("notes"))
        else:
            embed = Embed(
                title="No encontrado",
                color=Color.red(),
                description="No se ha encontrado el enlace en la base de datos.",
            )

        await ctx.respond(embed=embed)


def setup(bot):
    bot.add_cog(Commands(bot))
