FROM rust:1.83.0-slim-bullseye AS build

RUN apt-get update && apt-get install -y \
    libssl-dev \
    pkg-config \
    curl \
    build-essential

WORKDIR /build

COPY Cargo.lock Cargo.toml ./
COPY src src
COPY entrypoint.sh entrypoint.sh
COPY migrations migrations
RUN cargo clean && cargo build --locked --release --workspace

FROM debian:bullseye-slim AS final
WORKDIR /app

RUN apt-get update && apt-get install -y \
    build-essential \
    autoconf \
    wget \
    automake \
    libtool

COPY --from=build /build/target/release/invee /app
COPY --from=build /build/target/release/migrations /app
COPY --from=build /build/entrypoint.sh /app

USER root
RUN chmod +x entrypoint.sh
RUN addgroup --system --gid 1001 invee
RUN adduser --system --uid 1001 invee

USER invee
EXPOSE 8080

CMD ["./entrypoint.sh"]
