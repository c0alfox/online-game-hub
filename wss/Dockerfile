FROM rust:latest as build

RUN rustup target add x86_64-unknown-linux-musl && \
    apt update && \
    apt install -y musl-tools musl-dev && \
    update-ca-certificates

RUN adduser \
    --disabled-password \
    --gecos "" \
    --home "/nonexistent" \
    --shell "/sbin/nologin" \
    --no-create-home \
    --uid 10001 \
    "wss"

COPY ./Cargo.lock .
COPY ./Cargo.toml .
COPY ./dummy.rs .

RUN sed -i 's#src/main.rs#dummy.rs#' Cargo.toml
RUN cargo build --target x86_64-unknown-linux-musl --release
RUN sed -i 's#dummy.rs#src/main.rs#' Cargo.toml

COPY . .

RUN cargo build --target x86_64-unknown-linux-musl --release

FROM rust:alpine

COPY --from=build /etc/passwd /etc/passwd
COPY --from=build /etc/group /etc/group

USER wss:wss

COPY --from=build --chown=wss:wss ./target/x86_64-unknown-linux-musl/release/wss /app/wss

EXPOSE 8888

ENTRYPOINT [ "./app/wss" ]