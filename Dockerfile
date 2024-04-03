FROM osomahe/rust-trunk:22.05 as builder

RUN mkdir /web
COPY web /web
WORKDIR /web
RUN trunk build --release --public-url /


#Base Container
FROM rust:latest
#Add the cargo to the PATH
RUN echo "export PATH=$PATH:/usr/local/cargo/bin" >> /root/.bashrc

COPY . ./app
COPY --from=builder /web/dist/ /app/dist/

WORKDIR /app
RUN cargo build --bin server --release

ENV PORT 8080

ENTRYPOINT ["cargo", "run", "--bin", "server", "--release", "--", "--port", "$PORT", "--static-dir", "dist"]