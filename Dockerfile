# 1. Build rust
FROM rust:1.75 as build

# Create a new empty shell project
RUN USER=root cargo new --bin httpserve
WORKDIR /httpserve

# Copy manifests
COPY Cargo.lock ./Cargo.lock
COPY Cargo.toml ./Cargo.toml

# Build dependencies to cache them
RUN cargo build --release
RUN rm src/*.rs

# Update build directory's source code
COPY src ./src

# Build for release
RUN rm -f ./target/release/deps/httpserve*
RUN cargo install --path .

# 2. Package in a small production image
FROM debian:bookworm-slim
WORKDIR /web

# copy the build artifact from the build stage
COPY --from=build /httpserve/target/release/httpserve ./httpserve

EXPOSE 80

# Run the binary
CMD ["./httpserve"]