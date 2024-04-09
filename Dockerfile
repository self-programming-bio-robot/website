FROM zhdanovdev/rust-wasm-builder:latest as builder

RUN mkdir /app
COPY . /app
WORKDIR /app/web

RUN trunk build --release --public-url /
RUN wasm-opt -Oz -o dist/*.wasm dist/*.wasm

#Base Container
FROM rust:latest
#Add the cargo to the PATH
RUN echo "export PATH=$PATH:/usr/local/cargo/bin" >> /root/.bashrc

COPY . ./app
COPY --from=builder /app/web/dist/ /app/dist/

WORKDIR /app
RUN cargo build --bin server --release

ENV PORT 8080

ENTRYPOINT cargo run --bin server --release -- --port $PORT --static-dir dist