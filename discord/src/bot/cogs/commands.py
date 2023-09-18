import json
import os
import re

import requests
from discord import ApplicationContext, Color, Embed, Option, slash_command
from discord.ext import commands

from src.constants import MALICIOUS_CATEGORIES
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
            link: Option(str, name="enlace", required=True, description="El enlace a reportar."),
            reason: Option(str, name="razón", required=True, description="Razón por la que estás reportando el enlace."),
            category: Option(
                str,
                name="categoría",
                choices=MALICIOUS_CATEGORIES,
                required=False,
                description="La categoría que mejor se adapta al enlace a reportar."
            ),
            priority: Option(
                int,
                name="prioridad",
                min_value=0,
                max_value=10,
                required=False,
                description="El nivel de peligrosidad que crees que representa el enlace."
            ),
            notes: Option(str, name="nota", required=False, description="Cualquier otro detalle que quieras proporcionar."),
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
            link: Option(str, name="enlace", required=True, description="El enlace a comprobar.")
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
                color=Color.greyple(),
                description="Enlace encontrado en la base de datos, ¡ten cuidado!"
            )
            embed\
                .add_field(name="Categoría", value=MALICIOUS_CATEGORIES[int(body.get("category", 7))], inline=True)\
                .add_field(name="Prioridad", value=body.get("priority", "Sin prioridad."), inline=True)\
                .add_field(name="Notas", value=body.get("public_notes", "Sin notas."))
        else:
            embed = Embed(
                title="No encontrado",
                color=Color.red(),
                description="No se ha encontrado el enlace en la base de datos.",
            )

        await ctx.respond(embed=embed)


def setup(bot):
    bot.add_cog(Commands(bot))
