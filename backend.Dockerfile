FROM rust:1.66-alpine AS builder

WORKDIR /app
RUN apk add postgresql-dev musl-dev
RUN cargo init
COPY backend/Cargo.toml .
RUN cargo build --release
COPY backend .
RUN cargo build --release

FROM alpine
COPY --from=builder /app/target/release/backend /usr/bin/naff-server 
EXPOSE 8080
CMD [ "naff-server" ]
