#####################################################################################################
### Builder image
#####################################################################################################
ARG BASE_IMAGE=ekidd/rust-musl-builder:latest
# Our first FROM statement declares the build environment.
FROM ${BASE_IMAGE} AS builder

# Add our source code.
ADD --chown=rust:rust . ./

# Build our application.
RUN cargo build --release

#####################################################################################################
### Final image
#####################################################################################################
FROM alpine:edge

# Import from builder.
COPY --from=builder /etc/passwd /etc/passwd
COPY --from=builder /etc/group /etc/group

RUN apk update && apk add --no-cache \
      bash \
      chromium \
      nss \
      gcc \
      freetype \
      harfbuzz \
      ca-certificates \
      ttf-freefont \
      curl \
      nodejs \
      npm

# Copy our build
WORKDIR /usr/local/bin/
COPY --from=builder /home/rust/src/target/x86_64-unknown-linux-musl/release/onepunchman_parcer_bot .

#ENV CHROMIUM_PATH /usr/bin/chromium-browser
# Node.js installation
RUN npm install npm@latest -g && \
    npm install n -g && \
    n 14.17.0

COPY --from=builder /home/rust/src/js /usr/local/js
WORKDIR /usr/local/js
RUN npm install && npm i -g .

COPY ./docker/cert/rootCA.cert /usr/local/share/ca-certificates/rootCA.crt
RUN update-ca-certificates

WORKDIR /usr/local/bin/
CMD ["onepunchman_parcer_bot"]