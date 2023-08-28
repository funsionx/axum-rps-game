FROM rust:latest as builder
WORKDIR /app
COPY . .
RUN cargo build --release

FROM rust:latest
WORKDIR /app
COPY --from=builder /app/target/release/rps-game .
COPY index.html .
EXPOSE 3000
CMD ["./rps-game"]
