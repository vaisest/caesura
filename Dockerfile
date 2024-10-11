FROM rust:alpine AS builder
RUN apk add libc-dev curl
WORKDIR /temp
RUN curl --location \
    "https://github.com/casey/intermodal/releases/download/v0.1.14/imdl-v0.1.14-x86_64-unknown-linux-musl.tar.gz" \
    --output "imdl.tar.gz" \
    && tar \
    --extract \
    --gzip \
    --directory "/bin" \
    --file "imdl.tar.gz" \
    "imdl"
WORKDIR /src
COPY . .
RUN cargo build --release



FROM alpine:latest
RUN apk add --no-cache flac lame sox imagemagick imagemagick-jpeg
COPY --from=builder /bin/imdl /bin/imdl
COPY --from=builder /src/target/release/caesura /bin/caesura
WORKDIR /
ENTRYPOINT ["caesura"]
