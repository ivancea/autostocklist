FROM rust as dependencies
WORKDIR /build

# File to allow this to download dependencies
RUN mkdir src && echo 'fn main() {}' > src/main.rs
COPY Cargo.toml Cargo.lock ./
RUN cargo build --release

FROM rust as build
WORKDIR /build
COPY --from=dependencies /build/target target
COPY . .
# Makes cargo detect the file is newer to rebuild it
RUN touch src/main.rs
RUN cargo build --release

FROM debian as app
WORKDIR /app
COPY --from=build /build/target/release/autostocklist .
EXPOSE 8080
ENTRYPOINT ./autostocklist

