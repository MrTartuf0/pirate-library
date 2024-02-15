FROM rust:1.73.0 as rustedhttpd_builder
RUN apt -y install git
RUN apt -y install openssl libssl-dev

COPY ./rustedhttpd /rustedhttpd
WORKDIR /rustedhttpd
RUN cargo build --release
RUN mv /rustedhttpd/target/release/rustedhttpd /rustedhttpd_bin


FROM debian:bookworm-20240211-slim
RUN apt-get update && apt-get install -y ca-certificates libssl-dev
COPY . /app
WORKDIR /app
COPY --from=rustedhttpd_builder /rustedhttpd_bin /usr/bin
RUN chmod +x /usr/bin/rustedhttpd_bin
CMD /usr/bin/rustedhttpd_bin