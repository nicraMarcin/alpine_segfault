FROM rust:1.54.0 as build
WORKDIR /app
COPY . .
RUN rustc --version \
    && cargo --version \
    && cargo test --workspace --verbose --release \
    && cargo build --release

FROM alpine:3
ENV TZ=Europe/Warsaw
WORKDIR /app
RUN apk --no-cache add libgcc gcompat
COPY --from=build /app/target/release/gcore .
COPY config/gurita.toml /etc/gurita/gurita.toml

CMD ["/app/gcore"]
