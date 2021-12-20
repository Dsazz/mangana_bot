FROM rust:latest as builder
WORKDIR /usr/src/onepunchman_parcer_bot
COPY . .
ENV CARGO_HTTP_CHECK_REVOKE=false
RUN cargo install --path .

FROM debian:latest
WORKDIR /root
RUN apt-get update && apt-get install -y ca-certificates libpq-dev && rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/local/cargo/bin/onepunchman_parcer_bot /usr/local/bin/onepunchman_parcer_bot
CMD ["onepunchman_parcer_bot"]