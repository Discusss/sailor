import logging
import os

from discord import Bot
from dotenv import load_dotenv

logger = logging.getLogger("BOT")
logger.setLevel(logging.INFO)
handler = logging.StreamHandler()
handler.setFormatter(
    logging.Formatter("[%(asctime)s] [%(levelname)s] [%(name)s] %(message)s")
)
logger.addHandler(handler)

load_dotenv()

client = Bot()
cogs = ["cogs.commands", "cogs.events"]
client.load_extensions(*cogs)

client.run(os.getenv("BOT_TOKEN"))
