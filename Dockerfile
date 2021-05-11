FROM rust as dependencies
WORKDIR /build
# Fummy file to download dependencies
RUN mkdir src && echo 'fn main() {}' > src/main.rs
COPY Cargo.toml Cargo.lock ./
RUN cargo build --release

FROM rust as build
WORKDIR /build
COPY --from=dependencies /build/target target
COPY . .
# So that cargo detects the file is newer and rebuilds it
RUN touch src/main.rs
RUN cargo build --release

FROM debian as app
WORKDIR /app
COPY --from=build /build/target/release/autostocklist .
EXPOSE 8080
ENTRYPOINT ./autostocklist

