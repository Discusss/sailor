# Use an official Miniconda3 image as the base image
FROM continuumio/miniconda3:latest

# Set environment variables
ENV LANG=C.UTF-8 LC_ALL=C.UTF-8

# Create and set the working directory
WORKDIR .

# Copy the Conda environment specification file (environment.yml) to the container
COPY environment.yml .

# Create a Conda environment based on the specification file
RUN conda env create -f environment.yml

# Activate the Conda environment
RUN echo "source activate $(head -1 environment.yml | cut -d' ' -f2)" > ~/.bashrc
ENV PATH /opt/conda/envs/$(head -1 environment.yml | cut -d' ' -f2)/bin:$PATH

# Copy the rest of the application code to the container
COPY . .

# Start the application
CMD ["python3", "src/main.py"]