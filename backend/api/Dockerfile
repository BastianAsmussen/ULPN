FROM rust:1.77.2-slim-bookworm as builder

WORKDIR /usr/src/api
COPY . .

RUN apt-get update \
    && apt-get install -y --no-install-recommends libpq-dev \
    && rm -rf /var/lib/apt/lists/*

RUN cargo install --path .

FROM debian:bookworm-slim

RUN apt-get update \
    && apt-get install -y --no-install-recommends libpq-dev \
    && rm -rf /var/lib/apt/lists/*

RUN groupadd -r runner && useradd -g runner api
USER api

COPY --from=builder /usr/local/cargo/bin/api /usr/local/bin/api
COPY --from=builder /usr/src/api/.env .

EXPOSE 3000
CMD ["api"]
