FROM rust:alpine3.18 as builder
COPY . /app
WORKDIR /app
RUN apk add --no-cache musl-dev
RUN cargo b -r

# run
FROM scratch
COPY --from=builder /etc/passwd /etc/passwd
COPY --from=builder /app/target/release/lean /lean
CMD [ "./lean" ]