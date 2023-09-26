FROM python:3.10-slim

WORKDIR ./server

ADD . .

RUN pip install -r requirements-server.txt

EXPOSE 5001

CMD ["gunicorn", "-b", "0.0.0.0:5001", "src.server.main:app"]