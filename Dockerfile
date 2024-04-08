FROM zhdanovdev/rust-wasm-builder:latest as builder

RUN mkdir /app
COPY . /app
WORKDIR /app/web

RUN trunk build --release --public-url /
RUN wasm-opt -Oz -o dist/*.wasm dist/*.wasm

WORKDIR /app
RUN cargo build --bin server --release

#Base Container
FROM debian:bullseye-slim

RUN apt-get update && apt install -y openssl

ARG UID=10001
RUN adduser \
    --disabled-password \
    --gecos "" \
    --home "/nonexistent" \
    --shell "/sbin/nologin" \
    --no-create-home \
    --uid "${UID}" \
    appuser
USER appuser

COPY --from=builder /app/web/dist/ /app/dist/
COPY --from=builder /app/target/release/server /bin/

ENV PORT 8080

ENTRYPOINT /bin/server --port $PORT --static-dir /app/dist