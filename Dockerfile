FROM rust as dependencies
WORKDIR /build
RUN mkdir src && echo 'fn main() {}' > src/main.rs
COPY Cargo.toml Cargo.lock ./
RUN cargo build --release

FROM rust as build
WORKDIR /build
COPY --from=dependencies /build/target target
COPY . .
RUN cargo build --release

FROM rust as app
WORKDIR /app
COPY --from=build /build/target/release/autostocklist .
EXPOSE 8080
ENTRYPOINT ./autostocklist

