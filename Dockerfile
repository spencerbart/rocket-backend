FROM rust:1.62.1

COPY ./ ./

RUN cargo build --release

ENV ROCKET_ADDRESS=0.0.0.0
EXPOSE 8000
CMD ["./target/release/rocket-backend"]