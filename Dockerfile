FROM rust:alpine
COPY . /app
WORKDIR /app
RUN apk add --no-cache musl-dev 
CMD cargo run