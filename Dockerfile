FROM rust:1.87-bullseye as builder

WORKDIR /usr/src/quests-tracker
COPY . .
RUN cargo build --release

FROM debian:bullseye-slim

RUN apt-get update && apt-get install -y --no-install-recommends \
    libssl-dev \
    libpq-dev \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

COPY --from=builder /usr/src/quests-tracker/target/release/quests-tracker /usr/local/bin/quests-tracker

CMD ["quests-tracker"]