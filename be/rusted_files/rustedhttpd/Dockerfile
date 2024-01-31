FROM rust:1.73 as builder
COPY . /src
WORKDIR /src
RUN apt -y update
RUN apt -y install openssl libssl-dev
RUN cargo build --release
CMD bash

FROM debian as runtime
RUN apt -y update
RUN apt -y install openssl libssl-dev
COPY --from=builder /src/target/release/rustedhttpd /usr/local/bin/rustedhttpd
EXPOSE 80

WORKDIR /var/www/html
CMD ["rustedhttpd"]