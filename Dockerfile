FROM rust:bookworm as builder
WORKDIR /usr/src/converter
COPY . .
RUN cargo install --path .

FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y pandoc ffmpeg build-essential
WORKDIR /usr/src/converter
COPY filetypes.json .
COPY converters converters
COPY --from=builder /usr/local/cargo/bin/converter /usr/local/bin/converter
CMD ["converter"]