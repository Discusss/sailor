# Use an official Miniconda3 image as the base image
FROM python:3.10

WORKDIR .

ADD . .

RUN pip install -r requirements.txt

EXPOSE 5001

CMD ["python3", "src/main.py"]