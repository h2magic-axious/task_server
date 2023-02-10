FROM clux/muslrust as builder

COPY ./configure_server /app
WORKDIR /app

ENV DATABASE_URL=postgres://postgres:postgres@postgresql:5432/configure_server \
	CARGO_HOME=/root/.cargo

RUN echo "[source.crates-io]" >> /root/.cargo/config && \
    echo "replace-with = 'ustc'" >> /root/.cargo/config && \
    echo "[source.ustc]" >> /root/.cargo/config && \
    echo "registry='https://mirrors.ustc.edu.cn/crates.io-index'" >> /root/.cargo/config && \
    cargo build --release && \
    sqlx migrate run

FROM alpine:latest

COPY --from=builder /app/target/x86_64-unknown-linux-musl/release/server /application/server
COPY --from=builder /app/start.sh /application/start.sh

EXPOSE 80

WORKDIR /application

CMD ["./server"]
