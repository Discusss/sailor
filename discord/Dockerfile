FROM python:3.10

WORKDIR ./

ADD src/ ./

RUN pip install -r requirements.txt

EXPOSE 5001

CMD ["python3", "src/bot/main.py"]
CMD ["gunicorn", "-b", "0.0.0.0:5001", "'src.server.main:app'"]