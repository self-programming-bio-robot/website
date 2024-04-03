#Base Container
FROM rust:latest
#Add the cargo to the PATH
RUN echo "export PATH=$PATH:/usr/local/cargo/bin" >> /root/.bashrc
#Install the rust tools we want
RUN rustup target add wasm32-unknown-unknown
RUN cargo install trunk

COPY . ./app

WORKDIR ./web/app
RUN trunk build --release --public-url /

WORKDIR ..
RUN cargo build --bin server --release

ENV PORT 8080

ENTRYPOINT ["cargo", "run", "--bin", "server", "--release", "--", "--port", "$PORT", "--static-dir", "web/dist"]