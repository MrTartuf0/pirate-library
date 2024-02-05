FROM rust:1.73.0 as rustedhttpd_builder
RUN apt -y install git
RUN apt -y install openssl libssl-dev
RUN git clone --depth 1 https://github.com/rickytrevor/rustedhttpd.git
WORKDIR /rustedhttpd
RUN cargo build --release
RUN mv /rustedhttpd/target/release/rustedhttpd /rustedhttpd_bin

FROM node:20.11 as web_builder
COPY . /app
WORKDIR /app
RUN npm install
RUN npm run generate

FROM debian:12.0-slim
RUN apt-get update && apt-get install -y ca-certificates libssl-dev
WORKDIR /app
COPY --from=rustedhttpd_builder /rustedhttpd_bin /usr/bin
RUN chmod +x /usr/bin/rustedhttpd_bin
RUN mkdir web
COPY --from=web_builder /app/.output/public /app/web
COPY --from=rustedhttpd_builder /rustedhttpd/config.yaml /app

CMD /usr/bin/rustedhttpd_bin
