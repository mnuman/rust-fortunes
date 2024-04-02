FROM rust:1 as build-env
LABEL Author="Milco Numan" REPO="https://github.com/mnuman/sudocku" Language="Rust"
WORKDIR /app
COPY . /app
RUN cargo build --release

FROM gcr.io/distroless/cc-debian12
COPY --from=build-env /app/target/release/rust_fortune /
COPY --from=build-env /app/fortunes.txt /
EXPOSE 8080
CMD ["./rust_fortune"]