FROM rust:bookworm

# Install dependencies
RUN apt-get update \
  && apt-get install flac lame sox imagemagick --yes \
  && rm -rf /var/lib/apt/lists/*
RUN cargo install imdl

# Install app
WORKDIR /app
COPY . .
RUN cargo install --path .

# Run
ENTRYPOINT ["rogue_oxide"]
